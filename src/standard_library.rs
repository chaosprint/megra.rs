use crate::parser::{eval, FunctionMap};

/**
 * This is where all the "frontend" functions (that is, the DSL functions)
 * are defined and bound to their Rust equivalents ...  
 */
pub fn define_standard_library() -> FunctionMap {
    let mut standard_library = FunctionMap::new();
    // session
    standard_library.fmap.insert("sx".to_string(), eval::session::sync_context::sync_context);

    // constructors
    standard_library.fmap.insert("nuc".to_string(), eval::constructors::nuc::nuc);
    standard_library.fmap.insert("fully".to_string(), eval::constructors::fully::fully);
    standard_library.fmap.insert("friendship".to_string(), eval::constructors::friendship::friendship);
    standard_library.fmap.insert("lin".to_string(), eval::constructors::linear::linear);
    standard_library.fmap.insert("linear".to_string(), eval::constructors::linear::linear);
    standard_library.fmap.insert("loop".to_string(), eval::constructors::r#loop::a_loop);
    standard_library.fmap.insert("chop".to_string(), eval::constructors::chop::chop);
    standard_library.fmap.insert("infer".to_string(), eval::constructors::infer::infer);
    standard_library.fmap.insert("rule".to_string(), eval::constructors::infer::rule);
    standard_library.fmap.insert("learn".to_string(), eval::constructors::learn::learn);
    standard_library.fmap.insert("cyc".to_string(), eval::constructors::cyc::cyc);
    standard_library.fmap.insert("flower".to_string(), eval::constructors::flower::flower);
    standard_library.fmap.insert("stages".to_string(), eval::constructors::stages::stages);
    standard_library.fmap.insert("facts".to_string(), eval::constructors::facts::facts);
    standard_library.fmap.insert("vals".to_string(), eval::constructors::vals::vals);

    // commands
    standard_library.fmap.insert("defpart".to_string(), eval::commands::load_part);
    standard_library.fmap.insert("freeze".to_string(), eval::commands::freeze_buffer);
    standard_library.fmap.insert("load-sample".to_string(), eval::commands::load_sample);
    standard_library.fmap.insert("load-wavematrix".to_string(), eval::commands::load_sample_as_wavematrix);
    standard_library.fmap.insert("load-sample-sets".to_string(), eval::commands::load_sample_sets);
    standard_library.fmap.insert("load-sample-set".to_string(), eval::commands::load_sample_set);
    standard_library.fmap.insert("tmod".to_string(), eval::commands::tmod);
    standard_library.fmap.insert("latency".to_string(), eval::commands::latency);
    standard_library.fmap.insert("bpm".to_string(), eval::commands::bpm);
    standard_library.fmap.insert("default-duration".to_string(), eval::commands::default_duration);
    standard_library.fmap.insert("globres".to_string(), eval::commands::globres);
    standard_library.fmap.insert("global-resources".to_string(), eval::commands::globres);
    standard_library.fmap.insert("reverb".to_string(), eval::commands::reverb);
    standard_library.fmap.insert("delay".to_string(), eval::commands::delay);
    standard_library.fmap.insert("export-dot".to_string(), eval::commands::export_dot);
    standard_library.fmap.insert("once".to_string(), eval::commands::once);
    standard_library.fmap.insert("step-part".to_string(), eval::commands::step_part);
    standard_library.fmap.insert("clear".to_string(), eval::commands::clear);
    standard_library.fmap.insert("connect-visualizer".to_string(), eval::commands::connect_visualizer);
    standard_library.fmap.insert("rec".to_string(), eval::commands::start_recording);
    standard_library.fmap.insert("stop-rec".to_string(), eval::commands::stop_recording);
    standard_library.fmap.insert("midi-callback".to_string(), eval::commands::define_midi_callback);
    standard_library.fmap.insert("import-sample-set".to_string(), eval::commands::import_sample_set);
    
    // control event
    standard_library.fmap.insert("ctrl".to_string(), eval::events::control::control);

    // parameter structs
    standard_library.fmap.insert("vec".to_string(), eval::structs::vec);
    standard_library.fmap.insert("mat".to_string(), eval::structs::mat);
    
    // sound events (sample events are added as needed)
    standard_library.fmap.insert("risset".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("saw".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("wsaw".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("fmsaw".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("fmsqr".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("fmtri".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("sqr".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("cub".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("tri".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("sine".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("~".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("silence".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("feedr".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("freezr".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("wtab".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("wmat".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("white".to_string(), eval::events::sound::sound);
    standard_library.fmap.insert("brown".to_string(), eval::events::sound::sound);

    // modulators
    standard_library.fmap.insert("lfo~".to_string(), eval::events::modulators::lfo_modulator);
    standard_library.fmap.insert("lfsaw~".to_string(), eval::events::modulators::lfsaw_modulator);
    standard_library.fmap.insert("lfrsaw~".to_string(), eval::events::modulators::lfrsaw_modulator);
    standard_library.fmap.insert("lfsqr~".to_string(), eval::events::modulators::lfsquare_modulator);
    standard_library.fmap.insert("lftri~".to_string(), eval::events::modulators::lftri_modulator);
    standard_library.fmap.insert("linramp~".to_string(), eval::events::modulators::lin_ramp_modulator);
    standard_library.fmap.insert("logramp~".to_string(), eval::events::modulators::log_ramp_modulator);
    standard_library.fmap.insert("expramp~".to_string(), eval::events::modulators::exp_ramp_modulator);
    standard_library.fmap.insert("env~".to_string(), eval::events::modulators::multi_point_envelope_modulator);
    
    // parameter events

    // symbolic type paramerters
    standard_library.fmap.insert("lpt".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpt".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atkt".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dect".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("relt".to_string(), eval::events::parameters::parameter);
    
    standard_library.fmap.insert("pitch".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pitch-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pitch-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pitch-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pitch-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("freq".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("freq-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("freq-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("freq-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("freq-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("lvl".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lvl-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lvl-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lvl-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lvl-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("lpf".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpf-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpf-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpf-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpf-div".to_string(), eval::events::parameters::parameter);
    
    standard_library.fmap.insert("lpd".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpd-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpd-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpd-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpd-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("lpq".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpq-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpq-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpq-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("lpq-div".to_string(), eval::events::parameters::parameter);
    
    standard_library.fmap.insert("pff".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pff-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pff-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pff-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pff-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("pfq".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfq-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfq-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfq-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfq-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("pfg".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfg-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfg-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfg-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pfg-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("hpf".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpf-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpf-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpf-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpf-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("hpq".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpq-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpq-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpq-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("hpq-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("azi".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("azi-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("azi-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("azi-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("azi-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("ele".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("ele-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("ele-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("ele-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("ele-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("atk".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atk-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atk-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atk-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atk-div".to_string(), eval::events::parameters::parameter);

    // attack peak
    standard_library.fmap.insert("atkp".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atkp-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atkp-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atkp-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("atkp-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("sus".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("sus-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("sus-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("sus-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("sus-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("dec".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dec-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dec-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dec-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dec-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("rel".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rel-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rel-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rel-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rel-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("pos".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pos-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pos-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pos-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pos-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("dur".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dur-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dur-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dur-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dur-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("del".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("del-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("del-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("del-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("del-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("rev".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rev-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rev-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rev-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rev-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("pw".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pw-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pw-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pw-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("pw-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("start".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("start-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("start-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("start-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("start-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("rate".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rate-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rate-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rate-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("rate-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("gain".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("gain-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("gain-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("gain-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("gain-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("dist".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dist-add".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dist-mul".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dist-sub".to_string(), eval::events::parameters::parameter);
    standard_library.fmap.insert("dist-div".to_string(), eval::events::parameters::parameter);

    standard_library.fmap.insert("keys".to_string(), eval::events::parameters::sample_keys);
    standard_library.fmap.insert("keys-add".to_string(), eval::events::parameters::sample_keys);
    standard_library.fmap.insert("keys-sub".to_string(), eval::events::parameters::sample_keys);

    standard_library.fmap.insert("sample-number".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sample-number-add".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sample-number-mul".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sample-number-sub".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sample-number-div".to_string(), eval::events::parameters::sample_number);

    standard_library.fmap.insert("sno".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sno-add".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sno-mul".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sno-sub".to_string(), eval::events::parameters::sample_number);
    standard_library.fmap.insert("sno-div".to_string(), eval::events::parameters::sample_number);

    standard_library.fmap.insert("random-sample".to_string(), eval::events::parameters::random_sample);
    standard_library.fmap.insert("rands".to_string(), eval::events::parameters::random_sample);

        
    // some shorthands 
    standard_library.fmap.insert("transpose".to_string(), eval::events::parameters::transpose);
    standard_library.fmap.insert("tpo".to_string(), eval::events::parameters::transpose);

    // dynpars
    standard_library.fmap.insert("bounce".to_string(), eval::dynpar::bounce);
    standard_library.fmap.insert("brownian".to_string(), eval::dynpar::brownian);
    standard_library.fmap.insert("randr".to_string(), eval::dynpar::randrange);
    standard_library.fmap.insert("env".to_string(), eval::dynpar::env);
    standard_library.fmap.insert("fade".to_string(), eval::dynpar::fade);

    // generator processors
    standard_library.fmap.insert("pear".to_string(), eval::generator_processor::eval_pear);
    standard_library.fmap.insert("apple".to_string(), eval::generator_processor::eval_apple);
    standard_library.fmap.insert("every".to_string(), eval::generator_processor::eval_every);
    standard_library.fmap.insert("life".to_string(), eval::generator_processor::eval_lifemodel);
    standard_library.fmap.insert("inhibit".to_string(), eval::generator_processor::eval_inhibit);
    standard_library.fmap.insert("exhibit".to_string(), eval::generator_processor::eval_exhibit);

    // composition
    standard_library.fmap.insert("cmp".to_string(), eval::compose::compose);
    standard_library.fmap.insert("compose".to_string(), eval::compose::compose);
    standard_library.fmap.insert("ls".to_string(), eval::generator_list::generator_list);
    standard_library.fmap.insert("list".to_string(), eval::generator_list::generator_list);

    // multiplyer
    standard_library.fmap.insert("xspread".to_string(), eval::multiplyer::eval_xspread);
    standard_library.fmap.insert("xdup".to_string(), eval::multiplyer::eval_xdup);
    
    // generator modifiers
    standard_library.fmap.insert("haste".to_string(), eval::generator_modifier::eval_haste);
    standard_library.fmap.insert("relax".to_string(), eval::generator_modifier::eval_relax);
    standard_library.fmap.insert("grow".to_string(), eval::generator_modifier::eval_grow);
    standard_library.fmap.insert("grown".to_string(), eval::generator_modifier::eval_grown);
    standard_library.fmap.insert("shrink".to_string(), eval::generator_modifier::eval_shrink);
    standard_library.fmap.insert("solidify".to_string(), eval::generator_modifier::eval_solidify);
    standard_library.fmap.insert("blur".to_string(), eval::generator_modifier::eval_blur);
    standard_library.fmap.insert("sharpen".to_string(), eval::generator_modifier::eval_sharpen);
    standard_library.fmap.insert("shake".to_string(), eval::generator_modifier::eval_shake);
    standard_library.fmap.insert("skip".to_string(), eval::generator_modifier::eval_skip);
    standard_library.fmap.insert("rewind".to_string(), eval::generator_modifier::eval_rewind);
    standard_library.fmap.insert("rnd".to_string(), eval::generator_modifier::eval_rnd);
    standard_library.fmap.insert("rep".to_string(), eval::generator_modifier::eval_rep);
    standard_library.fmap.insert("reverse".to_string(), eval::generator_modifier::eval_reverse);
    standard_library.fmap.insert("keep".to_string(), eval::generator_modifier::eval_keep);

    // arithmetic
    standard_library.fmap.insert("add".to_string(), eval::arithmetic::add);
    standard_library.fmap.insert("mul".to_string(), eval::arithmetic::mul);
    standard_library.fmap.insert("sub".to_string(), eval::arithmetic::sub);
    standard_library.fmap.insert("div".to_string(), eval::arithmetic::div);
    standard_library.fmap.insert("mod".to_string(), eval::arithmetic::modulo);
    standard_library.fmap.insert("pow".to_string(), eval::arithmetic::pow);

    standard_library
}
