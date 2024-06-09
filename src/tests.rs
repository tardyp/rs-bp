#[cfg(test)]
mod tests {

    use std::io::Read;

    use crate::parser::*;
    use nom::error::VerboseError;
    use nom::Err;

    #[test]
    fn test_parse_array() {
        // Test case 1: Valid input
        let input = r#"[ "value1", "value2", "value3" ]"#;
        let expected_output = Ok(("", vec!["value1".into(), "value2".into(), "value3".into()]));
        assert_eq!(parse_array(input), expected_output);

        // Test case 2: Empty array
        let input = r#"[]"#;
        let expected_output = Ok(("", vec![]));
        assert_eq!(parse_array(input), expected_output);

        // Test case 3: Array with whitespace
        let input = r#"[ "value1" , "value2" , "value3" ]"#;
        let expected_output = Ok(("", vec!["value1".into(), "value2".into(), "value3".into()]));
        assert_eq!(parse_array(input), expected_output);

        // Test case 4: Array with empty values
        let input = r#"[ "", "", "" ]"#;
        let expected_output = Ok(("", vec!["".into(), "".into(), "".into()]));
        assert_eq!(parse_array(input), expected_output);

        // Test case 5: Invalid input - missing closing bracket
        let input = r#"[ "value1", "value2", "value3""#;
        assert!(parse_array(input).is_err());

        // Test case 5: Invalid input - missing opening bracket
        let input = r#""value1", "value2", "value3" ]"#;
        assert!(parse_array(input).is_err());

        // Test case 6: Array with trailing comma is not an error
        let input = r#"[ "value1", "value2", "value3", ]"#;
        let expected_output = Ok(("", vec!["value1".into(), "value2".into(), "value3".into()]));
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
                Value::Array(vec!["value1".into(), "value2".into(), "value3".into()]),
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
                    Value::Array(vec!["value2".into(), "value3".into()]),
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
    #[test]
    fn test_all_comment() {
        let input = r#"/*
        rust_test_host {
        //     name: "ss",
        //
        srcs: ["src/ss.rs"],
        test_options: {
            unit_test: true,
        },
        }        
        */"#;
        let output = BluePrint::parse(input);
        if output.is_err() {
            println!("Error: {}", output.unwrap_err());
            panic!("Error in parsing");
        }
    }

    #[test]
    fn test_issue_1() {
        let input = r#"
        aidl_interface {
            name: "android.hardware.tetheroffload",
            vendor_available: true,
            srcs: ["android/hardware/tetheroffload/*.aidl"],
            stability: "vintf",
            backend: {
                cpp: {
                    enabled: false,
                },
                java: {
                    sdk_version: "module_current",
                    apex_available: [
                        "com.android.tethering",
                    ],
                    min_sdk_version: "30",
                    enabled: true,
                },
                ndk: {
                    apps_enabled: false,
                },
            },
            versions_with_info: [
                {
                    version: "1",
                    imports: [],
                },
            ],
            frozen: true,
        
        }
        "#;
        let output = parse_module(input);
        display_error(input, &output);
        assert!(output.is_ok());
    }
    #[test]
    fn test_issue_2() {
        let input = r#"
        aidl_interface {
            name: "android.hardw\"are.tetheroffload",
        }
        "#;
        let output = parse_module(input);
        display_error(input, &output);
        assert!(output.is_ok());
    }
    #[test]
    fn test_module_second_form() {
        let input = r#"
        aidl_interface(name = "android.hardware.tetheroffload")
        "#;
        let output = parse_module(input);
        display_error(input, &output);
        assert!(output.is_ok());
    }
    fn display_error<T>(input: &str, output: &Result<(&str, T), Err<VerboseError<&str>>>) -> () {
        if let Err(e) = output {
            println!("Error: {}", format_err(input, e.clone()));
        }
    }
    #[test]
    fn test_expr() {
        let input = r#""abc" + "def""#;
        let output = parse_expr(input);
        display_error(input, &output);
        assert!(output.is_ok());
        assert!(output.as_ref().unwrap().0.is_empty());
        assert!(output.unwrap().1 == Value::String("abcdef".to_string()));
    }
    #[test]
    fn test_expr_array() {
        let input = r#"["abc", "def"] + [ "ghi" ]"#;
        let output = parse_expr(input);
        display_error(input, &output);
        assert!(output.is_ok());
        assert!(output.as_ref().unwrap().0.is_empty());
        assert!(output.unwrap().1 == Value::Array(vec!["abc".into(), "def".into(), "ghi".into()]));
    }
    #[test]
    fn test_expr_ident() {
        let input = r#"ident + [ "ghi" ]"#;
        let output = parse_expr(input);
        display_error(input, &output);
        assert!(output.is_ok());
        assert!(output.as_ref().unwrap().0.is_empty());
        assert!(
            output.unwrap().1
                == Value::ConcatExpr([
                    Value::Ident("ident".to_string()),
                    Value::Array(["ghi".into()].into())
                ].into())
        );
    }
    #[test]
    fn test_expr_value() {
        let input = r#"123"#;
        let output = parse_expr(input);
        display_error(input, &output);
        assert!(output.is_ok());
        assert!(output.as_ref().unwrap().0.is_empty());
        assert!(
            output.unwrap().1
                == Value::Integer(123));
    }
    // found in platform_testing/tests/health/scenarios/tests/Android.bp
    #[test]
    fn test_complicated_concat() {
        let input = r#""out_dir=$$(dirname $(out)) && assets_dir=\"assets\" " +
        "&& mkdir -p $$out_dir/$$assets_dir && src_protos=($(locations assets/*.textpb)) " +
        "&& for file in $${src_protos[@]} ; do fname=$$(basename $$file) " +
        "&& if ! ($(location aprotoc) --encode=longevity.profile.Configuration " +
        "$(location :profile-proto-def) < $$file > " +
        "$$out_dir/$$assets_dir/$${fname//.textpb/.pb}) ; then " +
        "echo \"\x1b[0;31mFailed to parse profile $$file. See above for errors.\x1b[0m\" " +
        "&& exit 1 ; fi ; done && jar cf $(out) -C $$(dirname $(out)) $$assets_dir""#;
        let output = parse_expr(input);
        display_error(input, &output);
        assert!(output.is_ok());
        assert!(output.as_ref().unwrap().0.is_empty());

    }
    #[test]
    fn test_linecomment_wo_eol() {
        let input = r#"// foo"#;
        let output = BluePrint::parse(input);
        assert!(output.is_ok());

    }
    #[test]
    fn test_defines_extends(){
        let input = r#"
        var = ["a", "b"]
        var2 = 12
        var += ["c"]
        var2 += 1
        var3 = "abc"
        var3 += "def"
        "#;
        let output = BluePrint::parse(input);
        assert!(output.is_ok());
        let bp = output.unwrap();
        assert_eq!(bp.variables.get("var").unwrap(), &Value::Array(vec!["a".into(), "b".into(), "c".into()]));
        assert_eq!(bp.variables.get("var2").unwrap(), &Value::Integer(13));
        assert_eq!(bp.variables.get("var3").unwrap(), &Value::String("abcdef".to_string()));
    }

    #[test]
    fn test_defines_extends_error(){
        let input = r#"
        var = ["a", "b"]
        var2 = 12
        var += 1
        var2 += "a"
        "#;
        let output = BluePrint::parse(input);
        println!("Error: {}", output.unwrap_err());
        // assert!(output.is_err());
    }
    #[test]
    fn test_function() {
        let input = r#"method("ss")"#;
        let output = parse_expr(input);
        display_error(input, &output);
        assert!(output.is_ok());
        assert_eq!(output.unwrap().1, Value::Function(Function {
            name: "method".to_string(),
            args: vec![Value::String("ss".to_string())]
        }));

    }
    #[test]
    fn test_aosp_db() {
        // generate tarball from aosp tree
        // fd -g Android.bp | tar cJf ../rs-bp/src/test_db.tar.xz -T -
        let data = include_bytes!("test_db.tar.xz");
        let mut archive = tar::Archive::new(liblzma::read::XzDecoder::new(&data[..]));
        let mut count = 0;
        let mut bytes = 0;
        let mut num_errors = 0;
        let mut all_bp = Vec::new();
        // first decompress in memory to avoid disk IO for measuring performance
        for entry in archive.entries().unwrap() {
            let entry = entry.unwrap();
            let mut entry_data = std::io::BufReader::new(entry);
            let mut contents = String::new();
            entry_data.read_to_string(&mut contents).unwrap();
            bytes += contents.len();
            all_bp.push((format!("{:?}", entry_data.into_inner().path().unwrap()), contents));
        }
        let now = std::time::Instant::now();
        for (path, contents) in all_bp {
            let output = BluePrint::parse(&contents);
            if output.is_err() {
                println!("Error for file: {:?}", path);
                println!("File content: {}", contents);
                println!("Error: {}", output.unwrap_err());
                num_errors += 1;
            }
            count += 1;
        }
        let elapsed = now.elapsed().as_secs_f32();
        println!("{} files ({} bytes) parsed in {:.3}s {}MB/s", count, bytes, elapsed, bytes as f32 / elapsed / 1024.0 / 1024.0);
        assert_eq!(num_errors, 0);
    }
}
