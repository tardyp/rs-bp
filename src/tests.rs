
#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::parser::*;
    use nom::Err;
    use nom::error::VerboseError;

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
        assert_eq!(parse_block_entry(input), expected_output);

        // Test case 2: Valid input with whitespace
        let input = r#"  key  :   "value"  "#;
        let expected_output = Ok(("", ("key".to_string(), Value::String("value".to_string()))));
        assert_eq!(parse_block_entry(input), expected_output);

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
        assert_eq!(parse_block_entry(input), expected_output);

        // Test case 4: Invalid input - missing colon
        let input = r#"key "value""#;
        assert!(parse_block_entry(input).is_err());

        // Test case 5: Invalid input - missing value
        let input = r#"key:"#;
        assert!(parse_block_entry(input).is_err());

        // Test case 6: Invalid input - missing key
        let input = r#":"value""#;
        assert!(parse_block_entry(input).is_err());

        // Test case 7: Invalid input - missing key and value
        let input = r#":"#;
        assert!(parse_block_entry(input).is_err());
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
    fn test_nested_dict() {
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
    
    #[test]
    fn test_comment() {
        let input = r#"
        rust_test_host {
        //     name: "ss",
        //
        srcs: ["src/ss.rs"],
        test_options: {
            unit_test: true,
        },
        }        "#;
        let output = parse_block(input);
        display_error(input, &output);
        assert!(output.is_ok());
    }

    fn display_error(input: &str, output: &Result<(&str, Block), Err<VerboseError<&str>>>) -> () {
        if let Err(e) = output {
            println!("Error: {}", format_err(input, e.clone()));
        }
    }
}
