## android-bp

a rust crate to parse Android.bp files

### Usage

```rust
    use android_bp::Blueprint;
    let bp = Blueprint::from_file("Android.bp").unwrap();
    println!("{:#?}", bp);
```
