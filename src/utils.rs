use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, multispace1},
    combinator::{map, recognize, value},
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
            tuple((tag("//"), take_until("\n"))),
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
