use nom::{
    IResult,
    character::complete::char,
    combinator::{cut, map},
    error::{context},
    multi::separated_list,
    sequence::{ preceded, separated_pair, terminated}
};

use crate::idl::common::{
    parse_identifier,
    parse_field_separator,
    trailing_comma,
    ws,
};
use crate::idl::r#value::{
    Value,
    parse_value,
};

#[derive(Debug, PartialEq)]
pub struct FieldOption {
    pub name: String,
    pub value: Value,
}

pub fn parse_field_options(input: &str) -> IResult<&str, Vec<FieldOption>> {
    context(
        "options",
        preceded(
            preceded(ws, char('(')),
            cut(terminated(
                separated_list(parse_field_separator, parse_field_option),
                preceded(trailing_comma, preceded(ws, char(')')))
            ))
        )
    )(input)
}

fn parse_field_option(input: &str) -> IResult<&str, FieldOption> {
    map(
        separated_pair(
            preceded(ws, parse_identifier),
            preceded(ws, char('=')),
            preceded(ws, parse_value),
        ),
        |(name, value)| FieldOption {
            name: name,
            value: value
        }
    )(input)
}

#[test]
fn test_parse_field_options_0() {
    let contents = [
        "()",
        "( )",
        "(,)",
        "( ,)",
        "(, )",
    ];
    for content in contents.iter() {
        assert_eq!(
            parse_field_options(content),
            Ok(("", vec![]))
        );
    }
}

#[test]
fn test_parse_field_options_1() {
    let contents = [
        "(foo=42)",
        "(foo= 42)",
        "(foo=42 )",
        "( foo=42)",
        "(foo=42,)"
    ];
    for content in contents.iter() {
        assert_eq!(
            parse_field_options(content),
            Ok(("", vec![FieldOption {
                name: "foo".to_owned(),
                value: Value::Integer(42)
            }]))
        );
    }
}

#[test]
fn test_parse_field_options_2() {
    let contents = [
        "(foo=42,bar=\"epic\")",
        "(foo= 42, bar= \"epic\")",
        "( foo=42,bar=\"epic\" )",
        "( foo= 42, bar= \"epic\" )",
        "( foo= 42, bar= \"epic\", )",
    ];
    for content in contents.iter() {
        assert_eq!(
            parse_field_options(content),
            Ok(("", vec![
                FieldOption {
                    name: "foo".to_owned(),
                    value: Value::Integer(42)
                },
                FieldOption {
                    name: "bar".to_owned(),
                    value: Value::String("epic".to_string())
                }
            ]))
        );
    }
}