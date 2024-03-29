
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
        assert_eq!(parse_module_entry(input), expected_output);

        // Test case 2: Valid input with whitespace
        let input = r#"  key  :   "value"  "#;
        let expected_output = Ok(("", ("key".to_string(), Value::String("value".to_string()))));
        assert_eq!(parse_module_entry(input), expected_output);

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
        assert_eq!(parse_module_entry(input), expected_output);

        // Test case 4: Invalid input - missing colon
        let input = r#"key "value""#;
        assert!(parse_module_entry(input).is_err());

        // Test case 5: Invalid input - missing value
        let input = r#"key:"#;
        assert!(parse_module_entry(input).is_err());

        // Test case 6: Invalid input - missing key
        let input = r#":"value""#;
        assert!(parse_module_entry(input).is_err());

        // Test case 7: Invalid input - missing key and value
        let input = r#":"#;
        assert!(parse_module_entry(input).is_err());
    }
    #[test]
    fn test_parse_module() {
        let input = r#"
            module_name {
                key1: "value1",
                key2: true,
                key3: [ "value2", "value3" ],
            }
        "#;

        let expected_output = Module {
            typ: "module_name".to_string(),
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

        assert_eq!(parse_module(input), Ok(("", expected_output)));
    }
    #[test]
    fn test_parse_blueprint() {
        let input = r#"
            module_name {
                key1: "value1",
                key2: true,
                key3: [ "value2", "value3" ],
            }
            module_name2 {
                key1: "value1",
                key2: true,
                key3: [ "value2", "value3" ],
            }"#;
        let output = BluePrint::parse(input).unwrap();
        assert_eq!(output.modules.len(), 2);
        assert_eq!(output.modules[0].typ, "module_name");
        assert_eq!(output.modules[1].typ, "module_name2");
        let mut keys = output.modules[0]
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
        let output: Result<(&str, Module), Err<VerboseError<&str>>> = parse_module(input);
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
        let output = parse_module(input);
        display_error(input, &output);
        assert!(output.is_ok());
    }

    fn display_error(input: &str, output: &Result<(&str, Module), Err<VerboseError<&str>>>) -> () {
        if let Err(e) = output {
            println!("Error: {}", format_err(input, e.clone()));
        }
    }
}
