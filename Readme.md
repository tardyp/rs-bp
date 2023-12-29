## android-bp

a rust crate to parse Android.bp files

### Usage

```rust
    use android_bp::BluePrint;

    let bp = BluePrint::from_file("fixtures/Android.bp").unwrap();
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
