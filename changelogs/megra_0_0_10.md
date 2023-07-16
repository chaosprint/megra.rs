# Changes for Mégra Version 0.0.10:

* introduce `progn`
* introduce `fun` (function definition)
* introduce `callback` (callback definition, same as `fun` but as a mnemonic)
* introduce `let` (variable definition), `defpart` now maps to `let` (no change from user perspective)
* introduce `print`
* osc sender
* some extra types (f64, i32, i64) for osc sender
* osc receiver 
* osc callbacks (toplevel functions with args)
* much more flexible midi callback (toplevel functions with args)
* start midi port from language instead of from command line
* negative rates for samples (can't believe I didn't think about that before ...)
