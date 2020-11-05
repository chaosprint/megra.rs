use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
    character::{is_alphanumeric, is_space},
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use parking_lot::Mutex;
use std::collections::HashMap;
use vom_rs::pfa::Pfa;
use crate::markov_sequence_generator::MarkovSequenceGenerator;
use crate::event::*;
use crate::parameter::*;

/// As this doesn't strive to be a turing-complete lisp, we'll start with the basic
/// megra operations, learning and inferring, plus the built-in events
pub enum BuiltIn {
    Learn,
    Infer,
    Sine,
    Saw,
}

pub enum Atom {
    Num(i32),
    Description(String), // pfa descriptions
    Keyword(String),
    Symbol(String),
    Boolean(bool),
    BuiltIn(BuiltIn),
    MarkovSequenceGenerator(MarkovSequenceGenerator),
    Event(Event)
}

pub enum Expr {
    Constant(Atom),
    /// (func-name arg1 arg2)
    Application(Box<Expr>, Vec<Expr>),
}


fn parse_builtin<'a>(i: &'a str) -> IResult<&'a str, BuiltIn, VerboseError<&'a str>> {
    // alt gives us the result of first parser that succeeds, of the series of
    // parsers we give it
    alt((	
	// map lets us process the parsed output, in this case we know what we parsed,
	// so we ignore the input and return the BuiltIn directly
	map(tag("learn"), |_| BuiltIn::Learn),
	map(tag("infer"), |_| BuiltIn::Infer),
	map(tag("sine"), |_| BuiltIn::Sine),
	map(tag("saw"), |_| BuiltIn::Saw),
    ))(i)
}

/// Our boolean values are also constant, so we can do it the same way
fn parse_bool<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
	map(tag("#t"), |_| Atom::Boolean(true)),
	map(tag("#f"), |_| Atom::Boolean(false)),
    ))(i)
}

fn parse_keyword<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(
	context("keyword", preceded(tag(":"), cut(alpha1))),
	|sym_str: &str| Atom::Keyword(sym_str.to_string()),
    )(i)
}

fn parse_symbol<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(
	context("symbol", preceded(tag("'"), cut(alpha1))),
	|sym_str: &str| Atom::Symbol(sym_str.to_string()),
    )(i)
}

fn parse_num<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
	map_res(digit1, |digit_str: &str| {
	    digit_str.parse::<i32>().map(Atom::Num)
	}),
	map(preceded(tag("-"), digit1), |digit_str: &str| {
	    Atom::Num(-1 * digit_str.parse::<i32>().unwrap())
	}),
    ))(i)
}

pub fn valid_char(chr: char) -> bool {
    return chr == '~' || is_alphanumeric(chr as u8) || is_space(chr as u8)
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(delimited(
        tag("\""),
        take_while(valid_char),
        tag("\"")), |desc_str: &str| {
	Atom::Description(desc_str.to_string())
    })(i)
}

fn parse_atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
	parse_num,
	parse_bool,
	map(parse_builtin, Atom::BuiltIn),
	parse_keyword,
	parse_symbol,
	parse_string
    ))(i)
}

/// We then add the Expr layer on top
fn parse_constant<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    map(parse_atom, |atom| Expr::Constant(atom))(i)
}
/// Before continuing, we need a helper function to parse lists.
/// A list starts with `(` and ends with a matching `)`.
/// By putting whitespace and newline parsing here, we can avoid having to worry about it
/// in much of the rest of the parser.
///
/// Unlike the previous functions, this function doesn't take or consume input, instead it
/// takes a parsing function and returns a new parsing function.
fn s_exp<'a, O1, F>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O1, VerboseError<&'a str>>
where
    F: Parser<&'a str, O1, VerboseError<&'a str>>,
{
    delimited(
	char('('),
	preceded(multispace0, inner),
	context("closing paren", cut(preceded(multispace0, char(')')))),
    )
}

/// We can now use our new combinator to define the rest of the `Expr`s.
///
/// Starting with function application, we can see how the parser mirrors our data
/// definitions: our definition is `Application(Box<Expr>, Vec<Expr>)`, so we know
/// that we need to parse an expression and then parse 0 or more expressions, all
/// wrapped in an S-expression.
///
/// `tuple` is used to sequence parsers together, so we can translate this directly
/// and then map over it to transform the output into an `Expr::Application`
fn parse_application<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    let application_inner = map(tuple((parse_expr, many0(parse_expr))), |(head, tail)| {
	Expr::Application(Box::new(head), tail)
    });
    // finally, we wrap it in an s-expression
    s_exp(application_inner)(i)
}

/// We tie them all together again, making a top-level expression parser!
fn parse_expr<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
    preceded(
	multispace0,
	alt((parse_constant, parse_application)),
    )(i)
}

/// And that's it!
/// We can now parse our entire lisp language.
///
/// But in order to make it a little more interesting, we can hack together
/// a little interpreter to take an Expr, which is really an
/// [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree) (AST),
/// and give us something back

/// To start we define a couple of helper functions
fn get_num_from_expr(e: &Expr) -> Option<i32> {
    if let Expr::Constant(Atom::Num(n)) = e {
	Some(*n)
    } else {
	None
    }
}

fn get_bool_from_expr(e: &Expr) -> Option<bool> {
    if let Expr::Constant(Atom::Boolean(b)) = e {
	Some(*b)
    } else {
	None
    }
}

fn get_string_from_expr(e: &Expr) -> Option<String> {
    if let Expr::Constant(Atom::Description(s)) = e {
	Some(s.to_string())
    } else if let Expr::Constant(Atom::Symbol(s)) = e {
	Some(s.to_string())
    } else {
	None
    }
}

/// This function tries to reduce the AST.
/// This has to return an Expression rather than an Atom because quoted s_expressions
/// can't be reduced
fn eval_expression(e: Expr) -> Option<Expr> {
    match e {
	// Constants and quoted s-expressions are our base-case
	Expr::Constant(_) => Some(e),	
	Expr::Application(head, tail) => {
	    let reduced_head = eval_expression(*head)?;
	    let mut reduced_tail = tail
		.into_iter()
		.map(|expr| eval_expression(expr))
		.collect::<Option<Vec<Expr>>>()?;

	    let mut tail_drain = reduced_tail.drain(..);
	    
	    if let Expr::Constant(Atom::BuiltIn(bi)) = reduced_head {
		Some(Expr::Constant(match bi {
		    BuiltIn::Learn => {
						
			// name is the first symbol
			let name: String = get_string_from_expr(&tail_drain.next().unwrap()).unwrap();
			
			let mut sample:String = "".to_string();
			let mut event_mapping = HashMap::<char, Vec<Event>>::new();
			
			let mut collect_events = false;
			let mut skip = false;
			let mut dur = 200;

			while let Some(Expr::Constant(c)) = tail_drain.next() {

			    if collect_events {
				if let Atom::Symbol(ref s) = c {
				    let mut ev_vec = Vec::new();
				    if let Expr::Constant(Atom::Event(e)) = tail_drain.next().unwrap() {
					ev_vec.push(e);
				    }
				    event_mapping.insert(s.chars().next().unwrap(), ev_vec);
				    continue;
				} else {
				    collect_events = false;
				}
				    
			    }
			    
			    match c {
				Atom::Keyword(k) => {
				    match k.as_str() {
					"sample" => {
					    if let Expr::Constant(Atom::Description(desc)) = tail_drain.next().unwrap() {
						sample = desc.to_string();
					    }	
					},
					"events" => {
					    collect_events = true;
					    continue;
					},
					"dur" => {
					    if let Expr::Constant(Atom::Num(n)) = tail_drain.next().unwrap() {
						dur = n;
					    }
					},
					_ => println!("{}", k)
				    }
				}
				_ => println!{"ignored"}
			    }
			}
						    					
			let s_v: std::vec::Vec<char> = sample.chars().collect();
			let pfa = Pfa::<char>::learn(&s_v, 3, 0.01, 30);
			Atom::MarkovSequenceGenerator (MarkovSequenceGenerator {
			    name: name,
			    generator: pfa,
			    event_mapping: event_mapping,
			    duration_mapping: HashMap::new(),
			    modified: false,
			    symbol_ages: HashMap::new(),
			    default_duration: dur as u64,
			    last_transition: None,			    
			})
		    },
		    BuiltIn::Infer => Atom::MarkovSequenceGenerator (MarkovSequenceGenerator {
			name: "hulli".to_string(),
			generator: Pfa::<char>::new(),
			event_mapping: HashMap::new(),
			duration_mapping: HashMap::new(),
			modified: false,
			symbol_ages: HashMap::new(),
			default_duration: 200,
			last_transition: None,			    
		    }),
		    BuiltIn::Sine => {
			let mut ev = Event::with_name("sine".to_string());
			ev.tags.push("sine".to_string());
			ev.params.insert("freq".to_string(),Box::new(Parameter::with_value(get_num_from_expr(&tail_drain.next().unwrap()).unwrap() as f32)));
			ev.params.insert("lvl".to_string(), Box::new(Parameter::with_value(1.0)));
			ev.params.insert("atk".to_string(), Box::new(Parameter::with_value(0.01)));
			ev.params.insert("sus".to_string(), Box::new(Parameter::with_value(0.1)));
			ev.params.insert("rel".to_string(), Box::new(Parameter::with_value(0.01)));
			Atom::Event (ev)
		    },
		    BuiltIn::Saw => {
			let mut ev = Event::with_name("saw".to_string());
			ev.tags.push("sine".to_string());
			ev.params.insert("freq".to_string(), Box::new(Parameter::with_value(get_num_from_expr(&tail_drain.next().unwrap()).unwrap() as f32)));
			ev.params.insert("lvl".to_string(), Box::new(Parameter::with_value(1.0)));
			ev.params.insert("atk".to_string(), Box::new(Parameter::with_value(0.01)));
			ev.params.insert("sus".to_string(), Box::new(Parameter::with_value(0.1)));
			ev.params.insert("rel".to_string(), Box::new(Parameter::with_value(0.01)));
			Atom::Event (ev)
		    },		    
		}))
	    } else {
		None
	    }
	}
    }
}

/// And we add one more top-level function to tie everything together, letting
/// us call eval on a string directly
pub fn eval_from_str(src: &str) -> Result<Expr, String> {
    parse_expr(src)
	.map_err(|e: nom::Err<VerboseError<&str>>| format!("{:#?}", e))
	.and_then(|(_, exp)| eval_expression(exp).ok_or("Eval failed".to_string()))
}


