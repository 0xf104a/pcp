use std::time::{SystemTime, UNIX_EPOCH};

pub mod runtime;
pub mod generic_iterator;
pub mod term;

///
/// Gets precise time in milliseconds
/// 
#[inline]
pub fn get_time() -> u128{
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH)
         .expect("Wrong system time").as_millis()
}

/// 
/// Takes first `n` characters of string
/// Works with Unicode
/// 
/// # Arguments 
/// 
/// * `s`: string to trim
/// * `n`: number of chars to take
/// 
/// returns: String: trimmed string
///
pub fn safe_string_trim_left(s: String, n: usize) -> String{
    let mut result: String = String::new();
    let mut r: usize = n;
    for c in s.chars(){
        if r == 0{
            break;
        }
        result.push(c);
        r -= 1;
    }
    result
}

///
/// Takes last `n` characters of string
/// Works with Unicode
///
/// # Arguments 
///
/// * `s`: string to trim
/// * `n`: number of chars to take
///
/// returns: String: trimmed string
///
pub fn safe_string_trim_right(s: String, n: usize) -> String{
    let mut result: String = String::new();
    let mut r: usize = 0;
    for c in s.chars(){
        if r < n{
            r += 1;
            continue;
        }
        result.push(c);
        r += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_string_trim_left_basic() {
        let input = String::from("Hello, World!");
        assert_eq!(safe_string_trim_left(input, 5), "Hello");
    }

    #[test]
    fn test_safe_string_trim_left_unicode() {
        let input = String::from("🌍🚀🌌🛸");
        assert_eq!(safe_string_trim_left(input, 2), "🌍🚀");
    }

    #[test]
    fn test_safe_string_trim_left_empty_string() {
        let input = String::from("");
        assert_eq!(safe_string_trim_left(input, 3), "");
    }

    #[test]
    fn test_safe_string_trim_left_more_chars_than_length() {
        let input = String::from("Short");
        assert_eq!(safe_string_trim_left(input, 10), "Short");
    }

    #[test]
    fn test_safe_string_trim_right_basic() {
        let input = String::from("Hello, World!");
        assert_eq!(safe_string_trim_right(input, 6), " World!");
    }

    #[test]
    fn test_safe_string_trim_right_unicode() {
        let input = String::from("🌍🚀🌌🛸");
        assert_eq!(safe_string_trim_right(input, 2), "🌌🛸");
    }

    #[test]
    fn test_safe_string_trim_right_empty_string() {
        let input = String::from("");
        assert_eq!(safe_string_trim_right(input, 3), "");
    }

    #[test]
    fn test_safe_string_trim_right_more_chars_than_length() {
        let input = String::from("Short");
        assert_eq!(safe_string_trim_right(input, 10), "");
    }
}
