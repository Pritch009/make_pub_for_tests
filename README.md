# `make_pub_for_tests` Macro Guide

## Overview

The `make_pub_for_tests` macro is designed to modify the visibility of items (such as structs, enums, functions, etc.) so that they are publicly visible during test compilation but remain at their original visibility level in non-test configurations. This can be particularly useful when you want certain internal details to be accessible only for testing purposes.

## Usage

### Applying the Macro

To use the `make_pub_for_tests` macro, simply add the attribute to any item that you want to modify:

```rust
#[make_pub_for_tests]
pub struct MyStruct {
    pub field1: i32,
    private_field: String,
}
```

In this example, `MyStruct` and its fields will be publicly visible during test compilation but will retain their original visibility (in this case, `private`) in non-test configurations.

### Parameters

The macro accepts an optional attribute that specifies the desired public visibility. If no attribute is provided, it defaults to `pub(crate)`:

```rust
#[make_pub_for_tests("pub")]
struct MyStruct {
    field1: i32,
}
```

In this example, `MyStruct` will be publicly visible both during test and non-test compilations.

### Supported Items

The macro supports the following items:
- Structs (`Item::Struct`)
- Enums (`Item::Enum`)
- Functions (`Item::Fn`)
- Modules (`Item::Mod`)
- Traits (`Item::Trait`)
- Trait aliases (`Item::TraitAlias`)
- Types (`Item::Type`)

## Example Usage

Here is a more detailed example demonstrating the use of `make_pub_for_tests` with various item types:

```rust
#[make_pub_for_tests]
pub struct MyStruct {
    pub field1: i32,
    private_field: String,
}

#[make_pub_for_tests(crate)]
enum MyEnum {
    Variant1,
    Variant2,
}

#[make_pub_for_tests]
fn my_function() {
    // Function implementation
}
```

In this example, `MyStruct`, `MyEnum`, and `my_function` will have their visibility modified as specified.

## Notes

- The macro does not modify the original item but instead generates a new item with the desired visibility.
- If the macro is applied to an unsupported item type, it will generate a compilation error indicating that visibility modification is not supported for that type.

By following these guidelines, you can effectively use the `make_pub_for_tests` macro to enhance testability in your Rust projects.