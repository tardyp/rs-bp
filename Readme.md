## android-bp

a rust crate to parse Android.bp files

### Usage

```rust
    use android_bp::BluePrint;

    let bp = BluePrint::from_file("fixtures/example.bp").unwrap();
    println!("{:#?}", bp);

    // variables are accessible as a rust HashMap
    println!("{:#?}", bp.variables);
    for m in &bp.modules {
        if m.typ == "rust_binary" {
            println!("{:?}", m.get("name").unwrap());
        }
    }
    // or iter them by type
    for m in bp.modules_by_type("rust_host_test") {
        // m.get return an sometime inconvenient Option<&Value>
        // so some helper methods are provided
        let name = m.get_string("name").unwrap();
        let srcs = m.get_array("srcs").unwrap();
        println!("{:?} {:?}", name, srcs);
    }
```

### Status

- [x] The project parses successfully all the Android.bp files in the AOSP tree.
      Test files are present in the src/test_db.tar.xz archive.

- [x] different possible values are abstracted in the `Value` enum
    - [x] strings
    - [x] arrays
    - [x] integers
    - [x] booleans
    - [x] expressions
    - [x] functions
    - [x] identifiers

- [x] modules (`module { ... }`)

- [x] variables (`var = "value"`)

- [x] variables extend (`var += [ "new value" ]`)
    - [x] arrays
    - [x] strings
    - [x] integers
    - [ ] expressions with other variables

- [x] expressions (`var : "value" + \n"value"`), used for strings long enough to be split in multiple lines
    - [x] arrays (automatically merged)
    - [x] strings (automatically merged)
    - [x] with identifiers (kept as an expression)
