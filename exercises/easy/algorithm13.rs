/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};

// add a sort function;
fn quick_sort<T1: PartialOrd + Copy>(array: &mut [T1], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let x = array[l];
    let mut i = l as i32 - 1;
    let mut j = r as i32 + 1;

    while i < j {
        loop {
            i += 1;
            if array[i as usize] >= x {
                break;
            }
        }
        
        loop {
            j-=1;
            if array[j as usize] <= x {
                break;
            }
        }

        if i < j {
            let a = array[i as usize];
            array[i as usize] = array[j as usize];
            array[j as usize] = a;
        } 
    }

    quick_sort(array, l as usize, j as usize);
    quick_sort(array, j as usize + 1, r as usize);
}

const N: usize = 100;
pub fn are_anagrams(s1: String, s2: String) -> bool {
    // TODO: Implement the logic to check if two strings are anagrams
    
    let mut a: [char; N] = ['a'; N];
    let mut b: [char; N] = ['a'; N];
    let mut sum: usize = 0;

    let mut idx: usize = 0;
    let s: String = s1.to_uppercase();
    for i in s.chars() {
        if 'A' <= i && i <= 'Z' {
            a[idx] = i;
            idx += 1;
            sum += 1;
        }
    }

    idx = 0;
    let s: String = s2.to_uppercase();
    for i in s.chars() {
        if 'A' <= i && i <= 'Z' {
            b[idx] = i;
            idx += 1;
            sum -= 1;
        }
    }

    if sum != 0 {
        return false;
    }

    let l: usize = s1.len();

    quick_sort(&mut a, 0, l - 1);
    quick_sort(&mut b, 0, l - 1);

    idx = 0;

    while idx < l {
        if a[idx] != b[idx] {
            return false;
        }
        idx += 1;
    }
    
    true // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
