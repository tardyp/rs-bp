## android-bp

A python module to parse Android.bp files (wrapper for android_bp rust module)

## Non-Goals

This module is only intended to parse Android.bp files, not to generate or rewrite them.

## Usage

```python
    from android_bp import Blueprint

    # bp is a rust object, but behave mostly like a read only python object
    bp = Blueprint.from_file("Android.bp")

    # for debug, you can print any of internal objects
    # they will be printed as rust would in debug fmt
    print(bp)

    # internal variables are accessible as a python dict
    print(bp.variables)

    # modules are accessible as a python list
    for m in bp.modules:
        if m.__type__ == "rust_binary":
            # module properties can be accessed directly as python attributes
            print(m.name)

    # or iter them by type
    for m in bp.modules_by_type("rust_host_test"):
        # or via __dict__
        print(m.__dict__["name"])

        # for convenience, unknown properties return None (not an AttributeError)
        print(m.unknown_attribute) # prints None

        # map properties are accessible as python dicts
        print(m.test_options['unit_test'])
```
