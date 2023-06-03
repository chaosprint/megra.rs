use crate::event::{ControlEvent, Event};
use crate::generator::Generator;
use crate::markov_sequence_generator::Rule;
use crate::parameter::{DynVal, ParameterValue};
use crate::session::SyncContext;
use crate::{Command, GeneratorProcessorOrModifier, PartProxy, VariableStore};
use crate::{OutputMode, SampleAndWavematrixSet};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace0, multispace1},
    character::{is_alphanumeric, is_newline, is_space},
    combinator::{cut, map, map_res, recognize},
    error::{context, ErrorKind, VerboseError, VerboseErrorKind},
    multi::many0,
    number::complete::float,
    sequence::{delimited, preceded, tuple},
    Err, IResult, Parser,
};
use parking_lot::Mutex;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::sync;

pub mod eval;

/// These are the basic building blocks of our casual lisp language.
/// You might notice that there's no lists in this lisp ... not sure
/// what to call it in that case ...
#[derive(Debug, Clone)]
pub enum Atom {
    Float(f32),
    String(String),
    Keyword(String),
    Symbol(String),
    Boolean(bool),
    Identifier(String),
}

/// Expression Type
#[derive(Debug, Clone)]
pub enum Expr {
    FunctionDefinition,
    VariableDefinition,
    Constant(Atom),
    Application(Box<Expr>, Vec<Expr>),
    Definition(Box<Expr>, Vec<Expr>),
}

pub enum BuiltIn {
    Rule(Rule),
    Command(Command),
    DefineMidiCallback(u8, Command),
    PartProxy(PartProxy),
    ProxyList(Vec<PartProxy>),
    Generator(Generator),
    GeneratorList(Vec<Generator>),
    GeneratorProcessorOrModifier(GeneratorProcessorOrModifier),
    GeneratorProcessorOrModifierList(Vec<GeneratorProcessorOrModifier>),
    GeneratorModifierList(Vec<GeneratorProcessorOrModifier>),
    SoundEvent(Event),
    Parameter(DynVal),
    Modulator(ParameterValue),
    Matrix(ParameterValue),
    Vector(ParameterValue),
    ControlEvent(ControlEvent),
    SyncContext(SyncContext),
}

impl fmt::Debug for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltIn::Rule(_) => write!(f, "BuiltIn::Rule(..)"),
            BuiltIn::Command(_) => write!(f, "BuiltIn::Command(..)"),
            BuiltIn::DefineMidiCallback(_, _) => write!(f, "BuiltIn::DefineMidiCallback(..)"),
            BuiltIn::PartProxy(_) => write!(f, "BuiltIn::PartProxy(..)"),
            BuiltIn::ProxyList(_) => write!(f, "BuiltIn::ProxyList(..)"),
            BuiltIn::Generator(g) => write!(f, "BuiltIn::Generator({:?})", g.id_tags),
            BuiltIn::GeneratorList(_) => write!(f, "BuiltIn::GeneratorList(..)"),
            BuiltIn::GeneratorProcessorOrModifier(_) => {
                write!(f, "BuiltIn::GeneratorProcessorOrModifier(..)")
            }
            BuiltIn::GeneratorProcessorOrModifierList(_) => {
                write!(f, "BuiltIn::GeneratorProcessorOrModifierList(..)")
            }
            BuiltIn::GeneratorModifierList(_) => write!(f, "BuiltIn::GeneratorModifierList(..)"),
            BuiltIn::SoundEvent(_) => write!(f, "BuiltIn::SoundEvent(..)"),
            BuiltIn::Parameter(_) => write!(f, "BuiltIn::Parameter(..)"),
            BuiltIn::Modulator(_) => write!(f, "BuiltIn::Modulator(..)"),
            BuiltIn::Vector(_) => write!(f, "BuiltIn::Vector(..)"),
            BuiltIn::Matrix(_) => write!(f, "BuiltIn::Matrix(..)"),
            BuiltIn::ControlEvent(_) => write!(f, "BuiltIn::ControlEvent(..)"),
            BuiltIn::SyncContext(_) => write!(f, "BuiltIn::SyncContext(..)"),
        }
    }
}

pub enum EvaluatedExpr {
    Float(f32),
    Symbol(String),
    Keyword(String),
    String(String),
    Boolean(bool),
    Identifier(String),
    BuiltIn(BuiltIn),
    Progn(Vec<EvaluatedExpr>),
    FunctionDefinition,
    VariableDefinition,
}

impl fmt::Debug for EvaluatedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluatedExpr::Float(fl) => write!(f, "EvaluatedExpr::Float({fl})"),
            EvaluatedExpr::Symbol(s) => write!(f, "EvaluatedExpr::Symbol({s})"),
            EvaluatedExpr::Keyword(k) => write!(f, "EvaluatedExpr::Keyword({k})"),
            EvaluatedExpr::String(s) => write!(f, "EvaluatedExpr::String({s})"),
            EvaluatedExpr::Boolean(b) => write!(f, "EvaluatedExpr::Boolean({b})"),
            EvaluatedExpr::Identifier(fna) => write!(f, "EvaluatedExpr::Identifier({fna})"),
            EvaluatedExpr::BuiltIn(b) => write!(f, "EvaluatedExpr::BuiltIn({b:?})"),
            EvaluatedExpr::Progn(_) => write!(f, "EvaluatedExpr::Progn"),
            EvaluatedExpr::FunctionDefinition => write!(f, "EvaluatedExpr::FunctionDefinition"),
            EvaluatedExpr::VariableDefinition => write!(f, "EvaluatedExpr::VariableDefinition"),
        }
    }
}

// std_lib are hard-coded,
// usr_lib is for user-defined functions ...
pub struct FunctionMap {
    pub usr_lib: HashMap<String, Vec<Expr>>,
    pub std_lib: HashMap<
        String,
        fn(
            &mut FunctionMap,
            &mut Vec<EvaluatedExpr>,
            &sync::Arc<VariableStore>,
            &sync::Arc<Mutex<SampleAndWavematrixSet>>,
            OutputMode,
        ) -> Option<EvaluatedExpr>,
    >,
}

impl FunctionMap {
    pub fn new() -> Self {
        FunctionMap {
            std_lib: HashMap::new(),
            usr_lib: HashMap::new(),
        }
    }
}

/// valid chars for a string
fn valid_string_char(chr: char) -> bool {
    chr == '~'
        || chr == '.'
        || chr == '\''
        || chr == '_'
        || chr == '/'
        || chr == '-'
        || chr == ':'
        || chr == '='
        || chr == '['
        || chr == ']'
        || is_alphanumeric(chr as u8)
        || is_space(chr as u8)
        || is_newline(chr as u8)
}

/// valid chars for a function name, symbol or keyword
pub fn valid_identifier_name_char(chr: char) -> bool {
    chr == '_' || chr == '~' || chr == '-' || is_alphanumeric(chr as u8)
}

/// parse a string, which is enclosed in double quotes
fn parse_string(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    map(
        delimited(tag("\""), take_while(valid_string_char), tag("\"")),
        |desc_str: &str| Atom::String(desc_str.to_string()),
    )(i)
}

/// booleans have a # prefix
fn parse_boolean(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    alt((
        map(tag("#t"), |_| Atom::Boolean(true)),
        map(tag("#f"), |_| Atom::Boolean(false)),
    ))(i)
}

fn parse_definition(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    alt((
        map(tag("function"), |_| Expr::FunctionDefinition),
        map(tag("let"), |_| Expr::VariableDefinition),
    ))(i)
}

/// keywords are language constructs that start with a ':'
fn parse_keyword(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    map(
        context(
            "keyword",
            preceded(tag(":"), take_while(valid_identifier_name_char)),
        ),
        |sym_str: &str| Atom::Keyword(sym_str.to_string()),
    )(i)
}

/// keywords are language constructs that start with a single quote
pub fn parse_symbol(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    map(
        context(
            "symbol",
            preceded(tag("'"), take_while(valid_identifier_name_char)),
        ),
        |sym_str: &str| Atom::Symbol(sym_str.to_string()),
    )(i)
}

/// function names are language constructs that contain allowed function name chars
fn parse_identifier(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    map(
        context("identifer", take_while1(valid_identifier_name_char)),
        |sym_str: &str| Atom::Identifier(sym_str.to_string()),
    )(i)
}

/// floating point numbers ... all numbers currently are ...
pub fn parse_float(i: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    // manually disallowing "infinity" because it doesn't make much sense here
    // and clashes with "infer", which led to an error ...
    if i.starts_with("inf") {
        Err(Err::Error(VerboseError {
            errors: vec![(
                "infinity disallowed",
                VerboseErrorKind::Nom(ErrorKind::Float),
            )],
        }))
    } else {
        map_res(recognize(float), |digit_str: &str| {
            digit_str.parse::<f32>().map(Atom::Float)
        })(i)
    }
}

/// parse all the atoms
fn parse_constant(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    map(
        alt((
            parse_boolean,
            parse_float,
            parse_keyword,
            parse_symbol,
            parse_string,
            parse_identifier,
        )),
        Expr::Constant,
    )(i)
}

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
fn parse_application(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let application_inner = map(
        tuple((
            parse_expr,
            many0(alt((
                preceded(multispace0, parse_application), // applications can follow one another without whitespace
                preceded(multispace1, parse_constant), // constants are delimited by at least one whitespace
            ))),
        )),
        |(head, tail)| match head {
            Expr::FunctionDefinition => Expr::Definition(Box::new(head), tail),
            Expr::VariableDefinition => Expr::Definition(Box::new(head), tail),
            _ => Expr::Application(Box::new(head), tail),
        },
    );
    // finally, we wrap it in an s-expression
    s_exp(application_inner)(i)
}

/// We tie them all together again, making a top-level expression parser!
/// This one generates the abstract syntax tree
pub fn parse_expr(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    alt((parse_definition, parse_application, parse_constant))(i)
}

/// This one reduces the abstract syntax tree ...
pub fn eval_expression(
    e: &Expr,
    functions: &mut FunctionMap,
    globals: &sync::Arc<VariableStore>,
    sample_set: &sync::Arc<Mutex<SampleAndWavematrixSet>>,
    out_mode: OutputMode,
) -> Option<EvaluatedExpr> {
    match e {
        Expr::Constant(c) => Some(match c {
            Atom::Float(f) => EvaluatedExpr::Float(*f),
            Atom::Symbol(s) => EvaluatedExpr::Symbol(s.to_string()),
            Atom::Keyword(k) => EvaluatedExpr::Keyword(k.to_string()),
            Atom::String(s) => EvaluatedExpr::String(s.to_string()),
            Atom::Boolean(b) => EvaluatedExpr::Boolean(*b),
            Atom::Identifier(f) => EvaluatedExpr::Identifier(f.to_string()),
        }),
        Expr::Application(head, tail) => {
            if let Some(EvaluatedExpr::Identifier(f)) =
                eval_expression(head, functions, globals, sample_set, out_mode)
            {
                // check if we have this function ...
                if functions.std_lib.contains_key(&f) {
                    let mut reduced_tail = tail
                        .iter()
                        .map(|expr| eval_expression(expr, functions, globals, sample_set, out_mode))
                        .collect::<Option<Vec<EvaluatedExpr>>>()?;
                    // push function name
                    reduced_tail.insert(0, EvaluatedExpr::Identifier(f.clone()));
                    functions.std_lib[&f](
                        functions,
                        &mut reduced_tail,
                        globals,
                        sample_set,
                        out_mode,
                    )
                } else if functions.usr_lib.contains_key(&f) {
                    let fun_expr = functions.usr_lib.get(&f).unwrap().clone();

                    let fun_tail = fun_expr
                        .iter()
                        .map(|expr| eval_expression(expr, functions, globals, sample_set, out_mode))
                        .collect::<Option<Vec<EvaluatedExpr>>>();

                    // return last form result, cl-style
                    fun_tail?.pop()
                } else {
                    None
                }
            } else {
                None
            }
        }
        Expr::Definition(head, tail) => match **head {
            Expr::FunctionDefinition => {
                if let Some(EvaluatedExpr::Identifier(i)) =
                    eval_expression(&tail[0], functions, globals, sample_set, out_mode)
                {
                    let mut tail_clone = tail.clone();
                    tail_clone.remove(0);
                    functions.usr_lib.insert(i, tail_clone);
                }
                Some(EvaluatedExpr::FunctionDefinition)
            }
            Expr::VariableDefinition => Some(EvaluatedExpr::VariableDefinition),
            _ => None,
        },
        _ => None,
    }
}

pub fn eval_from_str(
    src: &str,
    functions: &mut FunctionMap,
    globals: &sync::Arc<VariableStore>,
    sample_set: &sync::Arc<Mutex<SampleAndWavematrixSet>>,
    out_mode: OutputMode,
) -> Result<EvaluatedExpr, String> {
    // preprocessing - remove all comments ...
    let re = Regex::new(r";[^\n]+\n").unwrap();
    let src_nocomment = re.replace_all(src, "\n");
    parse_expr(&src_nocomment)
        .map_err(|e: nom::Err<VerboseError<&str>>| format!("{e:#?}"))
        .and_then(|(_, exp)| {
            eval_expression(&exp, functions, globals, sample_set, out_mode)
                .ok_or_else(|| "eval failed".to_string())
        })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_eval() {
        let snippet = "(text 'tar :lvl 1.0 :global #t :relate #f :boost (bounce 0 400))";

        let mut functions = FunctionMap::new();
        let globals = sync::Arc::new(VariableStore::new());
        let sample_set = sync::Arc::new(Mutex::new(SampleAndWavematrixSet::new()));

        functions
            .fmap
            .insert("text".to_string(), |_, tail, _, _, _| {
                // SYMBOLS
                if let EvaluatedExpr::Symbol(s) = &tail[1] {
                    assert!(s == "tar");
                } else {
                    panic!();
                }

                // KEYWORDS
                if let EvaluatedExpr::Keyword(k) = &tail[2] {
                    assert!(k == "lvl");
                } else {
                    panic!();
                }

                if let EvaluatedExpr::Keyword(k) = &tail[4] {
                    assert!(k == "global");
                } else {
                    panic!();
                }

                if let EvaluatedExpr::Keyword(k) = &tail[6] {
                    assert!(k == "relate");
                } else {
                    panic!();
                }

                if let EvaluatedExpr::Keyword(k) = &tail[8] {
                    assert!(k == "boost");
                } else {
                    panic!();
                }

                // BOOLEANS
                if let EvaluatedExpr::Boolean(b) = &tail[5] {
                    assert!(b);
                } else {
                    panic!();
                }

                if let EvaluatedExpr::Boolean(b) = &tail[7] {
                    assert!(!b);
                } else {
                    panic!();
                }

                // FLOAT
                if let EvaluatedExpr::Float(f) = &tail[3] {
                    assert!(*f == 1.0);
                } else {
                    panic!();
                }

                Some(EvaluatedExpr::Boolean(true))
            });

        functions
            .fmap
            .insert("bounce".to_string(), |_, tail, _, _, _| {
                if let EvaluatedExpr::Float(f) = &tail[1] {
                    assert!(*f == 0.0);
                } else {
                    panic!();
                }
                if let EvaluatedExpr::Float(f) = &tail[2] {
                    assert!(*f == 400.0);
                } else {
                    panic!();
                }

                Some(EvaluatedExpr::Boolean(true))
            });

        match eval_from_str(
            snippet,
            &functions,
            &globals,
            &sample_set,
            OutputMode::Stereo,
        ) {
            Ok(res) => {
                assert!(matches!(res, EvaluatedExpr::Boolean(true)))
            }
            Err(e) => {
                println!("err {e}");
                panic!()
            }
        }
    }

    #[test]
    fn test_parse_float() {
        assert!(matches!(parse_float("0.0"), Ok(("", Atom::Float(_)))));
        assert!(matches!(parse_float("1.0"), Ok(("", Atom::Float(_)))));
        assert!(matches!(parse_float("-1.0"), Ok(("", Atom::Float(_)))));
    }

    #[test]
    fn test_parse_symbol() {
        assert!(matches!(parse_symbol("'test"), Ok(("", Atom::Symbol(_)))));
        assert!(!matches!(parse_symbol(":test"), Ok(("", Atom::Symbol(_)))));
    }

    #[test]
    fn test_parse_keyword() {
        assert!(matches!(parse_keyword(":test"), Ok(("", Atom::Keyword(_)))));
    }

    #[test]
    fn test_parse_string() {
        assert!(matches!(
            parse_string("\"test\""),
            Ok(("", Atom::String(_)))
        ));
    }

    #[test]
    fn test_parse_boolean() {
        assert!(matches!(parse_boolean("#t"), Ok(("", Atom::Boolean(true)))));
        assert!(matches!(
            parse_boolean("#f"),
            Ok(("", Atom::Boolean(false)))
        ));
    }

    #[test]
    fn test_parse_atom_constant() {
        assert!(matches!(
            parse_constant("#t"),
            Ok(("", Expr::Constant(Atom::Boolean(true))))
        ));
        assert!(matches!(
            parse_constant("#f"),
            Ok(("", Expr::Constant(Atom::Boolean(false))))
        ));
        assert!(matches!(
            parse_constant("'test"),
            Ok(("", Expr::Constant(Atom::Symbol(_))))
        ));
        assert!(matches!(
            parse_constant(":test"),
            Ok(("", Expr::Constant(Atom::Keyword(_))))
        ));
        assert!(matches!(
            parse_constant("\"test\""),
            Ok(("", Expr::Constant(Atom::String(_))))
        ));
    }

    #[test]
    fn test_parse_expr() {
        assert!(matches!(
            parse_expr("#t"),
            Ok(("", Expr::Constant(Atom::Boolean(true))))
        ));
        assert!(matches!(
            parse_expr("#f"),
            Ok(("", Expr::Constant(Atom::Boolean(false))))
        ));
        assert!(matches!(
            parse_expr("'test"),
            Ok(("", Expr::Constant(Atom::Symbol(_))))
        ));
        assert!(matches!(
            parse_expr(":test"),
            Ok(("", Expr::Constant(Atom::Keyword(_))))
        ));
        assert!(matches!(
            parse_expr("\"test\""),
            Ok(("", Expr::Constant(Atom::String(_))))
        ));
        assert!(matches!(
            parse_expr("(#t)"),
            Ok(("", Expr::Application(_, _)))
        ));
        assert!(matches!(
            parse_expr("('test)"),
            Ok(("", Expr::Application(_, _)))
        ));
        assert!(matches!(
            parse_expr("(:test)"),
            Ok(("", Expr::Application(_, _)))
        ));
        assert!(matches!(
            parse_expr("(\"test\")"),
            Ok(("", Expr::Application(_, _)))
        ));

        if let Ok(("", Expr::Application(head, tail))) =
            parse_expr("(text 'tar :lvl 1.0 :global #t :relate #f :boost (bounce 0 400))")
        {
            if let Expr::Constant(Atom::Identifier(function_name)) = *head {
                assert!(function_name == "text");
            } else {
                panic!()
            }

            // SYMBOLS
            if let Expr::Constant(Atom::Symbol(s)) = &tail[0] {
                assert!(s == "tar");
            } else {
                panic!();
            }

            // KEYWORDS
            if let Expr::Constant(Atom::Keyword(k)) = &tail[1] {
                assert!(k == "lvl");
            } else {
                panic!();
            }

            if let Expr::Constant(Atom::Keyword(k)) = &tail[3] {
                assert!(k == "global");
            } else {
                panic!();
            }

            if let Expr::Constant(Atom::Keyword(k)) = &tail[5] {
                assert!(k == "relate");
            } else {
                panic!();
            }

            if let Expr::Constant(Atom::Keyword(k)) = &tail[7] {
                assert!(k == "boost");
            } else {
                panic!();
            }

            // BOOLEANS
            if let Expr::Constant(Atom::Boolean(b)) = &tail[4] {
                assert!(b);
            } else {
                panic!();
            }

            if let Expr::Constant(Atom::Boolean(b)) = &tail[6] {
                assert!(!b);
            } else {
                panic!();
            }

            // FLOAT
            if let Expr::Constant(Atom::Float(f)) = &tail[2] {
                assert!(*f == 1.0);
            } else {
                panic!();
            }

            // APPLICATION
            if let Expr::Application(head2, tail2) = &tail[8] {
                if let Expr::Constant(Atom::Identifier(function_name2)) = &**head2 {
                    assert!(function_name2 == "bounce")
                } else {
                    panic!()
                }
                // FLOAT
                if let Expr::Constant(Atom::Float(f)) = &tail2[0] {
                    assert!(*f == 0.0);
                } else {
                    panic!();
                }
                // FLOAT
                if let Expr::Constant(Atom::Float(f)) = &tail2[1] {
                    assert!(*f == 400.0);
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }
}
