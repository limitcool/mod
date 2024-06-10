// 将 Option<T> 转换为 Result<T, E>
pub fn option_to_result<T, E>(option: Option<T>, err: E) -> Result<T, E>
where
    E: Clone,
{
    match option {
        Some(value) => Ok(value),
        None => Err(err),
    }
}

// 将 Result<T, E> 转换为 Option<T>
pub fn result_to_option<T, E>(result: Result<T, E>) -> Option<T>
where
    E: Clone,
{
    match result {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_to_result() {
        let some_value = Some(42);
        let none_value: Option<i32> = None;
        let error = "Error";

        assert_eq!(option_to_result(some_value, error.to_string()), Ok(42));
        assert_eq!(
            option_to_result(none_value, error.to_string()),
            Err("Error".to_string())
        );
    }

    #[test]
    fn test_result_to_option() {
        let ok_result = Ok::<i32, &str>(42);
        let err_result = Err::<i32, &str>("Error");

        assert_eq!(result_to_option(ok_result), Some(42));
        assert_eq!(result_to_option(err_result), None);
    }
}
