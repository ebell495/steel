use im_lists::list::List;

use crate::rvals::{Result, SteelVal};
use crate::stop;

macro_rules! ok_string {
    ($string:expr) => {
        Ok(SteelVal::StringV($string.into()))
    };
}

pub struct StringOperations {}
impl StringOperations {
    pub fn string_append() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() >= 2 {
                let mut arg_iter = args.iter();

                let first_arg = arg_iter.next().unwrap();

                let mut first = if let SteelVal::StringV(first) = first_arg {
                    first.to_string()
                } else {
                    stop!(TypeMismatch => format!("string-append expected a string, found: {first_arg}"))
                };

                for arg in arg_iter {
                    if let SteelVal::StringV(r) = arg {
                        first = first + r;
                    } else {
                        stop!(TypeMismatch => format!("string-append expected a string, found: {first_arg}"))
                    };
                }

                ok_string!(first)
            } else {
                stop!(ArityMismatch => "string-append takes at least two arguments")
            }
        })
    }

    pub fn string_to_symbol() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    Ok(SteelVal::SymbolV(s.clone()))
                } else {
                    stop!(TypeMismatch => "string->int expected a string")
                }
            } else {
                stop!(ArityMismatch => "string->int takes one argument")
            }
        })
    }

    pub fn int_to_string() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::IntV(s) = &args[0] {
                    Ok(SteelVal::StringV(format!("{s}").into()))
                } else {
                    stop!(TypeMismatch => "string->int expected a string")
                }
            } else {
                stop!(ArityMismatch => "string->int takes one argument")
            }
        })
    }

    pub fn string_to_int() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    let parsed_int = s.parse::<isize>();
                    match parsed_int {
                        Ok(n) => Ok(SteelVal::IntV(n)),
                        Err(_) => {
                            stop!(TypeMismatch => "could not convert number to integer");
                        }
                    }
                } else {
                    stop!(TypeMismatch => "string->int expected a string")
                }
            } else {
                stop!(ArityMismatch => "string->int takes one argument")
            }
        })
    }

    pub fn string_to_list() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    Ok(s.chars().map(SteelVal::CharV).collect::<List<_>>().into())
                } else {
                    stop!(TypeMismatch => "string->list expected a string")
                }
            } else {
                stop!(ArityMismatch => "string->list takes one argument")
            }
        })
    }

    pub fn string_to_upper() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    let upper = s.to_uppercase();
                    ok_string!(upper)
                // Ok(Gc::new(SteelVal::StringV(upper)))
                } else {
                    stop!(TypeMismatch => "string-upcase expected a string")
                }
            } else {
                stop!(ArityMismatch => "string-upcase takes one argument")
            }
        })
    }

    pub fn string_to_lower() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    let lower = s.to_lowercase();
                    ok_string!(lower)
                // Ok(Gc::new(SteelVal::StringV(lower)))
                } else {
                    stop!(TypeMismatch => "string-lowercase expected a string")
                }
            } else {
                stop!(ArityMismatch => "string-lowercase takes one argument")
            }
        })
    }

    pub fn trim() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    let trimmed = s.trim();
                    ok_string!(trimmed.to_string())
                // Ok(Gc::new(SteelVal::StringV(trimmed.to_string())))
                } else {
                    stop!(TypeMismatch => "trim expected a string")
                }
            } else {
                stop!(ArityMismatch => "trim takes one argument")
            }
        })
    }

    pub fn trim_start() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    let trimmed = s.trim_start();
                    ok_string!(trimmed.to_string())
                // Ok(Gc::new(SteelVal::StringV(trimmed.to_string())))
                } else {
                    stop!(TypeMismatch => "trim-start expected a string")
                }
            } else {
                stop!(ArityMismatch => "trim-start takes one argument")
            }
        })
    }

    pub fn trim_end() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    let trimmed = s.trim_end();
                    ok_string!(trimmed.to_string())
                // Ok(Gc::new(SteelVal::StringV(trimmed.to_string())))
                } else {
                    stop!(TypeMismatch => "trim-end expected a string")
                }
            } else {
                stop!(ArityMismatch => "trim-end takes one argument")
            }
        })
    }

    pub fn split_whitespace() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    let split: List<SteelVal> = s
                        .split_whitespace()
                        .map(|x| SteelVal::StringV(x.into()))
                        .collect();
                    Ok(split.into())
                } else {
                    stop!(TypeMismatch => "split-whitespace expected a string")
                }
            } else {
                stop!(ArityMismatch => "split-whitespace takes one argument")
            }
        })
    }

    pub fn string_length() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 1 {
                if let SteelVal::StringV(s) = &args[0] {
                    Ok(SteelVal::IntV(s.len() as isize))
                } else {
                    stop!(TypeMismatch => "string-length expected a string")
                }
            } else {
                stop!(ArityMismatch => "string-length takes one argument")
            }
        })
    }

    pub fn starts_with() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 2 {
                match (&args[0], &args[1]) {
                    (SteelVal::StringV(s), SteelVal::StringV(p)) => {
                        Ok(SteelVal::BoolV(s.starts_with(p.as_ref())))
                    }
                    _ => {
                        stop!(ArityMismatch => "starts-with? takes two arguments")
                    }
                }
            } else {
                stop!(ArityMismatch => "starts-with? takes two arguments")
            }
        })
    }

    pub fn ends_with() -> SteelVal {
        SteelVal::FuncV(|args: &[SteelVal]| -> Result<SteelVal> {
            if args.len() == 2 {
                match (&args[0], &args[1]) {
                    (SteelVal::StringV(s), SteelVal::StringV(p)) => {
                        Ok(SteelVal::BoolV(s.ends_with(p.as_ref())))
                    }
                    _ => {
                        stop!(ArityMismatch => "ends-with? takes two arguments")
                    }
                }
            } else {
                stop!(ArityMismatch => "ends-with? takes two arguments")
            }
        })
    }
}

#[cfg(test)]
mod string_operation_tests {
    use super::*;
    // use crate::gc::Gc;
    use crate::rerrs::ErrorKind;
    // use crate::rvals::ConsCell;
    use crate::throw;
    use im_lists::list;

    // TODO combine these 3 macros into one
    macro_rules! apply_tests_arity_too_many {
        ($(($name:expr, $symbol:ident, $func:expr)),* $(,)?) => {
            $(
                #[test]
                pub fn $symbol() {
                    let args = vec![
                        SteelVal::StringV("FOO".into()),
                        SteelVal::StringV("BAR".into()),
                    ];
                    let res = apply_function($func.clone(), args);
                    let expected = ErrorKind::ArityMismatch;
                    assert_eq!(res.unwrap_err().kind(), expected);
                }
            )*
        };
    }

    macro_rules! apply_tests_arity_too_few {
        ($(($name:expr, $symbol:ident, $func:expr)),* $(,)?) => {
            $(
                #[test]
                pub fn $symbol() {
                    let args = vec![];
                    let res = apply_function($func.clone(), args);
                    let expected = ErrorKind::ArityMismatch;
                    assert_eq!(res.unwrap_err().kind(), expected);
                }
            )*
        };
    }

    macro_rules! apply_tests_bad_arg {
        ($(($name:expr, $symbol:ident, $func:expr)),* $(,)?) => {
            $(
                #[test]
                pub fn $symbol() {
                    let args = vec![SteelVal::NumV(10.0)];
                    let res = apply_function($func.clone(), args);
                    let expected = ErrorKind::TypeMismatch;
                    assert_eq!(res.unwrap_err().kind(), expected);
                }
            )*
        };
    }

    apply_tests_arity_too_many! {
        ("string-upcase", string_upper_arity_too_many, StringOperations::string_to_upper()),
        ("string-lowercase", string_lower_arity_too_many, StringOperations::string_to_lower()),
        ("trim", trim_arity_too_many, StringOperations::trim()),
        ("trim-start", trim_start_arity_too_many, StringOperations::trim_start()),
        ("trim-end", trim_end_arity_too_many, StringOperations::trim_end()),
        ("string->list", string_to_list_arity_too_many, StringOperations::string_to_list()),
        ("split-whitespace", split_whitespace_arity_too_many, StringOperations::split_whitespace()),
    }

    apply_tests_arity_too_few! {
        ("string-upcase", string_upper_arity_too_few, StringOperations::string_to_upper()),
        ("string-lowercase", string_lower_arity_too_few, StringOperations::string_to_lower()),
        ("trim", trim_arity_too_few, StringOperations::trim()),
        ("trim-start", trim_start_arity_too_few, StringOperations::trim_start()),
        ("trim-end", trim_end_arity_too_few, StringOperations::trim_end()),
        ("string->list", string_to_list_arity_too_few, StringOperations::string_to_list()),
        ("split-whitespace", split_whitespace_arity_too_few, StringOperations::split_whitespace())
    }

    apply_tests_bad_arg! {
        ("string-upcase", string_upper_arity_takes_string, StringOperations::string_to_upper()),
        ("string-lowercase", string_lower_arity_takes_string, StringOperations::string_to_lower()),
        ("trim", trim_arity_takes_string, StringOperations::trim()),
        ("trim-start", trim_start_arity_takes_string, StringOperations::trim_start()),
        ("trim-end", trim_end_arity_takes_string, StringOperations::trim_end()),
        ("string->list", string_to_list_takes_string, StringOperations::string_to_list()),
        ("split-whitespace", split_whitespace_arity_takes_string, StringOperations::split_whitespace())
    }

    fn apply_function(func: SteelVal, args: Vec<SteelVal>) -> Result<SteelVal> {
        func.func_or_else(throw!(BadSyntax => "string tests"))
            .unwrap()(&args)
    }

    #[test]
    fn string_append_test_normal() {
        let args = vec![
            SteelVal::StringV("foo".into()),
            SteelVal::StringV("bar".into()),
        ];
        let res = apply_function(StringOperations::string_append(), args);
        let expected = SteelVal::StringV("foobar".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn string_append_test_arity_mismatch_too_few() {
        let args = vec![SteelVal::StringV("foo".into())];
        let res = apply_function(StringOperations::string_append(), args);
        let expected = ErrorKind::ArityMismatch;
        assert_eq!(res.unwrap_err().kind(), expected);
    }

    #[test]
    fn string_append_test_takes_string() {
        let args = vec![SteelVal::CharV('a'), SteelVal::CharV('b')];
        let res = apply_function(StringOperations::string_append(), args);
        let expected = ErrorKind::TypeMismatch;
        assert_eq!(res.unwrap_err().kind(), expected);
    }

    #[test]
    fn string_to_upper_normal() {
        let args = vec![SteelVal::StringV("foobarbaz".into())];
        let res = apply_function(StringOperations::string_to_upper(), args);
        let expected = SteelVal::StringV("FOOBARBAZ".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn string_to_upper_spaces() {
        let args = vec![SteelVal::StringV("foo bar baz qux".into())];
        let res = apply_function(StringOperations::string_to_upper(), args);
        let expected = SteelVal::StringV("FOO BAR BAZ QUX".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn string_to_lower_normal() {
        let args = vec![SteelVal::StringV("FOOBARBAZ".into())];
        let res = apply_function(StringOperations::string_to_lower(), args);
        let expected = SteelVal::StringV("foobarbaz".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn string_to_lower_spaces() {
        let args = vec![SteelVal::StringV("FOO BAR BAZ QUX".into())];
        let res = apply_function(StringOperations::string_to_lower(), args);
        let expected = SteelVal::StringV("foo bar baz qux".into());
        assert_eq!(res.unwrap(), expected);
    }

    // TODO investigate this, assert_eq! fails without converting to string
    #[test]
    fn string_to_list_normal() {
        let args = vec![SteelVal::StringV("foo".into())];
        let res = apply_function(StringOperations::string_to_list(), args);

        let expected = SteelVal::ListV(list![
            SteelVal::CharV('f'),
            SteelVal::CharV('o'),
            SteelVal::CharV('o')
        ]);

        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn string_to_list_empty() {
        let args = vec![SteelVal::StringV("".into())];
        let res = apply_function(StringOperations::string_to_list(), args);
        let expected = SteelVal::ListV(List::new());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn trim_normal_no_changes() {
        let args = vec![SteelVal::StringV("foo".into())];
        let res = apply_function(StringOperations::trim(), args);
        let expected = SteelVal::StringV("foo".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn trim_normal_trims_both_sides() {
        let args = vec![SteelVal::StringV("      foo  ".into())];
        let res = apply_function(StringOperations::trim(), args);
        let expected = SteelVal::StringV("foo".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn trim_start_no_changes() {
        let args = vec![SteelVal::StringV("foo".into())];
        let res = apply_function(StringOperations::trim_start(), args);
        let expected = SteelVal::StringV("foo".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn trim_end_no_changes() {
        let args = vec![SteelVal::StringV("foo".into())];
        let res = apply_function(StringOperations::trim_end(), args);
        let expected = SteelVal::StringV("foo".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn trim_start_normal_trims_left_side() {
        let args = vec![SteelVal::StringV("      foo  ".into())];
        let res = apply_function(StringOperations::trim_start(), args);
        let expected = SteelVal::StringV("foo  ".into());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn trim_end_normal_trims_right_side() {
        let args = vec![SteelVal::StringV("      foo  ".into())];
        let res = apply_function(StringOperations::trim_end(), args);
        let expected = SteelVal::StringV("      foo".into());
        assert_eq!(res.unwrap(), expected);
    }

    // TODO investigate this one
    #[test]
    fn split_whitespace_no_whitespace() {
        let args = vec![SteelVal::StringV("foo".into())];
        let res = apply_function(StringOperations::split_whitespace(), args);

        let expected = SteelVal::ListV(list![SteelVal::StringV("foo".into())]);

        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn split_whitespace_some_whitespace() {
        let args = vec![SteelVal::StringV("foo bar baz".into())];
        let res = apply_function(StringOperations::split_whitespace(), args);

        let expected = SteelVal::ListV(list![
            SteelVal::StringV("foo".into()),
            SteelVal::StringV("bar".into()),
            SteelVal::StringV("baz".into())
        ]);
        assert_eq!(res.unwrap(), expected);
    }
}
