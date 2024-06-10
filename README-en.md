# mod

English| [简体中文](README.md)

## Introduction

`mod` is a Rust project that provides two simple and useful utility functions for converting between `Option` and `Result` types. The project includes two main functions:

1. `option_to_result<T, E>`: Converts an `Option<T>` to a `Result<T, E>`.
2. `result_to_option<T, E>`: Converts a `Result<T, E>` to an `Option<T>`.

## Installation

You can add this project to your Rust project by running the following command:

```sh
cargo add mod
```

## Usage

First, ensure that you include the `mod` crate in your project:

```rust
extern crate mod;

use mod::{option_to_result, result_to_option};
```

### Function Descriptions

#### `option_to_result`

Converts an `Option<T>` to a `Result<T, E>`. If the `Option` is `Some`, it returns the value wrapped in `Ok`; if it is `None`, it returns the specified error.

```rust
pub fn option_to_result<T, E>(option: Option<T>, err: E) -> Result<T, E>
where
    E: Clone,
{
    match option {
        Some(value) => Ok(value),
        None => Err(err),
    }
}
```

**Example**:

```rust
let some_value = Some(42);
let none_value: Option<i32> = None;
let error = "Error";

assert_eq!(option_to_result(some_value, error.to_string()), Ok(42));
assert_eq!(option_to_result(none_value, error.to_string()), Err("Error".to_string()));
```

#### `result_to_option`

Converts a `Result<T, E>` to an `Option<T>`. If the `Result` is `Ok`, it returns the value wrapped in `Some`; if it is `Err`, it returns `None`.

```rust
pub fn result_to_option<T, E>(result: Result<T, E>) -> Option<T>
where
    E: Clone,
{
    match result {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
```

**Example**:

```rust
let ok_result = Ok::<i32, &str>(42);
let err_result = Err::<i32, &str>("Error");

assert_eq!(result_to_option(ok_result), Some(42));
assert_eq!(result_to_option(err_result), None);
```

## Testing

This project includes some unit tests to verify the correctness of the functionality. You can run the tests by executing the following command:

```sh
cargo test
```

The test cases are located in the `tests` module and cover the `option_to_result` and `result_to_option` functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_to_result() {
        let some_value = Some(42);
        let none_value: Option<i32> = None;
        let error = "Error";

        assert_eq!(option_to_result(some_value, error.to_string()), Ok(42));
        assert_eq!(option_to_result(none_value, error.to_string()), Err("Error".to_string()));
    }

    #[test]
    fn test_result_to_option() {
        let ok_result = Ok::<i32, &str>(42);
        let err_result = Err::<i32, &str>("Error");

        assert_eq!(result_to_option(ok_result), Some(42));
        assert_eq!(result_to_option(err_result), None);
    }
}
```

## License

This project is licensed under the GPL license. See the LICENSE file for details.

## Contribution

Contributions are welcome! Please submit issues or pull requests.

---

Thank you for using `mod`! If you have any questions or suggestions, feel free to contact us.