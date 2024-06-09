use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alpha1, alphanumeric1, digit1, multispace1},
    combinator::{map, map_res, opt, recognize, value},
    error::{context, VerboseError},
    multi::{many0, many0_count, many1},
    sequence::{delimited, pair, tuple},
    IResult, Parser,
};
use crate::string::parse_string;

/// Result type with verbose error
pub(crate) type VerboseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub(crate) fn comment(input: &str) -> VerboseResult<()> {
    context(
        "comment",
        value(
            (),
            tuple((tag("//"), take_while(|c|c!='\n'))),
        ),
    )(input)
}

pub(crate) fn multiline_comment(input: &str) -> VerboseResult<()> {
    context(
        "multiline comment",
        value((), delimited(tag("/*"), take_until("*/"), tag("*/"))),
    )(input)
}

pub(crate) fn space_or_comments(input: &str) -> VerboseResult<()> {
    value(
        (),
        many0(alt((value((), multispace1), comment, multiline_comment))),
    )(input)
}
pub(crate) fn space_or_comments1(input: &str) -> VerboseResult<()> {
    value(
        (),
        many1(alt((value((), multispace1), comment, multiline_comment))),
    )(input)
}

pub(crate)fn ws<'a, F, O>(inner: F) -> impl Parser<&'a str, O, VerboseError<&'a str>>
    where
    F: Parser<&'a str, O, VerboseError<&'a str>>,
{
    delimited(
        space_or_comments,
        inner,
        space_or_comments
    )
}
pub(crate) fn identifier(input: &str) -> VerboseResult<&str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub(crate) fn string_literal(input: &str) -> VerboseResult<String> {
    context(
        "string",
        parse_string
    )(input)
}

pub(crate) fn comma(input: &str) -> VerboseResult<&str> {
    ws(tag(",")).parse(input)
}

pub(crate) fn parse_bool(input: &str) -> VerboseResult<bool> {
    alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(input)
}

pub(crate) fn parse_int(input: &str) -> VerboseResult<i64> {
    map_res(
        recognize(pair(opt(tag("-")), digit1)),
        |x| i64::from_str_radix(x, 10),
    )(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bool() {
        let input = "true";
        let expected_output = Ok(("", true));
        assert_eq!(parse_bool(input), expected_output);
    }
    #[test]
    fn test_parse_int() {
        let input = "123";
        let expected_output = Ok(("", 123));
        assert_eq!(parse_int(input), expected_output);
    }
    #[test]
    fn test_parse_nint() {
        let input: &str = "-123";
        let expected_output = Ok(("", -123));
        assert_eq!(parse_int(input), expected_output);
    }
}

