use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, multispace0, multispace1},
    combinator::{cut, map, opt},
    error::{context, convert_error, VerboseError},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashMap;

// define macro ttag with additional context
macro_rules! ttag {
    ($tag:expr) => {
        context(
            $tag,
            delimited(space_or_comments, tag($tag), space_or_comments),
        )
    };
}
// define macro ending delimiter with optional comma
macro_rules! end_delimiter {
    ($tag:expr) => {
        tuple((
            space_or_comments,
            opt(char(',')),
            space_or_comments,
            cut(tag($tag)),
            space_or_comments,
        ))
    };
}
// define Verbose Result
type VerboseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

fn comment(input: &str) -> VerboseResult<&str> {
    context(
        "comment",
        map(
            delimited(
                tag("//"),
                take_while(|c| c != '\n'),
                opt(tuple((char('\n'), multispace0))),
            ),
            |_| "",
        ),
    )(input)
}
fn multiline_comment(input: &str) -> VerboseResult<&str> {
    context(
        "multiline comment",
        map(delimited(tag("/*"), take_until("*/"), tag("*/")), |_| ""),
    )(input)
}
fn space_or_comments(input: &str) -> VerboseResult<&str> {
    map(
        many0(alt((multispace1, comment, multiline_comment))),
        |_| "",
    )(input)
}

fn is_alphanumeric(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn parse_key(input: &str) -> VerboseResult<&str> {
    take_while1(is_alphanumeric)(input)
}

fn parse_string(input: &str) -> VerboseResult<String> {
    context(
        "string",
        map(
            delimited(char('"'), take_while(|c| c != '"'), char('"')),
            |s: &str| s.to_string(),
        ),
    )(input)
}
fn comma_space_or_comments(input: &str) -> VerboseResult<&str> {
    delimited(space_or_comments, tag(","), space_or_comments)(input)
}
fn parse_array(input: &str) -> VerboseResult<Vec<String>> {
    context(
        "array",
        delimited(
            tuple((space_or_comments, char('['), space_or_comments)),
            separated_list0(comma_space_or_comments, parse_string),
            end_delimiter!("]"),
        ),
    )(input)
}
fn parse_bool(input: &str) -> VerboseResult<bool> {
    alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(input)
}

fn parse_value(input: &str) -> VerboseResult<Value> {
    context(
        "value",
        alt((
            map(parse_array, Value::Array),
            map(parse_string, Value::String),
            map(parse_bool, Value::Boolean),
            map(parse_dict, Value::Dict),
            map(parse_key, |x| Value::Ident(x.to_string())),
        )),
    )(input)
}
fn parse_entry(input: &str) -> VerboseResult<(String, Value)> {
    context(
        "block entry",
        map(
            tuple((
                space_or_comments,
                parse_key,
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

fn parse_block(input: &str) -> VerboseResult<Block> {
    // parse a identifier followed by a block of entries
    let (input, _) = space_or_comments(input)?;
    let (input, ident) = parse_key(input)?;
    let (input, _) = space_or_comments(input)?;
    let (input, block) = context(
        "block",
        map(
            delimited(
                tuple((space_or_comments, ttag!("{"), space_or_comments)),
                separated_list0(char(','), parse_entry),
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

fn parse_dict(input: &str) -> VerboseResult<Dict> {
    context(
        "dict",
        map(
            delimited(
                tuple((space_or_comments, ttag!("{"), space_or_comments)),
                separated_list0(char(','), parse_entry),
                end_delimiter!("}"),
            ),
            |entries| Dict {
                entries: entries.into_iter().collect(),
            },
        ),
    )(input)
}
fn parse_define(input: &str) -> VerboseResult<(String, Value)> {
    context(
        "define",
        map(
            tuple((
                space_or_comments,
                parse_key,
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
fn parse_blueprint(input: &str) -> VerboseResult<BluePrint> {
    let mut entries = Vec::new();
    let mut defines = HashMap::new();
    let (input, _) = context(
        "blueprint",
        many0(alt((
            map(parse_block, |b| {entries.push(b);()}),
            map(parse_define, |(k, v)| {defines.insert(k, v);()}),
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

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Value {
    String(String),
    Array(Vec<String>),
    Boolean(bool),
    Dict(Dict),
    Ident(String),
}
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Block {
    pub typ: String,
    pub entries: HashMap<String, Value>,
}
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Dict {
    pub entries: HashMap<String, Value>,
}
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct BluePrint {
    defines: HashMap<String, Value>,
    blocks: Vec<Block>,
}
use nom::Err;
fn format_err(input: &str, err: Err<VerboseError<&str>>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_array() {
        // Test case 1: Valid input
        let input = r#"[ "value1", "value2", "value3" ]"#;
        let expected_output = Ok((
            "",
            vec![
                "value1".to_string(),
                "value2".to_string(),
                "value3".to_string(),
            ],
        ));
        assert_eq!(parse_array(input), expected_output);

        // Test case 2: Empty array
        let input = r#"[]"#;
        let expected_output = Ok(("", vec![]));
        assert_eq!(parse_array(input), expected_output);

        // Test case 3: Array with whitespace
        let input = r#"[ "value1" , "value2" , "value3" ]"#;
        let expected_output = Ok((
            "",
            vec![
                "value1".to_string(),
                "value2".to_string(),
                "value3".to_string(),
            ],
        ));
        assert_eq!(parse_array(input), expected_output);

        // Test case 4: Array with empty values
        let input = r#"[ "", "", "" ]"#;
        let expected_output = Ok(("", vec!["".to_string(), "".to_string(), "".to_string()]));
        assert_eq!(parse_array(input), expected_output);

        // Test case 5: Array with mixed types
        let input = r#"[ "value1", 2, true ]"#;
        assert!(parse_array(input).is_err());

        // Test case 6: Invalid input - missing closing bracket
        let input = r#"[ "value1", "value2", "value3""#;
        assert!(parse_array(input).is_err());

        // Test case 7: Invalid input - missing opening bracket
        let input = r#""value1", "value2", "value3" ]"#;
        assert!(parse_array(input).is_err());

        // Test case 8: Array with trailing comma is not an error
        let input = r#"[ "value1", "value2", "value3", ]"#;
        let expected_output = Ok((
            "",
            vec![
                "value1".to_string(),
                "value2".to_string(),
                "value3".to_string(),
            ],
        ));
        assert_eq!(parse_array(input), expected_output);
    }
    #[test]
    fn test_parse_entry() {
        // Test case 1: Valid input
        let input = r#"key: "value""#;
        let expected_output = Ok(("", ("key".to_string(), Value::String("value".to_string()))));
        assert_eq!(parse_entry(input), expected_output);

        // Test case 2: Valid input with whitespace
        let input = r#"  key  :   "value"  "#;
        let expected_output = Ok(("", ("key".to_string(), Value::String("value".to_string()))));
        assert_eq!(parse_entry(input), expected_output);

        // Test case 3: Valid input with array value
        let input = r#"key: [ "value1", "value2", "value3" ]"#;
        let expected_output = Ok((
            "",
            (
                "key".to_string(),
                Value::Array(vec![
                    "value1".to_string(),
                    "value2".to_string(),
                    "value3".to_string(),
                ]),
            ),
        ));
        assert_eq!(parse_entry(input), expected_output);

        // Test case 4: Invalid input - missing colon
        let input = r#"key "value""#;
        assert!(parse_entry(input).is_err());

        // Test case 5: Invalid input - missing value
        let input = r#"key:"#;
        assert!(parse_entry(input).is_err());

        // Test case 6: Invalid input - missing key
        let input = r#":"value""#;
        assert!(parse_entry(input).is_err());

        // Test case 7: Invalid input - missing key and value
        let input = r#":"#;
        assert!(parse_entry(input).is_err());
    }
    #[test]
    fn test_parse_block() {
        let input = r#"
            block_name {
                key1: "value1",
                key2: true,
                key3: [ "value2", "value3" ],
            }
        "#;

        let expected_output = Block {
            typ: "block_name".to_string(),
            entries: vec![
                ("key1".to_string(), Value::String("value1".to_string())),
                ("key2".to_string(), Value::Boolean(true)),
                (
                    "key3".to_string(),
                    Value::Array(vec!["value2".to_string(), "value3".to_string()]),
                ),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(parse_block(input), Ok(("", expected_output)));
    }
    #[test]
    fn test_parse_blueprint() {
        let input = r#"
            block_name {
                key1: "value1",
                key2: true,
                key3: [ "value2", "value3" ],
            }
            block_name2 {
                key1: "value1",
                key2: true,
                key3: [ "value2", "value3" ],
            }"#;
        let output = BluePrint::parse(input).unwrap();
        assert_eq!(output.blocks.len(), 2);
        assert_eq!(output.blocks[0].typ, "block_name");
        assert_eq!(output.blocks[1].typ, "block_name2");
        let mut keys = output.blocks[0]
            .entries
            .keys()
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        keys.sort();
        assert_eq!(
            keys,
            vec!["key1".to_string(), "key2".to_string(), "key3".to_string()]
        );
    }
    #[test]
    fn test_parse_blueprint_error() {
        let input = r#"
        rust_test_host {
            name: "ss",
            srcs: ["src/ss.rs"],
            test_options: {
                unit_test: true,
            },
        }        "#;
        let output: Result<(&str, Block), Err<VerboseError<&str>>> = parse_block(input);
        assert!(output.is_ok());
    }
}
