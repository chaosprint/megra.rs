use crate::builtin_types::*;
use crate::parser::{EvaluatedExpr, FunctionMap};
use crate::{OutputMode, SampleAndWavematrixSet};
use parking_lot::Mutex;
use std::sync;

pub fn osc_define_sender(
    _: &FunctionMap,
    tail: &mut Vec<EvaluatedExpr>,
    _: &sync::Arc<VariableStore>,
    _: &sync::Arc<Mutex<SampleAndWavematrixSet>>,
    _: OutputMode,
) -> Option<EvaluatedExpr> {
    let mut tail_drain = tail.drain(..);
    tail_drain.next();
    let sender_name = if let Some(EvaluatedExpr::Typed(TypedEntity::Symbol(s))) = tail_drain.next()
    {
        s
    } else {
        return None;
    };
    let host_name = if let Some(EvaluatedExpr::Typed(TypedEntity::String(s))) = tail_drain.next() {
        s
    } else {
        return None;
    };

    Some(EvaluatedExpr::Command(Command::OscDefineClient(
        sender_name,
        host_name,
    )))
}

pub fn osc_send(
    _: &FunctionMap,
    tail: &mut Vec<EvaluatedExpr>,
    _: &sync::Arc<VariableStore>,
    _: &sync::Arc<Mutex<SampleAndWavematrixSet>>,
    _: OutputMode,
) -> Option<EvaluatedExpr> {
    let mut tail_drain = tail.drain(..);
    tail_drain.next();

    let sender_name = if let Some(EvaluatedExpr::Typed(TypedEntity::Symbol(s))) = tail_drain.next()
    {
        s
    } else {
        return None;
    };
    let addr = if let Some(EvaluatedExpr::Typed(TypedEntity::String(s))) = tail_drain.next() {
        s
    } else {
        return None;
    };

    let mut args = Vec::new();
    for thing in tail_drain {
        match thing {
            EvaluatedExpr::Typed(TypedEntity::Float(f)) => args.push(TypedEntity::Float(f)),
            EvaluatedExpr::Typed(TypedEntity::Double(f)) => args.push(TypedEntity::Double(f)),
            EvaluatedExpr::Typed(TypedEntity::Int32(f)) => args.push(TypedEntity::Int32(f)),
            EvaluatedExpr::Typed(TypedEntity::Int64(f)) => args.push(TypedEntity::Int64(f)),
            EvaluatedExpr::Typed(TypedEntity::Symbol(s)) => args.push(TypedEntity::Symbol(s)),
            EvaluatedExpr::Typed(TypedEntity::String(s)) => args.push(TypedEntity::String(s)),
            _ => {}
        }
    }

    Some(EvaluatedExpr::Command(Command::OscSendMessage(
        sender_name,
        addr,
        args,
    )))
}

pub fn osc_start_receiver(
    _: &FunctionMap,
    tail: &mut Vec<EvaluatedExpr>,
    _: &sync::Arc<VariableStore>,
    _: &sync::Arc<Mutex<SampleAndWavematrixSet>>,
    _: OutputMode,
) -> Option<EvaluatedExpr> {
    let mut tail_drain = tail.drain(..);
    tail_drain.next();

    let host_name = if let Some(EvaluatedExpr::Typed(TypedEntity::String(s))) = tail_drain.next() {
        s
    } else {
        return None;
    };

    Some(EvaluatedExpr::Command(Command::OscStartReceiver(host_name)))
}

pub fn osc_define_callback(
    _: &FunctionMap,
    tail: &mut Vec<EvaluatedExpr>,
    _: &sync::Arc<VariableStore>,
    _: &sync::Arc<Mutex<SampleAndWavematrixSet>>,
    _: OutputMode,
) -> Option<EvaluatedExpr> {
    let mut tail_drain = tail.drain(..);
    tail_drain.next();

    let addr = if let Some(EvaluatedExpr::Typed(TypedEntity::String(s))) = tail_drain.next() {
        s
    } else {
        return None;
    };

    tail_drain
        .next()
        .map(|c| EvaluatedExpr::Command(Command::OscDefineCallback(addr, Box::new(c))))
}
