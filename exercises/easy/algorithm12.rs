/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.
    
    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

use std::{fmt::{self, Display, Formatter}, vec};

pub fn is_palindrome(s: String) -> bool {
    // TODO: Implement the logic to check if the string is a palindrome
    let s = s.to_uppercase();
    let mut v: Vec<char> = Vec::new(); // Use vector as a statck 
    for i in s.chars() {
        if 'A' <= i && i <= 'Z' {
            v.push(i);
        }
    }

    let mut i: usize = 0;
    let last: usize = v.len() - 1;
    while i < v.len() / 2 {
        println!("{} vs {}", v[i] , v[last - i]);
        if v[i] != v[last - i] {
            return false;   
        }

        i += 1;
    }

    true // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
