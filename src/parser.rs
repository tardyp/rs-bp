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
use std::path::Path;

/// a dictionary in a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Map(pub HashMap<String, Value>);
impl Deref for Map {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
fn parse_dict(input: &str) -> VerboseResult<Map> {
    context(
        "dict",
        map(
            delimited(
                tuple((space_or_comments, context_tag!("{"), space_or_comments)),
                separated_list0(char(','), parse_module_entry),
                end_delimiter!("}"),
            ),
            |entries| Map(entries.into_iter().collect())
        ),
    )(input)
}

/// a value in a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Value {
    String(String),
    Array(Vec<String>),
    Boolean(bool),
    Map(Map),
    Ident(String),
}
fn parse_value(input: &str) -> VerboseResult<Value> {
    context(
        "value",
        alt((
            map(parse_array, Value::Array),
            map(string_literal, Value::String),
            map(parse_bool, Value::Boolean),
            map(parse_dict, Value::Map),
            map(identifier, |x| Value::Ident(x.to_string())),
        )),
    )(input)
}

/// a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct BluePrint {
    /// variables in the blueprint file 
    /// found in root of the file in the form of `key = value`
    pub variables: HashMap<String, Value>,
    /// all ordered modules in the blueprint file
    pub modules: Vec<Module>,
}


/// a module in a blueprint file
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Module {
    pub typ: String,
    pub entries: HashMap<String, Value>,
}
impl Module {
    /// get an attribute value from a module
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key)
    }
    /// get a string attribute value from a module
    pub fn get_string(&self, key: &str) -> Option<&String> {
        match self.get(key) {
            Some(Value::String(s)) => Some(s),
            _ => None,
        }
    }
    /// get a boolean attribute value from a module
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get(key) {
            Some(Value::Boolean(b)) => Some(*b),
            _ => None,
        }
    }
    /// get an array attribute value from a module
    pub fn get_array(&self, key: &str) -> Option<&Vec<String>> {
        match self.get(key) {
            Some(Value::Array(a)) => Some(a),
            _ => None,
        }
    }
    /// get a map attribute value from a module
    pub fn get_map(&self, key: &str) -> Option<&Map> {
        match self.get(key) {
            Some(Value::Map(d)) => Some(d),
            _ => None,
        }
    }
    /// get an identifier attribute value from a module
    pub fn get_ident(&self, key: &str) -> Option<&String> {
        match self.get(key) {
            Some(Value::Ident(i)) => Some(i),
            _ => None,
        }
    }

}
pub(crate) fn parse_module_entry(input: &str) -> VerboseResult<(String, Value)> {
    context(
        "module entry",
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

pub(crate) fn parse_module(input: &str) -> VerboseResult<Module> {
    // parse a identifier followed by a module of entries
    let (input, _) = space_or_comments(input)?;
    let (input, ident) = identifier(input)?;
    let (input, _) = space_or_comments(input)?;
    let (input, module) = context(
        "module",
        map(
            delimited(
                tuple((space_or_comments, context_tag!("{"), space_or_comments)),
                separated_list0(char(','), parse_module_entry),
                end_delimiter!("}"),
            ),
            |entries| entries.into_iter().collect(),
        ),
    )(input)?;
    Ok((
        input,
        Module {
            typ: ident.to_string(),
            entries: module,
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
    let mut variables = HashMap::new();
    let (input, _) = context(
        "blueprint",
        many0(alt((
            map(parse_module, |b| {
                entries.push(b);
                ()
            }),
            map(parse_define, |(k, v)| {
                variables.insert(k, v);
                ()
            }),
        ))),
    )(input)?;
    Ok((
        input,
        BluePrint {
            variables: variables,
            modules: entries,
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
    /// parse an Android.bp file from a string
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
    /// parse an Android.bp file from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let input = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::parse(&input)
    }
    /// get all modules of a specific type
    pub fn modules_by_type<'a>(&'a self, typ: &'static str) -> impl Iterator<Item = &'a Module> {
        self.modules
            .iter()
            .filter(move |b| b.typ == typ)
    }
}
