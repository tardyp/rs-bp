use crate::{context_tag, end_delimiter, utils::*};
use nom::Err;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    error::{context, convert_error, VerboseError},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
};
use std::collections::HashMap;
use std::ops::{DerefMut, Deref};

/// a dictionary in a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Dict(pub HashMap<String, Value>);
impl Deref for Dict {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Dict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
fn parse_dict(input: &str) -> VerboseResult<Dict> {
    context(
        "dict",
        map(
            delimited(
                tuple((space_or_comments, context_tag!("{"), space_or_comments)),
                separated_list0(char(','), parse_block_entry),
                end_delimiter!("}"),
            ),
            |entries| Dict(entries.into_iter().collect())
        ),
    )(input)
}

/// a value in a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Value {
    String(String),
    Array(Vec<String>),
    Boolean(bool),
    Dict(Dict),
    Ident(String),
}
fn parse_value(input: &str) -> VerboseResult<Value> {
    context(
        "value",
        alt((
            map(parse_array, Value::Array),
            map(string_literal, Value::String),
            map(parse_bool, Value::Boolean),
            map(parse_dict, Value::Dict),
            map(identifier, |x| Value::Ident(x.to_string())),
        )),
    )(input)
}

/// a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct BluePrint {
    /// defines in the blueprint file 
    /// found in root of the file in the form of `key = value`
    pub defines: HashMap<String, Value>,
    /// all ordered blocks in the blueprint file
    pub blocks: Vec<Block>,
}


/// a block in a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Block {
    pub typ: String,
    pub entries: HashMap<String, Value>,
}

pub(crate) fn parse_block_entry(input: &str) -> VerboseResult<(String, Value)> {
    context(
        "block entry",
        map(
            tuple((
                space_or_comments,
                identifier,
                space_or_comments,
                char(':'),
                space_or_comments,
                cut(parse_value),
                space_or_comments,
            )),
            |(_, key, _, _, _, value, _)| (key.to_string(), value),
        ),
    )(input)
}

pub(crate) fn parse_block(input: &str) -> VerboseResult<Block> {
    // parse a identifier followed by a block of entries
    let (input, _) = space_or_comments(input)?;
    let (input, ident) = identifier(input)?;
    let (input, _) = space_or_comments(input)?;
    let (input, block) = context(
        "block",
        map(
            delimited(
                tuple((space_or_comments, context_tag!("{"), space_or_comments)),
                separated_list0(char(','), parse_block_entry),
                end_delimiter!("}"),
            ),
            |entries| entries.into_iter().collect(),
        ),
    )(input)?;
    Ok((
        input,
        Block {
            typ: ident.to_string(),
            entries: block,
        },
    ))
}

pub(crate) fn parse_define(input: &str) -> VerboseResult<(String, Value)> {
    context(
        "define",
        map(
            tuple((
                space_or_comments,
                identifier,
                space_or_comments,
                char('='),
                space_or_comments,
                cut(parse_value),
                space_or_comments,
            )),
            |(_, key, _, _, _, value, _)| (key.to_string(), value),
        ),
    )(input)
}
pub(crate) fn parse_blueprint(input: &str) -> VerboseResult<BluePrint> {
    let mut entries = Vec::new();
    let mut defines = HashMap::new();
    let (input, _) = context(
        "blueprint",
        many0(alt((
            map(parse_block, |b| {
                entries.push(b);
                ()
            }),
            map(parse_define, |(k, v)| {
                defines.insert(k, v);
                ()
            }),
        ))),
    )(input)?;
    Ok((
        input,
        BluePrint {
            defines: defines,
            blocks: entries,
        },
    ))
}

pub(crate)fn format_err(input: &str, err: Err<VerboseError<&str>>) -> String {
    match err {
        Err::Error(e) | Err::Failure(e) => convert_error(input, e.into()),
        Err::Incomplete(_) => "Incomplete".to_string(),
    }
}
impl BluePrint {
    pub fn parse(input: &str) -> Result<Self, String> {
        match parse_blueprint(input) {
            Ok((rest, result)) => {
                if rest.len() > 0 {
                    return Err(format!("Unexpected left input: {}", rest));
                }
                Ok(result)
            }
            Err(err) => Err(format_err(input, err)),
        }
    }
}
