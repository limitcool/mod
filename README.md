# mod

[English](README-en.md) | 简体中文

## 简介

`mod` 是一个 Rust 项目，提供了两个简单而实用的工具函数，用于在 `Option` 和 `Result` 类型之间进行转换。这个项目包含两个主要函数：

1. `option_to_result<T, E>`: 将 `Option<T>` 转换为 `Result<T, E>`。
2. `result_to_option<T, E>`: 将 `Result<T, E>` 转换为 `Option<T>`。

## 安装

您可以通过运行以下命令将此项目添加到您的 Rust 项目中：

```sh
cargo add mod
```

## 用法

首先，确保在您的项目中引入了 `mod` 包：

```rust
use r#mod::{option_to_result, result_to_option};
```

### 函数说明

#### `option_to_result`

将 `Option<T>` 转换为 `Result<T, E>`。如果 `Option` 是 `Some`，则返回 `Ok` 包装的值；如果是 `None`，则返回指定的错误。

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

**示例**:

```rust
let some_value = Some(42);
let none_value: Option<i32> = None;
let error = "Error";

assert_eq!(option_to_result(some_value, error.to_string()), Ok(42));
assert_eq!(option_to_result(none_value, error.to_string()), Err("Error".to_string()));
```

#### `result_to_option`

将 `Result<T, E>` 转换为 `Option<T>`。如果 `Result` 是 `Ok`，则返回 `Some` 包装的值；如果是 `Err`，则返回 `None`。

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

**示例**:

```rust
let ok_result = Ok::<i32, &str>(42);
let err_result = Err::<i32, &str>("Error");

assert_eq!(result_to_option(ok_result), Some(42));
assert_eq!(result_to_option(err_result), None);
```

## 测试

该项目包含一些单元测试来验证功能的正确性。您可以通过运行以下命令来执行测试：

```sh
cargo test
```

测试用例位于 `tests` 模块中，包括对 `option_to_result` 和 `result_to_option` 函数的测试：

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

## 许可证

此项目采用 GPL 许可证。详情请参阅 LICENSE 文件。

## 贡献

欢迎贡献！请提交问题 (Issues) 或拉取请求 (Pull Requests)。

---

感谢您使用 `mod`！如果您有任何问题或建议，请随时与我们联系。