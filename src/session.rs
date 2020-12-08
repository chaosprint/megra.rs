use std::collections::{BTreeSet, HashMap};
use std::sync;
use parking_lot::Mutex;

use ruffbox_synth::ruffbox::Ruffbox;
use ruffbox_synth::ruffbox::synth::SynthParameter;

use crate::scheduler::{Scheduler, SchedulerData};
use crate::generator::Generator;
use crate::event_helpers::*;

#[derive(Clone,Copy,PartialEq)]
pub enum OutputMode {
    Stereo,
    // AmbisonicsBinaural,
    // Ambisonics
    FourChannel,
    EightChannel,
    //SixteenChannel,
    //TwentyFourChannel,           
}

pub struct SyncContext {
    pub name: String,
    pub synced: Vec<String>,
    pub generators: Vec<Generator>
}

pub struct Session <const BUFSIZE:usize, const NCHAN:usize> {
    schedulers: HashMap<BTreeSet<String>, (Scheduler<BUFSIZE, NCHAN>, sync::Arc<Mutex<SchedulerData<BUFSIZE, NCHAN>>>)>,
    output_mode: OutputMode,
}

impl <const BUFSIZE:usize, const NCHAN:usize> Session<BUFSIZE, NCHAN> {

    pub fn with_mode(mode: OutputMode) -> Self {
	Session {
	    schedulers: HashMap::new(),
	    output_mode: mode,
	}
    }
    
    pub fn start_generator(&mut self, gen: Box<Generator>, ruffbox: sync::Arc<Mutex<Ruffbox<BUFSIZE, NCHAN>>>) {

	let id_tags = gen.id_tags.clone();
	// start scheduler if it exists ...
	if let Some((_, data)) = self.schedulers.get_mut(&id_tags) {
	    // keep the scheduler running, just replace the data ...
	    let mut sched_data = data.lock();
	    *sched_data = SchedulerData::<BUFSIZE, NCHAN>::from_previous(&sched_data, gen, ruffbox);
	} else {
	    // otherwise, create new sched and data ...
	    let sched_data:sync::Arc<Mutex<SchedulerData<BUFSIZE, NCHAN>>>
		= sync::Arc::new(Mutex::new(SchedulerData::<BUFSIZE, NCHAN>::from_data(gen, ruffbox, self.output_mode)));	    
	    let mut sched = Scheduler::<BUFSIZE, NCHAN>::new();
		    
	    // the evaluation function ...
	    // or better, the inside part of the time recursion
	    let eval_loop = |data: &mut SchedulerData<BUFSIZE, NCHAN>| -> f64 {
		
		let events = data.generator.current_events();
		let mut ruff = data.ruffbox.lock();
		for ev in events.iter() {

		    // no need to allocate a string everytime here, should be changed
		    if ev.name == "silence" {
			continue;
		    }
		    
		    let mut bufnum:usize = 0;
		    if let Some(b) = ev.params.get(&SynthParameter::SampleBufferNumber) {
			bufnum = *b as usize;
		    }
		    
		    // latency 0.05, should be made configurable later ...
		    let inst = ruff.prepare_instance(map_name(&ev.name), data.stream_time + 0.05, bufnum);
		    
		    for (k,v) in ev.params.iter() {
			// special handling for stereo param
			if k == &SynthParameter::ChannelPosition && data.mode == OutputMode::Stereo {			
			    let pos = (*v + 1.0) * 0.5;			
			    ruff.set_instance_parameter(inst, *k, pos);
			} else {
			    ruff.set_instance_parameter(inst, *k, *v);
			}
		    }
		    ruff.trigger(inst);
		}

		(data.generator.current_transition().params[&SynthParameter::Duration] as f64 / 1000.0) as f64
	    };
	    
	    sched.start(eval_loop, sync::Arc::clone(&sched_data));
	    self.schedulers.insert(id_tags, (sched, sched_data));
	}		
    }

    pub fn stop_generator(&mut self, gen_name: &BTreeSet<String>) {
	if let Some((sched, _)) = self.schedulers.get_mut(gen_name) {
	    sched.stop();
	}
    }

    pub fn clear_session(&mut self) {
	for (k,(sched, _)) in self.schedulers.iter_mut() {
	    print!("stop generator \'");
	    for tag in k.iter() {
		print!("{} ", tag);
	    }
	    println!("\'");
	    sched.stop();
	}
	self.schedulers = HashMap::new();
    }
}
