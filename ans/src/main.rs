mod offer;
mod daily;

use std::collections::HashMap;
use std::option::Option;

/**
* 1. two sum
* Given an array of integers, return indices of the two numbers such that they add up to a specific target.
* You may assume that each input would have exactly one solution, and you may not use the same element twice.
*
* Example:
* Given nums = [2, 7, 11, 15], target = 9,
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 */

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let v: Vec<i32> = Vec::new();
    let mut m = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let num = target - *value;
        if m.contains_key(&num) {
            return vec![m[&num], index as i32];
        }
        m.insert(value, index as i32);
    }

    return v;
}

#[test]
fn test_two_sum() {
    let v = vec![2, 7, 11, 9];
    let ans = two_sum(v, 9);

    assert_eq!(ans[0], 0);
    assert_eq!(ans[1], 1);
}


/**
 * 2. add two numbers
 * You are given two non-empty linked lists representing two non-negative integers.
 * The digits are stored in reverse order and each of their nodes contain a single digit.
 * Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example:
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 */
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ret = Some(Box::new(ListNode::new(0)));
    let (mut p1, mut p2, mut carry) = (l1, l2, 0);
    let mut pos = ret.as_mut();
    while p1.is_some() || p2.is_some() {
        let mut sum = carry;

        if let Some(v) = p1 {
            sum += v.val;
            p1 = v.next;
        }

        if let Some(v) = p2 {
            sum += v.val;
            p2 = v.next;
        }

        if sum >= 10 {
            carry = 1;
            sum = sum - 10;
        } else {
            carry = 0;
        }

        let v = pos.unwrap();
        v.next = Some(Box::new(ListNode::new(sum)));
        pos = v.next.as_mut();
    }

    if carry > 0 {
        let v = pos.unwrap();
        v.next = Some(Box::new(ListNode::new(1)));
    }

    return ret.unwrap().next;
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(1, 1);

    let mut l1 = Box::new(ListNode::new(2));
    l1.next = Some(Box::new(ListNode::new(4)));
    l1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(7)));

    let mut l2 = Box::new(ListNode::new(5));
    l2.next = Some(Box::new(ListNode::new(6)));
    l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let ret = add_two_numbers(Some(l1), Some(l2));

    let expected_ret = [7, 0, 2, 1];
    let mut pos = ret;
    for i in 0..expected_ret.len() {
        assert_eq!(pos.is_some(), true);
        if !pos.is_some() {
            break;
        }

        if let Some(v) = pos {
            assert_eq!(v.val, expected_ret[i]);
            pos = v.next;
        }
    }
}

/**
 * 3. Longest Substring Without Repeating Characters
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example 1:
 *  Input: "abcabcbb"
 *  Output: 3
 *  Explanation: The answer is "abc", with the length of 3.
 *
 * Example 2:
 *  Input: "bbbbb"
 *  Output: 1
 *  Explanation: The answer is "b", with the length of 1.
 *
 * Example 3:
 *  Input: "pwwkew"
 *  Output: 3
 *  Explanation: The answer is "wke", with the length of 3.
 *  Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 */
use std::cmp;

fn length_of_longest_substring(s: String) -> i32 {
    let mut ret: i32 = 0;
    let mut start: i32 = -1;

    let mut dic = vec![-1; 128];

    let b = s.as_bytes();
    for i in 0..s.len() {
        // if repeat just move the start
        if dic[b[i as usize] as usize] > start {
            start = dic[b[i as usize] as usize]
        }

        ret = std::cmp::max(ret, i as i32 - start);
        dic[b[i as usize] as usize] = i as i32;
    }

    return ret as i32;
}

#[test]
fn test_length_of_longest_substring() {
    let case1 = String::from("abcabcbb");
    let case2 = String::from("bbbbb");
    let case3 = String::from("pwwkew");

    assert_eq!(length_of_longest_substring(case1), 3);
    assert_eq!(length_of_longest_substring(case2), 1);
    assert_eq!(length_of_longest_substring(case3), 3);
}

/**
 * 4. Median of Two Sorted Arrays
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 *
 * You may assume nums1 and nums2 cannot be both empty.
 *
 * Example 1:
 *  nums1 = [1, 3]
 *  nums2 = [2]
 *
 *  The median is 2.0
 *
 * Example 2:
 *  nums1 = [1, 2]
 *  nums2 = [3, 4]
 *
 *  The median is (2 + 3)/2 = 2.5
 */
fn get_median_of_two_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let ret: f64 = 0.0;

    let short;
    let long;

    if nums1.len() <= nums2.len() {
        short = nums1;
        long = nums2;
    } else {
        short = nums2;
        long = nums1;
    }

    let mut low = 0;
    let mut high = short.len();

    let mut part_x;
    let mut part_y;

    while low <= high {
        part_x = (low + high) / 2;
        part_y = (short.len() + long.len() + 1) / 2 - part_x;

        // get the value in short array
        let short_left = if part_x == 0 {
            i32::MIN
        } else {
            short[part_x - 1]
        };

        let short_right = if part_x == short.len() {
            i32::MAX
        } else {
            short[part_x]
        };

        // get the value in long array
        let long_left = if part_y == 0 {
            i32::MIN
        } else {
            long[part_y - 1]
        };

        let long_right = if part_y == long.len() {
            i32::MAX
        } else {
            long[part_y]
        };

        if short_left <= long_right && long_left <= short_right {
            if (short.len() + long.len()) % 2 == 0 {
                return f64::from(short_left.max(long_left) + short_right.min(long_right)) / 2.0;
            } else {
                return f64::from(short_left.max(long_left));
            }
        } else if short_left > long_right {
            high = part_x - 1;
        } else {
            low = part_x + 1;
        }
    }

    return ret;
}

#[test]
fn test_get_median_of_two_sored_arrays() {
    assert_eq!(1, 1);

    let demo1_num1 = vec![1, 3];
    let demo1_num2 = vec![2];

    assert_eq!(2.0, get_median_of_two_sorted_arrays(demo1_num1, demo1_num2));

    let demo2_num1 = vec![1, 2];
    let demo2_num2 = vec![3, 4];

    assert_eq!(2.5, get_median_of_two_sorted_arrays(demo2_num1, demo2_num2));
}

/**
 * 5. Longest Palindromic Substring
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum
 * length of s is 1000.
 *
 * Example 1:
 *  Input: "babad"
 *  Output: "bab"
 *  Note: "aba" is also a valid answer.
 *
 * Example 2:
 *  Input: "cbbd"
 *  Output: "bb"
 */
fn longest_palindrome(s: String) -> String {
    let size = s.len();

    if size == 0 {
        return String::from("");
    }

    let mut flags = vec![vec![false; size]; size];

    let mut start = 0;
    let mut max = 1;
    let b = s.as_bytes();

    for i in 1..size {
        flags[i][i] = true;

        for j in 0..i {
            flags[j][i] = b[j] == b[i] && (i - j < 3 || flags[j + 1][i - 1]);
            if flags[j][i] && i - j + 1 > max {
                start = j;
                max = i - j + 1;
            }
        }
    }

    let end = start + max;
    return (s[start as usize..end as usize]).to_string();
}

#[test]
fn test_longest_palindrome() {
    let demo1 = String::from("cbbd");
    let ret = longest_palindrome(demo1);
    assert_eq!(ret, "bb");
}

/**
* 6. ZigZag Conversion
* The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

* P   A   H   N
* A P L S I I G
* Y   I   R

* And then read line by line: "PAHNAPLSIIGYIR"

* Write the code that will take a string and make this conversion given a number of rows:
*
* string convert(string s, int numRows);
*
* Example 1:
*
* Input: s = "PAYPALISHIRING", numRows = 3
* Output: "PAHNAPLSIIGYIR"
*
* Example 2:
*
* Input: s = "PAYPALISHIRING", numRows = 4
* Output: "PINALSIGYAHRPI"
* Explanation:
*
* P     I    N
* A   L S  I G
* Y A   H R
* P     I
*/
fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || num_rows >= s.len() as i32 {
        return s;
    }

    let mut ret: Vec<Vec<char>> = vec![vec![]; num_rows as usize];

    let mut row = 0;
    let mut step: i32 = 1;

    for c in s.chars() {
        ret[row].push(c);
        if row == 0 {
            step = 1;
        }

        if row == num_rows as usize - 1 {
            step = -1;
        }

        row = (row as i32 + step) as usize;
    }

    let mut str_ret = String::new();
    for i in 0..num_rows {
        for j in &ret[i as usize] {
            str_ret.push(*j);
        }
    }

    return str_ret;
}

#[test]
fn test_convert() {
    let s = String::from("PAYPALISHIRING");
    let row = 3;
    let ret = convert(s, row);

    assert_eq!(ret, "PAHNAPLSIIGYIR");
}

/**
* 7. Reverse Integer
* Given a 32-bit signed integer, reverse digits of an integer.

* Example 1:
*
* Input: 123
* Output: 321
*
* Example 2:
*
* Input: -123
* Output: -321
*
* Example 3:
*
* Input: 120
* Output: 21
*/
fn reverse(x: i32) -> i32 {
    let mut num = x;
    let mut reverse_num: i64 = 0;
    while num != 0 {
        reverse_num = reverse_num * 10 + (num % 10) as i64;
        num = num / 10;
    }

    if reverse_num > i32::MAX as i64 || reverse_num < i32::MIN as i64 {
        return 0;
    }

    return reverse_num as i32;
}

#[test]
fn test_reverse() {
    let demo1 = -123;
    let ret = reverse(demo1);
    assert_eq!(ret, -321);
}

/**
* 8. String to Integer (atoi)
* Implement atoi which converts a string to an integer.

* The function first discards as many whitespace characters as necessary until the first non-whitespace character is found.
* Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
*
* The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
*
* If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
*
* If no valid conversion could be performed, a zero value is returned.
*
* Note:
*
* Only the space character ' ' is considered as whitespace character.
* Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1].
* If the numerical value is out of the range of representable values, INT_MAX (231 − 1) or INT_MIN (−231) is returned.
* Example 1:
*
* Input: "42"
* Output: 42
* Example 2:
*
* Input: "   -42"
* Output: -42
* Explanation: The first non-whitespace character is '-', which is the minus sign.
*              Then take as many numerical digits as possible, which gets 42.
* Example 3:
*
* Input: "4193 with words"
* Output: 4193
* Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
* Example 4:
*
* Input: "words and 987"
* Output: 0
* Explanation: The first non-whitespace character is 'w', which is not a numerical
*              digit or a +/- sign. Therefore no valid conversion could be performed.
* Example 5:
*
* Input: "-91283472332"
* Output: -2147483648
* Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
*              Thefore INT_MIN (−231) is returned.
*/
fn my_atoi(str: String) -> i32 {
    let mut ret: i64 = 0;
    let mut is_recording: bool = false;
    let mut is_positive_or_negative_sign_apper: bool = false;
    let mut is_negative: bool = false;

    for ch in str.chars().into_iter() {
        if ch == ' ' {
            if is_recording {
                break;
            }
            continue;
        }

        if ch == '-' {
            if is_positive_or_negative_sign_apper {
                break;
            }
            if is_recording {
                break;
            }
            is_recording = true;
            is_positive_or_negative_sign_apper = true;
            is_negative = true;
            continue;
        }

        if ch == '+' {
            if is_positive_or_negative_sign_apper {
                break;
            }
            if is_recording {
                break;
            }
            is_recording = true;
            is_positive_or_negative_sign_apper = true;
            continue;
        }

        if ch >= '0' && ch <= '9' {
            is_recording = true;
            ret = ret * 10 + ch.to_digit(10).unwrap() as i64;
            if ret > i32::MAX as i64 {
                break;
            }
            continue;
        }

        break;
    }

    if is_negative {
        ret = 0 - ret;
    }

    if ret > i32::MAX as i64 {
        return i32::MAX;
    }

    if ret < i32::MIN as i64 {
        return i32::MIN;
    }

    return ret as i32;
}

#[test]
fn test_my_atoi() {
    let s = String::from("9223372036854775808");
    assert_eq!(my_atoi(s), i32::MAX);
}

/**
* 9. Palindrome Numbera
* Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

* Example 1:
*
* Input: 121
* Output: true
* Example 2:
*
* Input: -121
* Output: false
* Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
* Example 3:
*
* Input: 10
* Output: false
* Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
*/
fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x != 0 && x % 10 == 0) {
        return false;
    }

    let mut tmp: i32 = x;
    let mut recv: i32 = 0;
    while tmp > recv {
        recv = recv * 10 + tmp % 10;
        tmp = tmp / 10;
    }

    return tmp == recv || tmp == recv / 10;
}

#[test]
fn test_is_palindrome() {
    let demo1 = 121;
    assert_eq!(is_palindrome(demo1), true);
    let demo2 = -121;
    assert_eq!(is_palindrome(demo2), false);
}

/**
* 10. Regular Expression Matching
* Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.

* '.' Matches any single character.
* '*' Matches zero or more of the preceding element.
* The matching should cover the entire input string (not partial).
*
* Note:
*
* s could be empty and contains only lowercase letters a-z.
* p could be empty and contains only lowercase letters a-z, and characters like . or *.
* Example 1:
*
* Input:
* s = "aa"
* p = "a"
* Output: false
* Explanation: "a" does not match the entire string "aa".
* Example 2:
*
* Input:
* s = "aa"
* p = "a*"
* Output: true
* Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
* Example 3:
*
* Input:
* s = "ab"
* p = ".*"
* Output: true
* Explanation: ".*" means "zero or more (*) of any character (.)".
* Example 4:
*
* Input:
* s = "aab"
* p = "c*a*b"
* Output: true
* Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".
* Example 5:
*
* Input:
* s = "mississippi"
* p = "mis*is*p*."
* Output: false
*/
fn is_match_bytes(s: &[u8], p: &[u8]) -> bool {
    match parse(p) {
        (Pattern::Empty, _) => s.is_empty(),
        (Pattern::Single(c), subp) => is_match_single(s, c, subp),
        (Pattern::Repeatable(c), subp) => is_match_single(s, c, p) || is_match_bytes(s, subp),
    }
}

fn is_match_single(s: &[u8], to_match: u8, p: &[u8]) -> bool {
    match s.split_first() {
        Some((c, s)) if to_match == b'.' || to_match == *c => is_match_bytes(s, p),
        _ => false,
    }
}

// Parser part:

enum Pattern {
    Empty,
    Single(u8),
    Repeatable(u8),
}

/// Returns the parsed pattern and the next pattern to parse.
fn parse(p: &[u8]) -> (Pattern, &[u8]) {
    match p.split_first() {
        None => (Pattern::Empty, p),
        Some((c, p)) => match p.split_first() {
            Some((b'*', p)) => (Pattern::Repeatable(*c), p),
            _ => (Pattern::Single(*c), p),
        },
    }
}

fn is_match(s: String, p: String) -> bool {
    return is_match_bytes(s.as_bytes(), p.as_bytes());
}

#[test]
fn test_is_match() {
    {
        let demo1 = String::from("aa");
        let match1 = String::from("a*");
        assert_eq!(is_match(demo1, match1), true);
    }
    {
        let demo1 = String::from("aa");
        let match1 = String::from("a");
        assert_eq!(is_match(demo1, match1), false);
    }
}

/**
 * 11. Container With Most Water
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai).
 * n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0). Find
 * two lines, which together with x-axis forms a container, such that the container contains the
 * most water.
 *
 * Note: You may not slant the container and n is at least 2.
 * Example:
 *
 * Input: [1,8,6,2,5,4,8,3,7]
 * Output: 49
 */

fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let len = (right - left) as i32;
        let h = height[left].min(height[right]);

        max = max.max((len * h) as i32);

        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }

    return max;
}

#[test]
fn test_max_area() {
    let demo = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(demo), 49);
}

/**
* 12. Integer to Roman
* Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

* Symbol       Value
* I             1
* V             5
* X             10
* L             50
* C             100
* D             500
* M             1000
* For example, two is written as II in Roman numeral, just two one's added together.
* Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
*
* Roman numerals are usually written largest to smallest from left to right.
* However, the numeral for four is not IIII. Instead, the number four is written as IV.
* Because the one is before the five we subtract it making four.
* The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
*
* I can be placed before V (5) and X (10) to make 4 and 9.
* X can be placed before L (50) and C (100) to make 40 and 90.
* C can be placed before D (500) and M (1000) to make 400 and 900.
* Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range from 1 to 3999.
*
* Example 1:
*
* Input: 3
* Output: "III"
* Example 2:
*
* Input: 4
* Output: "IV"
* Example 3:
*
* Input: 9
* Output: "IX"
* Example 4:
*
* Input: 58
* Output: "LVIII"
* Explanation: L = 50, V = 5, III = 3.
* Example 5:
*
* Input: 1994
* Output: "MCMXCIV"
* Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
*/

fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut table = vec![
        (1, "I"),
        (4, "IV"),
        (5, "V"),
        (9, "IX"),
        (10, "X"),
        (40, "XL"),
        (50, "L"),
        (90, "XC"),
        (100, "C"),
        (400, "CD"),
        (500, "D"),
        (900, "CM"),
        (1000, "M"),
    ];

    let mut ret = String::from("");

    while let Some(&(value, roman)) = table.last() {
        if num < value {
            table.pop();
        } else {
            num -= value;
            ret += roman;
        }
    }

    return ret;
}

#[test]
fn test_int_to_roman() {
    assert_eq!(int_to_roman(3), "III");
    assert_eq!(int_to_roman(4), "IV");
    assert_eq!(int_to_roman(9), "IX");
    assert_eq!(int_to_roman(58), "LVIII");
    assert_eq!(int_to_roman(1994), "MCMXCIV")
}

/**
 * 13. Roman to Integer
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 * I can be placed before V (5) and X (10) to make 4 and 9.
 * X can be placed before L (50) and C (100) to make 40 and 90.
 * C can be placed before D (500) and M (1000) to make 400 and 900.
 * Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
 *
 * Example 1:
 *
 * Input: "III"
 * Output: 3
 * Example 2:
 *
 * Input: "IV"
 * Output: 4
 * Example 3:
 *
 * Input: "IX"
 * Output: 9
 * Example 4:
 *
 * Input: "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 * Example 5:
 *
 * Input: "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 */
fn roman_to_int(s: String) -> i32 {
    let mut num: i32 = 0;

    let mut val: i32;
    let mut last: i32 = 0;

    for i in s.chars() {
        match i {
            'M' => val = 1000,
            'D' => val = 500,
            'C' => val = 100,
            'L' => val = 50,
            'X' => val = 10,
            'V' => val = 5,
            'I' => val = 1,
            _ => val = 0,
        }

        if val > last {
            num -= last * 2;
        }
        last = val;
        num += val;
    }

    return num;
}

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int(String::from("LVIII")), 58);
}

/**
* 14. Longest Common Prefix
* Write a function to find the longest common prefix string amongst an array of strings.

* If there is no common prefix, return an empty string "".
*
* Example 1:
*
* Input: ["flower","flow","flight"]
* Output: "fl"
* Example 2:
*
* Input: ["dog","racecar","car"]
* Output: ""
* Explanation: There is no common prefix among the input strings.
*/
fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::from("");
    }

    if strs.len() == 1 {
        return strs[0].to_string();
    }

    let min_len = strs.iter().map(|s| s.len()).min().unwrap();

    for i in 0..min_len {
        let c = strs[0].chars().nth(i).unwrap();

        for s in &strs {
            if s.chars().nth(i).unwrap() != c {
                return s[0..i].to_string();
            }
        }
    }

    return strs[0][..min_len].to_string();
}

#[test]
fn test_logest_common_prefix() {
    let demo = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    assert_eq!(longest_common_prefix(demo), "fl");
}

/**
* 15. 3Sum
* Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.

* Note:
*
* The solution set must not contain duplicate triplets.
*
* Example:
*
* Given array nums = [-1, 0, 1, 2, -1, -4],
*
* A solution set is:
* [
*   [-1, 0, 1],
*   [-1, -1, 2]
* ]
*/
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![vec![]];
    }

    let mut nums = nums;

    nums.sort();

    let mut result: Vec<Vec<i32>> = Vec::new();

    // fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn two_sum_in(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = Vec::new();

        let mut m = HashMap::new();

        for (index, value) in numbers.iter().enumerate() {
            let num = target - *value;
            if m.contains_key(&num) {
                v.push(vec![m[&num], index as i32]);
            }
            m.insert(value, index as i32);
        }

        return v;
    }

    let pos = nums.len() - 2;

    for i in 0..pos {
        if i != 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let ret = two_sum_in(nums[i + 1..nums.len()].to_vec(), 0 - nums[i]);
        if ret.len() > 0 {
            for j in &ret {
                println!(
                    "{} : {} : {}",
                    nums[i],
                    nums[i + 1 + j[0] as usize],
                    nums[i + 1 + j[1] as usize]
                );
                result.push(vec![
                    nums[i],
                    nums[i + 1 + j[0] as usize],
                    nums[i + 1 + j[1] as usize],
                ]);
            }
        }
    }

    return result;
}

#[test]
fn test_three_sum() {
    let demo = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!(three_sum(demo), vec![vec![-1, 0, 1], vec![-1, -1, 2]]);
}

/**
* 16. 3Sum Closest
* Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.

* Example:
*
* Given array nums = [-1, 2, 1, -4], and target = 1.
*
* The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
*/

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    let mut nums = nums;

    nums.sort();
    let mut ret = nums[0] + nums[1] + nums[2];

    for i in 0..(nums.len() - 2) {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let tmp = nums[i] + nums[left] + nums[right];
            if tmp == target {
                return target;
            }

            if (tmp - target).abs() < (ret - target).abs() {
                ret = tmp;
            }

            if tmp > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
    }

    return ret;
}

#[test]
fn test_three_sum_closet() {
    let demo = vec![-1, 2, 1, -4];
    assert_eq!(three_sum_closest(demo, 1), 2);
}

/**
* 17. Letter Combinations of a phone number
* Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
* A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
* Example 1:

* Input: digits = "23"
* Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
* Example 2:
*
* Input: digits = ""
* Output: []
* Example 3:
*
* Input: digits = "2"
* Output: ["a","b","c"]
*/
fn letter_combinations(digits: String) -> Vec<String> {
    let mut map = HashMap::new();
    map.insert('2', "abc".to_string());
    map.insert('3', "def".to_string());
    map.insert('4', "ghi".to_string());
    map.insert('5', "jkl".to_string());
    map.insert('6', "mno".to_string());
    map.insert('7', "pqrs".to_string());
    map.insert('8', "tuv".to_string());
    map.insert('9', "wxyz".to_string());

    fn dfs(
        idx: usize,
        digits: &String,
        map: &HashMap<char, String>,
        path: &mut String,
        ans: &mut Vec<String>,
    ) {
        if digits.len() == idx {
            ans.push(path.clone());
            return;
        }
        let cc = digits.chars().nth(idx).unwrap();
        for c in map.get(&cc).unwrap().chars() {
            path.push(c);
            dfs(idx + 1, &digits, &map, path, ans);
            path.pop();
        }
    }
    let mut ans = Vec::new();
    if digits.is_empty() {
        return ans;
    }
    dfs(0, &digits, &map, &mut String::new(), &mut ans);

    return ans;
}

#[test]
fn test_letter_combinations() {
    let digits = String::from("23");
    let ret = letter_combinations(digits);
    for i in ret {
        println!("{}", i);
    }
}
/**
* 18.
*/
fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];

    if nums.len() < 4 {
        return vec![];
    }
    let mut nums = nums;

    nums.sort();

    for i in 0..nums.len() - 3 {
        if i > 0 && nums[i] == nums[i-1] {continue;}
        for j in i+1..nums.len() - 2 {
            if j > i + 1 && nums[j] == nums[j-1] {continue;}
            let new_target = target - nums[i] - nums[j];
            let mut l = j + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let tmp = nums[l] + nums[r];
                if tmp == new_target {
                    ret.push(vec![nums[i], nums[j],nums[l], nums[r]]);
                    l = l+1;
                    r = r-1;
                    while nums[l] == nums[l-1] && l < r {l = l + 1; continue;}
                    while nums[r] == nums[r + 1] && l < r {r = r - 1; continue;}
                    continue;
                }

                if tmp > new_target {
                    r = r - 1;
                }

                if tmp < new_target {
                    l = l + 1;
                }

            }

        }
    }

    return ret;
}
#[test]
fn test_four_sum() {
    let nums = vec![2,2,2,2,2];
    let ret = four_sum(nums, 8);
    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            print!("{}\t", ret[i][j]);
        }
        println!();
    }
}

/**
* 40.
*/
fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    
    fn dfs(candidates: &Vec<i32>, target:i32, cans: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>, idx: usize) {
        if target < 0 {
            return
        }

        if target == 0 {
            ret.push(cans.clone());
            return
        }

        for i in idx..candidates.len() {
            if i > idx && candidates[i] == candidates[i-1] { continue; }
            cans.push(candidates[i]);
            dfs(candidates, target - candidates[i], cans, ret, i+1);
            cans.pop();
        }
    }

    let mut candidates = candidates;
    candidates.sort();
    dfs(&candidates, target, &mut vec![], &mut ret, 0);
    return ret;
}
#[test]
fn test_combination_sum2() {
    let candidates = vec![10,1,2,7,6,1,5];
    let ret = combination_sum2(candidates, 8);
    for i in 0..ret.len() {
        let tmp = ret.get(i).unwrap();
        for j in 0..tmp.len() {
            print!("{}\t", tmp.get(j).unwrap())
        }
        print!("\n");
    }
}

/**
* 46.
*/
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];

    if nums.len() == 1 {
        ret.push(nums);
        return  ret
    }

    for i in 0..nums.len() {
        let mut clone = nums.clone();
        let current = clone.remove(i);
        let tmp = permute(clone);
        for mut r in tmp {
            r.insert(0, current);
            ret.push(r);
        }
    }

    return ret;
}

#[test]
fn test_permute() {
    let nums = vec![1,2,3];
    let ret = permute(nums);
    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            print!("{}\t", ret[i][j])
        }
        println!();
    }
}

/**
* 47.
*/
fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];

    fn helper(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        if nums.len() == 1 {
            ret.push(nums);
            return  ret
        }

        for i in 0..nums.len() {
            let mut clone = nums.clone();
            let current = clone.remove(i);
            let tmp = permute(clone);
            for mut r in tmp {
                r.insert(0, current);
                ret.push(r);
            }
        }

        return ret;
    }

    let duplicated_ret = helper(nums);

    let mut tmp = HashMap::new();
    for i in 0..duplicated_ret.len() {
        tmp.insert(duplicated_ret[i].clone(), true);
    }

    for (k, _) in tmp {
        ret.push(k);
    }

    return ret;
}

#[test]
fn test_permute_unique() {
    let nums = vec![1,1,2];
    let ret = permute_unique(nums);
    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            print!("{}\t", ret[i][j])
        }
        println!();
    }
}

/**
* 51
*/

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn make_ans(pos: Vec<Vec<usize>>) -> Vec<Vec<String>> {
        let mut ans = vec![];
        for cols in pos.iter() {
            let len = cols.len();
            let mut solution = vec![];
            for &line in cols {
                let mut s = ".".repeat(len).to_string();
                s.replace_range(line..line + 1, "Q");
                solution.push(s);
            }
            ans.push(solution);
        }
        ans
    }
    fn not_valid(cols: &Vec<usize>, cur: usize) -> bool {
        cols.iter().enumerate().any(|(r, &c)| {
            cur == c
                || cols.len() == r
                || (cur as i64 - c as i64).abs() == (cols.len() as i64 - r as i64).abs()
        })
    }
    fn dfs(n: usize, cols: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>) {
        if cols.len() == n {
            ans.push(cols.to_vec());
            return;
        }
        for cur in 0..n {
            if not_valid(cols, cur) {
                continue;
            }
            cols.push(cur);
            dfs(n, cols, ans);
            cols.pop();
        }
    }
    let mut ans = vec![];
    dfs(n as usize, &mut vec![], &mut ans);
    return make_ans(ans)
}

#[test]
fn test_solve_n_queens() {
    let ret = solve_n_queens(4);
    for i in ret.iter() {
        for j in i.iter() {
            print!("{}\n", j);
        }
        println!("-------------------------------");
    }
}

/**
* 52.
*/
fn total_n_queens(n: i32) -> i32 {
    let ret = solve_n_queens(n);
    return ret.len() as i32;
}

#[test]
fn test_total_n_queens() {
    let ret = total_n_queens(10);
    println!("{}", ret);
}

/**
* 55.
*/
fn can_jump(nums: Vec<i32>) -> bool {
    let (mut start, mut end) = (0, 0);
    while start <= end {
        end = end.max(nums[start] as usize + start);
        if end >= nums.len() - 1 {
            return true;
        }
        start += 1;
    }
    return end >= nums.len() - 1;
}

#[test]
fn test_can_jump() {
    let example = vec![2,3,1,1,4];
    let ret = can_jump(example);
    println!("{}", ret);
}

/**
* 56
*/
fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ret = vec![];

    let mut intervals = intervals;

    intervals.sort();

    let mut start = intervals[0][0];
    let mut end = intervals[0][1];

    for i in 1..intervals.len() {
        let target = intervals.get(i).unwrap();

        // add new start and end
        if target[0] > end {
            ret.push(vec![start, end]);
            start = target[0];
            end = target[1];
        }

        if target[1] > end {
            end = target[1]
        }
    }
    ret.push(vec![start, end]);

    return ret;
}

#[test]
fn test_merge() {
    let example = vec![vec![1, 4], vec![4, 5]];
    let ret = merge(example);
    println!("{}", ret.len());
}

/**
* 58.
*/
fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;

    intervals.push(new_interval);
    return merge(intervals);
}

#[test]
fn test_insert() {
    let example = vec![vec![1, 3], vec![6, 9]];
    let new = vec![2, 5];
    let ret = insert(example, new);
    println!("{}", ret.len())
}

/**
 * 59
 */
fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![0; n as usize]; n as usize];

    let mut direction = 0; // 0: left->right, 1: top->down, 2: right->left, 3: down->top
    let (mut top, mut right, mut left, mut down) = (0 as usize, (n - 1) as usize, 0 as usize, (n - 1) as usize);
    let mut count = 1;
    while top <= down && left <= right {
        match direction {
            0 => {
                for i in left..right+1 {
                    ret[top][i] = count;
                    count += 1;
                }
                top += 1;
                direction = 1;
            }
            1 => {
                for i in top..down+1 {
                    ret[i][right] = count;
                    count += 1;
                }
                right -= 1;
                direction = 2;
            }
            2 => {
                for i in (left..right+1).rev() {
                    ret[down][i] = count;
                    count +=1;
                }
                down -=1;
                direction = 3
            }
            3=> {
                for i in (top..down+1).rev() {
                    ret[i][left] = count;
                    count += 1;
                }
                left +=1;
                direction = 0;
            }
            _ => {}
        }
    }


    return ret;
}

#[test]
fn test_generate_matrix() {
    let ret = generate_matrix(3);
    println!("{}", ret.len());
}

/**
 * 60
 */
fn get_permutation(n: i32, k: i32) -> String {
    let (mut seq, mut res) = (String::with_capacity(n as usize), String::with_capacity(n as usize));
    (1..=(n as u8)).for_each(|x| { seq.push((b'0' + x) as char); });
    let (mut base, mut k) = ((1..=(n - 1)).fold(1, |m, x| { x * m }), k - 1);

    for i in (1..=(n - 1)).rev() {
        let pos = (k / base) as usize;
        let c = seq.chars().nth(pos);
        res.push(c.unwrap());
        seq.remove(pos);
        k %= base;
        base /= i;
    }

    res.push(seq.chars().nth(0).unwrap());

    res
}

#[test]
fn test_get_permutation() {
    let ret = get_permutation(3, 3);
    println!("{}", ret);
}

/**
 * 61.
 */
fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = match head {
        Some(a) => a,
        None => return None,
    };

    let mut size = 1;
    let mut pos = &head;

    while let Some(ref cur) = pos.next {
        size += 1;
        pos = cur;
    }

    println!("size: {}", size);

    let mut target = size - k % size;
    if target == size {
        return Some(head);
    }


    let mut cur = &mut head;
    while target > 1 {
        cur = cur.next.as_mut().unwrap();
        target -= 1;
    }

    let mut ret = std::mem::replace(&mut cur.next, None).unwrap();
    let mut cur = &mut ret;
    while let Some(ref mut next) = cur.next {
        cur = next;
    }
    cur.next = Some(head);

    return Some(ret);
}

#[test]
fn test_rotate_right() {
    let head = Box::new(ListNode::new(2));
    let ret = rotate_right(Some(head), 1);
    while let Some(node) = ret.as_ref() {
        println!("{}", node.val);
        break;
    }
}

/**
* 62 Unique Paths
* A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

* The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
*
* How many possible unique paths are there?
*
* Example 1:

* Input: m = 3, n = 2
* Output: 3
* Explanation:
* From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
* 1. Right -> Right -> Down
* 2. Right -> Down -> Right
* 3. Down -> Right -> Right
*
* Example 2:
*
* Input: m = 7, n = 3
* Output: 28
**/

fn unique_paths(m: i32, n: i32) -> i32 {
    let n = n as usize;
    let m = m as usize;

    let mut paths = vec![vec![0; n]; m];

    for i in 0..m {
        paths[i][0] = 1;
    }

    for j in 0..n {
        paths[0][j] = 1;
    }

    for i in 1..m {
        for j in 1..n {
            paths[i][j] = paths[i - 1][j] + paths[i][j - 1];
        }
    }

    return paths[m - 1][n - 1] as i32;
}

#[test]
fn test_unique_paths() {
    let m = 7;
    let n = 3;

    assert_eq!(unique_paths(m, n), 28)
}

/**
* 64. Minimum Path Sum
* Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
* Note: You can only move either down or right at any point in time.

* Example:
*
* Input:
* [
*   [1,3,1],
*   [1,5,1],
*   [4,2,1]
* ]
* Output: 7
* Explanation: Because the path 1→3→1→1→1 minimizes the sum.
*/

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let grid = grid;

    let m = grid.len();
    let n = grid[0].len();

    let mut min_paths = grid;
    for i in 1..m {
        min_paths[i][0] = min_paths[i - 1][0] + min_paths[i][0];
    }

    for i in 1..n {
        min_paths[0][i] = min_paths[0][i - 1] + min_paths[0][i];
    }

    for i in 1..m {
        for j in 1..n {
            min_paths[i][j] = min_paths[i][j] + cmp::min(min_paths[i - 1][j], min_paths[i][j - 1]);
        }
    }

    return min_paths[m - 1][n - 1];
}

#[test]
fn test_in_path_sum() {
    let grid: Vec<Vec<i32>> = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(min_path_sum(grid), 7);
}

/**
 * 65.
 */
fn is_number(s: String) -> bool {
    for ch in s.chars() {
        if ch.is_alphabetic() && (ch != 'e' && ch != 'E') {
            return false;
        }
    }

    enum State {
        Begin,
        Sign,   // "+"     "-"
        Num,    // "+1"    "-12",   "123"
        NumDot, // "1."    "-2."
        InDot,  // "-."    "+."     "."
        DotNum, // ".9"    "+1.2"   "0.0"
        InE,    // "2.E"   "-0.9e"
        InESign,// "4E-"   "0.01e+"
        InENum, // "3e-2"  "4.e12"
        Error,
    }

    use State::*;

    fn is_digit(c: u8) -> bool{
        return b'0' <= c && c <= b'9';
    }

    let mut state = Begin;
    for c in s.as_bytes() {
        state = match state {
            Begin => match c {
                b'+' | b'-' => Sign,
                b'.' => InDot,
                c if is_digit(*c) => Num,
                _ => Error,
            }
            Sign => match c {
                c if is_digit(*c) => Num,
                b'.' => InDot,
                _ => Error,
            },
            Num => match c {
                c if is_digit(*c) => Num,
                b'.' => NumDot,
                b'e' | b'E' => InE,
                _ => Error,
            },
            InDot => match c {
                c if is_digit(*c) => DotNum,
                _ => Error,
            },
            NumDot => match c {
                c if is_digit(*c) => DotNum,
                b'e' | b'E' => InE,
                _ => Error,
            }
            DotNum => match c {
                c if is_digit(*c) => DotNum,
                b'e' | b'E' => InE,
                _ => Error,
            },
            InE => match c {
                b'+' | b'-' => InESign,
                c if is_digit(*c) => InENum,
                _ => Error,
            },
            InESign => match c {
                c if is_digit(*c) => InENum,
                _ => Error,
            }
            InENum => match c {
                c if is_digit(*c) => InENum,
                _ => Error,
            }
            Error => return false,
        };
    }
    return match state {
        Num | NumDot | DotNum | InENum => true,
        _ => false,
    }
}

#[test]
fn test_is_number() {
    let example = String::from("0");
    let ret = is_number(example);
    println!("result: {}", ret);
}

/**
 * 67.
 */
fn add_binary(a: String, b: String) -> String {
    let ia = i128::from_str_radix(&a, 2).unwrap();
    let ib = i128::from_str_radix(&b, 2).unwrap();
    return format!("{:b}", (ia+ib)).to_string();
}

#[test]
fn test_add_binary() {
    let a = String::from("11");
    let b = String::from("1");
    let ret = add_binary(a, b);
    println!("{}", ret);
}

/**
* 72. Edit Distance
* Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.

* You have the following 3 operations permitted on a word:
*
* Insert a character
* Delete a character
* Replace a character
* Example 1:
*
* Input: word1 = "horse", word2 = "ros"
* Output: 3
* Explanation:
* horse -> rorse (replace 'h' with 'r')
* rorse -> rose (remove 'r')
* rose -> ros (remove 'e')
* Example 2:
*
* Input: word1 = "intention", word2 = "execution"
* Output: 5
* Explanation:
* intention -> inention (remove 't')
* inention -> enention (replace 'i' with 'e')
* enention -> exention (replace 'n' with 'x')
* exention -> exection (replace 'n' with 'c')
* exection -> execution (insert 'u')
**/

fn min_distance(word1: String, word2: String) -> i32 {
    let n = word1.len();
    let m = word2.len();

    let mut steps = vec![vec![0; n + 1]; m + 1];

    for i in 0..m + 1 {
        steps[i][0] = i;
    }

    for i in 0..n + 1 {
        steps[0][i] = i;
    }

    let b1 = word1.as_bytes();
    let b2 = word2.as_bytes();

    for i in 0..m {
        for j in 0..n {
            if b1[j] == b2[i] {
                steps[i + 1][j + 1] = steps[i][j];
            } else {
                steps[i + 1][j + 1] =
                    std::cmp::min(std::cmp::min(steps[i][j], steps[i + 1][j]), steps[i][j + 1]) + 1;
            }
        }
    }
    return steps[m][n] as i32;
}

#[test]
fn test_min_distance() {
    let word1 = String::from("intention");
    let word2 = String::from("execution");

    assert_eq!(min_distance(word1, word2), 5);
}

/**
* 74.
*/
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut matrix = matrix;

    let mut container = vec![];
    for i in 0..matrix.len() {
        container.append(&mut matrix[i]);
    }

    fn helper(array: Vec<i32>, target: i32) -> bool {
        if array.is_empty() {
            return false;
        }

        let mut start = 0;
        let mut end = array.len() - 1;
        while start < end {
            let mid = (start + end) / 2;
            if array[mid] == target {
                return true;
            }
            if array[mid] < target {
                start = mid + 1;
            } else {
                end = mid;
            }
        }

        return array[start] == target;
    }

    return helper(container, target);
}

#[test]
fn test_search_matrix() {
    let example = vec![vec![1,2,5,7], vec![10, 11, 16,20], vec![23,30,34,60]];
    let ret = search_matrix(example, 13);
    println!("{}", ret);
}

use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/*
 * 94. binary tree inorder traversal
 */
fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn do_inorder(r: &mut Option<Rc<RefCell<TreeNode>>>, a: &mut Vec<i32>) {
        if let Some(n) = r {
            let mut n = n.borrow_mut();
            do_inorder(&mut n.left, a);
            a.push(n.val);
            do_inorder(&mut n.right, a);
        }
    }
    let mut root = root;
    let mut res = vec![];
    do_inorder(&mut root, &mut res);
    return res;
}

#[test]
fn test_inorder_traversal() {
    let root = None;
    inorder_traversal(root);
}

/*
 * 95. Unique Binary Search Trees II
 * Given an integer n, return all the structurally unique binary search trees, which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
 */
fn get_all_unique_binary_search_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let ret ;

    fn helper(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l > r {
            return vec![None];
        }

        let mut res = vec![];
        for i in l..r + 1 {
            let l_nodes = helper(l, i - 1);
            let r_nodes = helper(i + 1, r);
            for ln in l_nodes.iter() {
                for rn in r_nodes.iter() {
                    let node = Some(Rc::new(RefCell::new(TreeNode {
                        val: i,
                        left: ln.clone(),
                        right: rn.clone(),
                    })));
                    res.push(node);
                }
            }
        }

        return res;
    }

    ret = helper(1, n);

    return ret;
}

#[test]
fn test_get_al_l_unique_binary_search_trees() {
    get_all_unique_binary_search_trees(5);
}

/*
 * 96.  Unique Binary Search Trees
 */
fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp: Vec<i32> = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        for j in 1..=i {
            dp[i] += dp[j - 1] * dp[i - j];
        }
    }

    return dp[n];
}

#[test]
fn test_num_trees() {
    num_trees(10);
}
/*
 * 98. Validate Binary Search Tree
 */
fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn valid(node: Option<Rc<RefCell<TreeNode>>>, max: i64, min: i64) -> bool {
        if let Some(node) = node {
            if node.borrow().val as i64 <= min || node.borrow().val as i64 >= max {
                return false;
            }
            return valid(node.borrow().left.clone(), min, node.borrow().val as i64)
                && valid(node.borrow().right.clone(), node.borrow().val as i64, max);
        }
        true
    }
    valid(root, std::i64::MIN, std::i64::MAX)
}

#[test]
fn test_unique_binary_search_trees_ii() {
    is_valid_bst(None);
}

/*
 * 100. same tree
 */
fn same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p = p.borrow();
            let q = q.borrow();
            p.val == q.val
                && same_tree(p.left.clone(), q.left.clone())
                && same_tree(p.right.clone(), q.right.clone())
        }
        _ => false,
    }
}

#[test]
fn test_same_tree() {
    let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let q = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    assert_eq!(true, same_tree(p, q));
}

/*
 * 101. Symmetric Tree
 */
fn symmetric_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    type Rn = Rc<RefCell<TreeNode>>;

    fn f(p: Option<&Rn>, q: Option<&Rn>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && f(p.left.as_ref(), q.right.as_ref())
                    && f(p.right.as_ref(), q.left.as_ref())
            }
            _ => false,
        }
    }

    match root {
        None => true,
        Some(n) => {
            let n = n.borrow();
            f(n.left.as_ref(), n.right.as_ref())
        }
    }
}

#[test]
fn test_symmetric_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    symmetric_tree(root);
}

/**
* 103.
*/
fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut q = VecDeque::new();

    if root != None {
        q.push_back(root);
    }

    let mut level = 0;
    while !q.is_empty() {
        let mut tmp = vec![];
        let size = q.len();
        for _ in 0..size {
            if let Some(node)= q.pop_front().unwrap() {
                tmp.push(node.borrow().val);

                if node.borrow().left != None {
                    q.push_back(node.borrow_mut().left.take());
                }

                if node.borrow().right != None {
                    q.push_back(node.borrow_mut().right.take());
                }
            }
        }

        if level % 2 == 1 {
            tmp.reverse();
        }

        level += 1;
        ret.push(tmp);
    }

    return ret;
}

#[test]
fn test_zigzag_level_order() {
    let example = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let ret = zigzag_level_order(example);
    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            print!("{}\t", ret[i][j]);
        }
    }
}

/**
* 104.
*/
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left = max_depth(node.borrow_mut().left.take());
            let right = max_depth(node.borrow_mut().right.take());
            return if left >= right {
                left + 1
            } else {
                right + 1
            }
        }
    }
}

#[test]
fn test_max_depth() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let ret = max_depth(root);
    println!("{}", ret);
}

/**
 * 108. convert sorted array into binary search tree
 */
fn convert_sorted_array_into_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;
    let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
    root.borrow_mut().left = convert_sorted_array_into_bst(nums[..mid].to_vec());
    root.borrow_mut().right = convert_sorted_array_into_bst(nums[mid + 1..].to_vec());

    return Some(root);
}

#[test]
fn test_convert_sorted_array_into_bst() {
    let nums = vec![-10, -3, 0, 5, 9];
    let ret = convert_sorted_array_into_bst(nums);
    print!("{}", ret.is_none());
}

/*
 * 113. Path sum II
 */
fn path_sum_ii(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];

    fn helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        buf: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            let val = node.borrow().val;
            let l = &node.borrow().left;
            let r = &node.borrow().right;
            buf.push(val);
            if target == val && l.is_none() && r.is_none() {
                ret.push(buf.clone());
            }
            helper(l, target - val, buf, ret);
            helper(r, target - val, buf, ret);
            buf.pop();
        }
    }

    helper(&root, target_sum, &mut vec![], &mut ret);
    return ret;
}

#[test]
fn test_path_sum_ii() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));
    let ret = path_sum_ii(Some(root), 5);
    for i in ret {
        for j in i {
            print!("{}\t", j);
        }

        print!("\n");
    }
}

/**
* 219. Contains Duplicate II
*/
fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut container = HashMap::new();

    for i in 0..nums.len() {
        if let Some(j) = container.get(&nums[i]) {
            if i - j <= k as usize {
                return true;
            }
        }

        container.insert(nums[i], i);
    }
    return false;
}

#[test]
fn test_contains_nearby_duplicate() {
    let nums = vec![1, 2, 3, 1, 2, 3];
    println!("{}", contains_nearby_duplicate(nums, 2))
}

/*
 * 257. binary tree paths
 */
fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        if let Some(node) = root {
            let (left, right) = (&node.borrow().left, &node.borrow().right);
            if left.is_none() && right.is_none() {
                result.push(vec![node.borrow().val.to_string()]);
            } else {
                [left, right].iter().for_each(|branch| {
                    result.extend(helper(*branch).into_iter().map(|mut p| {
                        p.push(node.borrow().val.to_string());
                        p
                    }))
                });
            }
        }
        return result;
    }

    return helper(&root)
        .into_iter()
        .map(|path| path.into_iter().rev().collect::<Vec<String>>().join("->"))
        .collect();
}

#[test]
fn test_binary_tree_paths() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    binary_tree_paths(Some(root));
}

/**
* 258.
*/
fn add_digits(num: i32) -> i32 {
    return (num - 1) % 9 + 1;
}

#[test]
fn test_add_digits() {
    let ret = add_digits(35);
    println!("{}", ret);
}

/**
* 299.
*/
fn get_hint(secret: String, guess: String) -> String {
    let ret = String::from("");

    return ret;
}

#[test]
fn test_get_hint() {
    let secret = String::from("1807");
    let guess = String::from("7810");

    let ret = get_hint(secret, guess);
    println!("{}", ret)
}



/**
* 334.
*/
fn increasing_triplet(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    let mut small = i32::MAX;
    let mut mid = i32::MAX;

    for i in nums {
        if small >= i {
            small = i;
        } else if mid >= i {
            mid = i;
        } else if i > mid {
            return true;
        }
    }

    return false;
}

#[test]
fn test_increasing_triplet() {
    let example = vec![1,2,3,4,5];
    let ret = increasing_triplet(example);
    assert_eq!(ret, true);
}

/**
* 373.
*/
fn  k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut sums_pair:Vec<Vec<i32>> = Vec::new();
    let (len1, len2) = (nums1.len(), nums2.len());

    if len1 == 0 || len2 == 0 {
        return sums_pair;
    }

    let mut next = vec![0; len1];

    let mut floor = 0;
    let mut ceiling = 0;
    let mut curr = 0;

    for _ in 0..k {
        sums_pair.push(vec!(nums1[curr], nums2[next[curr]]));
        next[curr] += 1;
        if next[curr] >= len2 {
            floor = curr +1;
            if floor >= len1 {
                break;
            }
        }
        if curr == ceiling && ceiling +1 < len1{
            ceiling += 1;
        }
        let mut sum = i32::MAX;
        for c in floor..ceiling+1 {
            if nums1[c] + nums2[next[c]] < sum{
                sum = nums1[c] + nums2[next[c]];
                curr = c;
            }
        }
    }

    return sums_pair;
}

#[test]
fn test_k_smallest_pairs() {
    let nums1 = vec![1, 2, 4];
    let nums2 = vec![-1, 1, 2];
    let ret = k_smallest_pairs(nums1, nums2, 10);
    let ret = ret;
    for i in 0..ret.len() {
        print!("[{}, {}]\n", ret[i][0], ret[i][1]);
    }

}

/**
* 382.
*/
struct EqualList {
    list: Vec<i32>,
}

use rand::Rng;

impl EqualList {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut list = vec![];

        let mut pos = head.as_ref();

        while let Some(node) = pos {
            list.push(node.val);
            pos = node.next.as_ref();
        }

        return EqualList {list};
    }

    fn get_random(&self) -> i32 {
        let mut ra = rand::thread_rng();
        let target: usize = ra.gen_range(0..self.list.len());
        return self.list[target];
    }
}
#[test]
fn test_equal_list() {
    let head = Box::new(ListNode::new(2));
    let obj = EqualList::new(Some(head));
    let ret = obj.get_random();
    println!("{}", ret);
}

/**
* 386.
*/
fn lexical_order(n: i32) -> Vec<i32> {
    fn helper(cur: i32, max: i32, ret: &mut Vec<i32>) {
        if cur > max {
            return;
        }
        ret.push(cur);
        for i in 0..10 {
            helper(cur * 10 + i, max, ret);
        }
    }

    let mut ret = vec![];

    for i in 1..10 {
        helper(i, n, &mut ret)
    }

    return ret;
}

#[test]
fn test_lexical_order() {
    let ret = lexical_order(13);
    for i in 0..ret.len() {
        print!("{} ", ret[i]);
    }
}


/*
 * 414. third max number
 */
fn third_max(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums.dedup();
    if nums.len() < 3 {
        return nums[nums.len() - 1];
    }

    return nums[nums.len() - 3];
}

#[test]
fn test_third_max() {
    let arr = vec![4, 2, 2, 3, 2, 1];
    let ret = third_max(arr);
    print!("{}", ret);
}

/**
* 521.
*/
fn find_lu_slength(a: String, b: String) -> i32 {
    return if a == b {-1} else {(a.len().max(b.len())) as i32};
}

#[test]
fn test_find_lu_slength() {
    let a = String::from("aba");
    let b = String::from("dcd");
    let ret = find_lu_slength(a, b);
    println!("{}", ret);
}

/**
 * 538. Convert BST to Greater Tree
 */
fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            helper(&n.right, sum);
            n.val += *sum;
            *sum = n.val;
            helper(&n.left, sum);
        }
    }

    let mut sum: i32 = 0;
    helper(&root, &mut sum);

    root
}

#[test]
fn test_convert_bst() {
    convert_bst(None);
}

/**
* 539.
*/
fn find_min_difference(time_points: Vec<String>) -> i32 {
    fn get_minutes(str: String) -> i32 {
        let b = str.as_bytes();
        return ((b[0] - '0' as u8) as i32 * 10 + (b[1] - '0' as u8) as i32) * 60 + (b[3] - '0' as u8) as i32 * 10 + (b[4] - '0' as u8) as i32;
    }

    let mut container = vec![];
    let time_points = time_points;
    for i in 0..time_points.len() {
        container.push(get_minutes(time_points[i].to_string()));
    }

    container.sort();
    let mut ret = i32::MAX;
    for i in 1..container.len() {
        let tmp = container[i] - container[i-1];
        ret = min(tmp, ret);
    }

    ret = min(ret, container[0] + 1440 - container[container.len()-1]);
    return ret;
}

#[test]
fn test_find_min_difference() {
    let example = vec![String::from("23:59"), String::from("00:01"), String::from("00:12")];
    let ret = find_min_difference(example);
    print!("{}", ret);
}

/**
* 547
*/
struct OrderedUnion {
    root: Vec<i32>,
    rank: Vec<i32>
}

impl OrderedUnion {
    fn union(&mut self, target1: i32, target2: i32) {
        let r1 = self.find(target1 as usize);
        let r2 = self.find(target2 as usize);

        if r1 != r2 {
            if self.rank[r1 as usize] > self.rank[r2 as usize] {
                self.root[r2 as usize] = r1
            } else if self.rank[r1 as usize] < self.rank[r2 as usize] {
                self.root[r1 as usize] = r2
            } else {
                self.root[r2 as usize] = r1;
                self.rank[r1 as usize]+=1;
            }
        }

        for i in 0..self.root.len() {
            print!("{}\t", self.root[i]);
        }
        println!();
    }

    fn find(&mut self, t: usize) -> i32 {
        if self.root[t] == t as i32 {
            return t as i32;
        }

        self.root[t as usize] = self.find(self.root[t] as usize);
        return self.root[t];
    }

    fn connected(&mut self, t1: i32, t2: i32) -> bool{
        self.find(t1 as usize) == self.find(t2 as usize)
    }

    fn new(size: usize) -> OrderedUnion{

        let mut obj = OrderedUnion{
            root: vec![0; size],
            rank: vec![1; size]
        };

        for i in 0..obj.root.len() {
            obj.root[i] = i as i32;
        }

        return obj
    }}

fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut obj = OrderedUnion::new(is_connected[0].len());

    for i in 0..is_connected.len() {
        for j in 0..is_connected[0].len() {
            if is_connected[i][j] == 1 {
                obj.union(i as i32, j as i32)
            }
        }
    }

    let mut map = HashMap::new();
    for i in 0..obj.root.len(){
        map.insert(obj.root[i], 0);
    }

    map.len() as i32
}

#[test]
fn test_find_circle_num() {
    let obj = vec![
        vec![1,0,0,1],
        vec![0,1,1,0],
        vec![0,1,1,1],
        vec![1,0,1,1],
    ];
    println!("{}", find_circle_num(obj));

}

/**
* 599
*/
fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut ret = vec![];
    let mut container: HashMap<String, usize>= HashMap::new();


    for (i,item) in list1.iter().enumerate() {
        container.insert(item.to_string(), i);
    }

    let mut min = usize::MAX;
    for (i, item) in list2.iter().enumerate() {
        if let Some(j) = container.get(item) {
            if i + j < min {
                ret = vec![item.to_string()];
                min = i + j;
            } else if i + j == min {
                ret.push(item.to_string());
            }
        }
    }

    return ret;
}

#[test]
fn test_find_restaurant() {
    let l1 = vec! [String::from("Shogun"), String::from("Tapioca Express"), String::from("Burger King"), String::from("KFC")];
    let l2 = vec![String::from("Piatti"), String::from("The Grill at Torrey Pines"), String::from("Hungry Hunter Steakhouse"), String::from("Shogun")];
    let ret = find_restaurant(l1, l2);
    for i in ret {
        println!("{}", i)
    }
}

/**
 * 688
 */
fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let mut dp = vec![vec![vec![0.0; k as usize + 1]; n as usize]; n as usize];
    let dirs = vec![vec![-1, -2], vec![-1, 2], vec![1, -2], vec![1, 2], vec![-2, 1], vec![-2, -1], vec![2, 1], vec![2, -1]];

    for i in 0..n {
        for j in 0..n {
            dp[i as usize][j as usize][0] = 1.0 as f64;
        }
    }

    for p in 1..k + 1 {
        for i in 0..n {
            for j in 0..n {
                for  d in dirs.clone() {
                    let nx = i + d[0];
                    let ny = j + d[1];
                    if nx < 0 || ny < 0 || nx >= n || ny >= n {
                        continue;
                    }

                    dp[i as usize][j as usize][p as usize] += dp[nx as usize][ny as usize][p as usize-1]/8.0;
                }
            }
        }
    }

    return dp[row as usize][column as usize][k as usize];
}

#[test]
fn test_knight_probability() {
    let ret = knight_probability(10, 13, 5, 3);
    println!("{}", ret);
}

/**
* 717.
*/
fn is_one_bit_character(bits: Vec<i32>) -> bool {
    if bits[bits.len() - 1] == 1 {
        return false;
    }

    if bits.len() > 2 {
        if bits[bits.len() - 1] == 0 && bits[bits.len() - 2] == 0 {
            return true;
        }
    }

    let mut ret = false;

    let mut container = vec![];
    for i in bits {
        if i == 1 {
            if container.len() == 1 {
                container.clear();
                ret = false;
            } else {
                container.push(i);
            }
        } else {
            if container.len() == 1 {
                container.clear();
                ret = false;
            } else {
                ret = true;
            }
        }
    }

    return ret;
}

#[test]
fn test_is_one_bit_character() {
    let example = vec![1,1,1,0];
    let ret = is_one_bit_character(example);
    println!("ret: {}", ret);
}

/**
* 720.
*/
fn longest_word(words: Vec<String>) -> String {
    let mut ret = String::from("");

    return ret;
}

#[test]
fn test_longest_word() {
    let example = vec![String::from("w"), String::from("wo"), String::from("wor"), String::from("worl"), String::from("world")];
    let ret = longest_word(example);
    println!("{}", ret);

}

/**
 * 747.
 */
fn dominant_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut max = i32::MIN;
    let mut max_pos = 0;

    let mut second_max = i32::MIN;

    for i in 0..nums.len() {
        if nums[i] > max {
            second_max = max;
            max = nums[i];
            max_pos = i;
            continue;
        }

        if nums[i] > second_max {
            second_max = nums[i]
        }
    }

    if max < second_max * 2 {
        return -1
    }

    return max_pos as i32;
}

#[test]
fn test_dominant_index() {
    let nums = vec![1,2,3,4];
    let ret = dominant_index(nums);
    assert_eq!(ret, -1)
}

/**
 * 783.  Minimum Distance Between BST Nodes
 */
fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut buff = vec![];

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, buf: &mut Vec<i32>) {
        if let Some(node) = root {
            let val = node.borrow().val;
            let l = &node.borrow().left;
            let r = &node.borrow().right;
            helper(l, buf);
            buf.push(val);
            helper(r, buf);
        }
    }

    helper(&root, &mut buff);

    return buff.windows(2).map(|c| c[1] - c[0]).min().unwrap();
}

#[test]
fn test_min_diff_in_bst() {
    let root = Rc::new(RefCell::new(TreeNode{
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode{
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode{
            val: 6,
            left: None,
            right: None,
        }))),
    }));

    min_diff_in_bst(Some(root));
}

/**
* 797
**/
fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // bfs
    let mut ret = vec![];

    use std::collections::VecDeque;

    let mut path : Vec<i32> = vec![0];
    let mut container = VecDeque::from(vec![path]);

    let mut cc = 0;
    while container.len()  > 0{
        cc+=1;
        let mut current = container.pop_front().unwrap().clone();
        let node = current[current.len() - 1];
        let tmp_c = graph[node as usize].clone();
        for i in tmp_c {
            let mut tmp_path = current.clone();
            tmp_path.push(i );
            if i == graph.len() as i32 - 1 {
                ret.push(tmp_path);
            } else {
                container.push_back(tmp_path);
            }
        }
    }

    return ret;
}

// fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
// dfs
//     fn dfs(graph: &Vec<Vec<i32>>, result: &mut Vec<Vec<i32>>, node: i32, path: &mut Vec<i32>) {
//         path.push(node);
//
//         if node == graph.len() as i32 - 1 {
//             result.push(path.clone());
//             return
//         }
//
//         let next_node = graph[node as usize].clone();
//         for i in next_node {
//             dfs(graph, result, i, path);
//             path.remove(path.len() - 1);
//         }
//
//     }
//
//     let mut ret = vec![];
//     let mut path = vec![];
//
//     dfs(&graph, &mut ret, 0, &mut path);
//
//     return ret;
// }

#[test]
fn test_all_paths_source_target() {
    let eg = vec![
        vec![1, 2],
        vec![3],
        vec![3],
        vec![]
    ];

    let ret = all_paths_source_target(eg);
    for i in ret {
        for j in i {
            print!("{}->", j);
        }
        println!();
    }
}

/**
* 884.
*/
fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut ret = vec![];

    let mut tmp = std::collections::HashMap::new();

    fn helper(dics: &mut std::collections::HashMap<String, i32>, msg: &String) {
        for word in msg.split(' ') {
            if dics.contains_key(word) {
                if let Some(s) = dics.get_mut(word) {
                    *s  += 1;
                }
                continue;
            }

            dics.insert(word.parse().unwrap(), 1);
        }
    }

    helper(&mut tmp, &s1);
    helper(&mut tmp, &s2);

    for (k,v) in tmp {
        if v == 1 {
            ret.push(k.to_string());
        }
    }

    return ret
}

#[test]
fn test_uncommon_from_sentences() {
    let ret = uncommon_from_sentences(String::from("this apple is sweet"), String::from("this apple is sour"));
    for i in 0..ret.len() {
        print!("{}\t", ret[i])
    }
}

/**
 * 944. delete columns to make sorted
 */
fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut ret = 0;

    let mut new_strs: Vec<Vec<char>> = Vec::new();

    for i in 0..strs.len() {
        new_strs.push(strs[i].chars().collect::<Vec<char>>());
    }

    for i in 0..new_strs[0].len() {
        for j in 1..new_strs.len() {
            if new_strs[j - 1][i] > new_strs[j][i] {
                ret += 1;
                break;
            }
        }
    }

    return ret;
}

#[test]
fn test_min_deletion_size() {
    let strs = vec![
        String::from("cba"),
        String::from("daf"),
        String::from("ghi"),
    ];
    let pos = min_deletion_size(strs);
    print!("{}", pos)
}

/**
 * 965. Univalued Binary Tree
 */
use std::collections::HashSet;
use std::collections::vec_deque::VecDeque;
fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut buf: HashSet<i32> = HashSet::new();

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, buf: &mut HashSet<i32>) {
        if let Some(node) = root {
            buf.insert(node.borrow().val);
            helper(&node.borrow().left, buf);
            helper(&node.borrow().right, buf);
        }
    }
    helper(&root, &mut buf);

    return buf.len() == 1 || buf.len() == 0;
}

#[test]
fn test_is_unival_tree() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    let ret = is_unival_tree(Some(root));
    print!("{}", ret)
}

/**
 * 969
 */
fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];

    let mut arr = arr;

    fn helper(arr: &mut Vec<i32>, container: &mut Vec<i32>) {
        if arr.len() == 0 {
            return;
        }

        let mut sorted = arr.clone();
        sorted.sort();
        if sorted == *arr {
            return;
        }

        fn get_max(arr: &Vec<i32>) -> i32 {
            let mut tmp = i32::MIN;
            let mut pos = 0;
            for i in 0..arr.len() {
                if arr[i] > tmp {
                    pos = i as i32;
                    tmp = arr[i];
                }
            }

            return pos;
        }

        fn reverse(arr: &mut Vec<i32>, end: usize) {
            let mut tmp = vec![];

            for i in 0..=end {
                tmp.push(arr[i]);
            }
            tmp.reverse();

            for i in 0..=end {
                arr[i] = tmp[i];
            }
            for i in arr {
                print!("{}\t", i)
            }
            println!();
        }

        let pos = get_max(arr);
        if pos != 0 {
            container.push(pos + 1);
            reverse(arr, pos as usize);
        }
        container.push(arr.len() as i32);
        reverse(arr, arr.len() - 1);
        let mut next = vec![];
        for i in 0..arr.len() - 1 {
            next.push(arr[i]);
        }
        helper(&mut next, container);
    }

    helper(&mut arr, &mut ret);
    return ret;
}

#[test]
fn test_pancake_sort() {
    let example = vec![3, 2, 4, 1];
    let ret = pancake_sort(example);
    for i in ret {
        print!("{}\t", i);
    }
}

/**
 * 1020.
 */
fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;

    fn helper(g: &mut Vec<Vec<i32>>, col: usize, row: usize) {
        if g[col][row] == 0 {
            return
        }

        g[col][row] = 0;

        if col == g.len() && row == g[0].len() {
            return;
        }

        // left
        if row > 0 {
            helper(g, col, row - 1);
        }

        // top
        if col > 0 {
            helper(g, col - 1, row);
        }

        // right
        if row < g[0].len() - 1 {
            helper(g, col, row + 1);
        }

        // bottom
        if col < g.len() - 1 {
            helper(g, col + 1, row);
        }
    }

    let height = grid.len();
    let width = grid[0].len();

    for i in 0..width {
        helper(&mut grid, 0, i);
        helper(&mut grid, height - 1, i);
    }

    for i in 0..height {
        helper(&mut grid, i, 0);
        helper(&mut grid, i, width - 1);
    }

    let mut ret = 0;
    for i in 0..height {
        for j in 0..width {
            ret += grid[i][j];
        }
    }

    return ret;
}

#[test]
fn test_num_enclaves() {
    // let example = vec![vec![0, 0, 0, 0], vec![1, 0, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]];
    let example = vec![vec![0], vec![1], vec![1], vec![0], vec![0]];
    let ret = num_enclaves(example);
    println!("{}", ret);
}

/**
* 1189.
*/
fn max_number_of_balloons(text: String) -> i32 {
    let mut container = HashMap::new();
    container.insert('b', 0);
    container.insert('a', 0);
    container.insert('l', 0);
    container.insert('o', 0);
    container.insert('n', 0);

    for i in text.chars() {
        if container.contains_key(&i) {
            *container.get_mut(&i).unwrap() += 1;
        }
    }

    let mut ret = i32::MAX;
    ret = ret.min(*container.get(&'b').unwrap());
    ret = ret.min(*container.get(&'a').unwrap());
    ret = ret.min(*container.get(&'l').unwrap()/2);
    ret = ret.min(*container.get(&'o').unwrap()/2);
    ret = ret.min(*container.get(&'n').unwrap());

    return ret;
}

#[test]
fn test_max_number_of_balloons() {
    let example = String::from("nlaebolko");
    let ret = max_number_of_balloons(example);
    println!("{}", ret);
}

/**
 * 1220.
 */
fn count_vowel_permutation(n: i32) -> i32 {
    let tmp: i64 = 1000000000 + 7;
    let mut dp = vec![1, 1, 1, 1, 1];
    let mut ndp = vec![0, 0, 0, 0, 0];

    for _ in 1..n {
        ndp[0] = (dp[1] + dp[2] + dp[4]) % tmp;
        ndp[1] = (dp[0] + dp[2]) % tmp;
        ndp[2] = (dp[1] + dp[3]) % tmp;
        ndp[3] = (dp[2]) % tmp;
        ndp[4] = (dp[2] + dp[3]) % tmp;
        dp = ndp.clone();
    }

    let mut ans = 0;
    for i in 0..5 {
        ans = (ans + dp[i]) % tmp;
    }
    return ans as i32;
}

#[test]
fn test_count_vowel_permutation() {
    let ret = count_vowel_permutation(144);
    print!("{}", ret);
}

/**
* 1332.
*/
fn remove_palindrome_sub(s: String) -> i32 {
    return if s == s.chars().rev().collect::<String>() {1} else {2};
}

#[test]
fn test_remove_palindrome_sub() {
    let s: String = String::from("ababa");
    let ret = remove_palindrome_sub(s);
    print!("{}\n", ret);
}

/**
* 1342.
*/
fn number_of_steps(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    let mut ret = 0;

    let mut num = num;
    loop {
        if num == 0 {
            break;
        }

        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = num - 1;
        }

        ret += 1;
    }

    return ret;
}

#[test]
fn test_number_of_steps() {
    let ret = number_of_steps(14);
    println!("{}", ret);
}

/**
* 1380
*/
fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut col = vec![0; n];
    let mut res = vec![];
    for j in 0..n {
        for i in 0..m {
            col[j] = col[j].max(matrix[i][j]);
        }
    }
    for x in 0..m {
        let mut i = 0;
        for y in 1..n  {
            if matrix[x][y] < matrix[x][i] {
                i = y;
            }
        }
        if matrix[x][i] == col[i] {
            res.push(col[i]);
        }
    }
    return res;
}

#[test]
fn test_lucky_numbers () {
    let example = vec![vec![7,8], vec![1,2]];
    let ret = lucky_numbers(example);
    println!("{}", ret[0]);
}

/**
* 1405.
*/
fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut ch = vec![(a, 'a'), (b, 'b'), (c, 'c')];
    let mut ans = vec![];
    loop {
        ch.sort_unstable();
        if ch[2].0 == 0 {
            break;
        }
        let len = ans.len();
        if len >= 2 && ans[len - 1] == ans[len - 2] && ans[len - 1] == ch[2].1 {
            if ch[1].0 > 0 {
                ch[1].0 -= 1;
                ans.push(ch[1].1);
            } else {
                ch[2].0 -= 1;
            }
        } else {
            ch[2].0 -= 1;
            ans.push(ch[2].1);
        }
    }
    ans.iter().collect()
}

#[test]
fn test_longest_diverse_string() {
    let ret = longest_diverse_string(0,8,11);
    print!("{}\n", ret);
}

/**
* 1423. Maximum Points You Can Obtain from Cards
*/
fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let mut init: i32 = card_points
        .iter()
        .skip(card_points.len() - k as usize)
        .sum();
    let mut res = init;
    for (sub, add) in card_points
        .iter()
        .skip(card_points.len() - k as usize)
        .zip(card_points.iter().take(k as usize))
    {
        init -= sub;
        init += add;
        if init > res {
            res = init;
        }
    }

    res
}

#[test]
fn test_max_score() {
    max_score(vec![1,2,3,4,5,6,1], 3);
}

/**
 * 1446. Consecutive Characters
 */
fn max_power(s: String) -> i32 {
    let mut max_length: i32 = 0;
    let mut res = 1;

    let mut pre_char = ' ';

    for c in s.chars() {
        match c == pre_char {
            true => {
                res = res + 1;
                max_length = max_length.max(res);
            }
            false => {
                res = 1;
                pre_char = c;
            }
        }
    }

    return max_length;
}

#[test]
fn test_max_power() {
    let ret = max_power(String::from("leetcode"));
    print!("{}", ret);
}

/**
* 1447.
*/
fn simplified_fractions(n: i32) -> Vec<String> {
    let mut ret = vec![];

    fn helper(a: i32, b :i32) -> i32 {
        return if b == 0 {a} else {helper(b, a % b)};
    }

    for i in 1..n+1 {
        for j in 1..i {
            println!("i: {}, j: {}", i, j);
            if helper(i, j) == 1 {
                ret.push(format!("{}/{}", j,i));
            }
        }
    }


    return ret;
}

#[test]
fn test_simplified_fractions() {
    let ret = simplified_fractions(4);
    for x in ret {
        print!("{}\t", x);
    }
}

/**
* 1584
*/
fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    // using Kruskal

    struct Node {
        x: usize,
        y: usize,
        distance: i32
    }

    let mut container: Vec<Node> = vec![];
    // calculate all distance
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let dis = i32::abs(points[i][0] - points[j][0]) + i32::abs(points[i][1]-points[j][1]);
            container.push(Node {x: i, y:j, distance:dis})
        }
    }

    container.sort_by(|a,b| a.distance.cmp(&b.distance));
    for i in 0..container.len() {
        println!("{}, {}: {}", container[i].x, container[i].y, container[i].distance);
    }

    let mut count = 0;
    let mut obj = OrderedUnion::new(points.len());
    let mut ret = 0;

    for i in 0..container.len() {
       if count == points.len() - 1 {
           break;
       }

       if obj.connected(container[i].x as i32, container[i].y as i32) {
           continue;
       }

       obj.union(container[i].x as i32, container[i].y as i32);
       ret += container[i].distance;
       count+=1;
    }

    ret
}


#[test]
fn test_min_cost_connect_points() {
    let obj = vec![
        vec![0,0],
        vec![2,2],
        vec![3, 10],
        vec![5, 2],
        vec![7,10]
    ];
    let ret = min_cost_connect_points(obj);
    println!("result: {}", ret);
}

/**
 * 1662. check if two string arrays are equivalent
 */
fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let s1: String = word1.into_iter().collect();
    let s2: String = word2.into_iter().collect();

    return s1 == s2;
}

#[test]
fn test_array_strings_are_equal() {
    let w1 = vec![String::from("ab"), String::from("c")];
    let w2 = vec![String::from("abc")];
    print!("{}", array_strings_are_equal(w1, w2))
}

/**
* 1678
*/
fn interpret(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}

#[test]
fn test_interpret() {
    let obj = String::from("G()(al)");
    let ret = interpret(obj);
    println!("{}", ret);
}

/**
* 1688.
*/
fn number_of_matches(n: i32) -> i32 {
    return n - 1;
}

#[test]
fn test_number_of_matches() {
    number_of_matches(11);
}


/**
 * 1716.
 */
fn total_money(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }

    let mut total = 1;
    let mut begin = 1;
    for i in 1..n {
        if i % 7 == 0 {
            begin += 1;
        }

        total += begin + i % 7;
    }
    return total;
}

#[test]
fn test_total_money() {
    let ret = total_money(10);
    assert_eq!(ret, 37);
}

/**
 * 1748.
 */
fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut container = vec![0; 100];

    for i in 0..nums.len() {
        container[nums[i] as usize - 1] += 1;
    }

    let mut sum = 0;
    for i in 0..100 {
        if container[i] == 1 {
            sum += i + 1;
        }
    }
    return sum as i32;
}

#[test]
fn test_sum_of_unique() {
    let example = vec![1, 2, 3, 4, 5];
    let ret = sum_of_unique(example);
    print!("ret: {}\n", ret);
}

/**
 * 1763.
 */
fn longest_nice_substring(s: String) -> String {
    let mut upper_letter = vec![false; 26];
    let mut lower_letter = vec![false; 26];

    for i in s.chars() {
        if i.is_ascii_uppercase() {
            upper_letter[i as usize - 'A' as usize] = true;
        } else {
            lower_letter[i as usize - 'a' as usize] = true;
        }
    }

    for (i, ch) in s.chars().enumerate() {
        if upper_letter[ch.to_ascii_uppercase() as usize - 'A' as usize] == true && lower_letter[ch.to_ascii_lowercase() as usize - 'a' as usize] == true {
            continue;
        }

        let s1 = longest_nice_substring(s[0..i].to_string());
        let s2 = longest_nice_substring(s[i + 1..].to_string());
        return if s1.len() >= s2.len() { s1 } else { s2 };
    }

    return s;
}

#[test]
fn test_longest_nice_substring() {
    let example = String::from("YazaAay");
    let ret = longest_nice_substring(example);
    println!("{}", ret);
}


/**
 * 1765.
 */
fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = is_water.len();
    let n = is_water[0].len();

    let mut ret = vec![vec![0; n]; m];
    let mut container = VecDeque::new();

    let mut visited = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if is_water[i][j] == 1 {
                visited[i][j] = true;
                container.push_back((i, j));
            }
        }
    }

    let mut target = 1;
    while !container.is_empty() {
        for _ in 0..container.len() {
            let (i, j) = container.pop_front().unwrap();
            for (x, y) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if x < m && y < n && !visited[x][y] {
                    ret[x][y] = target;
                    visited[x][y] = true;
                    container.push_back((x, y));
                }
            }
        }

        target = target + 1;
    }

    return ret;
}

#[test]
fn test_highest_peak() {
    // let is_water = vec![vec![0,1],vec![0,0]];
    // highest_peak(is_water);
}

/**
* 1779
*/

fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    // let mut dis = i32::MAX;
    // let mut ret:i32 = -1;

    // for i in 0..points.len() {
    //     if x == points[i][0] || y == points[i][1] {
    //         let target = (x-points[i][0]).abs() + (y-points[i][1]).abs();
    //         if target < dis {
    //             dis = target;
    //             ret = i as i32;
    //         }
    //     }
    // }

    // return ret;

    points
        .iter()
        .enumerate()
        .filter(|(_, p)| p[0] == x || p[1] == y)
        .map(|(i, p)| (i, (p[0] - x).abs() + (p[1] - y).abs()))
        .min_by_key(|&(_, d)| d)
        .map_or(-1, |(i, _)| i as i32)
}

#[test]
fn test_nearest_valid_point() {
    let x = 3;
    let y = 4;
    let points = vec![vec![1,2], vec![3,1],vec![2,4],vec![2,3],vec![4,4]];

    println!("{}", nearest_valid_point(x,y,points))
}
/**
* 1791.
*/
fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let mut container = HashMap::new();

    for i in edges {
        for j in i {
            if container.contains_key(&j) {
                return j;
            }

            container.insert(j, true);

        }
    }

    return 0;
}

#[test]
fn test_find_center() {
    let example = vec![vec![1,2], vec![1,3]];
    let ret = find_center(example);
    println!("{}", ret);
}

/**
 * 1796. Second Largest Digit in a String
 */
fn second_highest(s: String) -> i32 {
    let mut ar = [false; 10];

    s.as_bytes()
        .iter()
        .filter(|x| x.is_ascii_digit())
        .map(|x| x - b'0')
        .for_each(|x| ar[x as usize] = true);

    ar.iter()
        .enumerate()
        .filter_map(|(ind, val)| if *val { Some(ind as i32) } else { None })
        .rev()
        .nth(1)
        .unwrap_or(-1)
}

#[test]
fn test_second_highest() {
    second_highest(String::from("1234"));
}

/**
 * 1984.
 */
fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    if k == 1 {
        return 0;
    }

    let mut nums = nums;
    nums.sort();

    let mut ret = i32::MAX;

    for i in 0..(nums.len() - k as usize + 1) {
        let tmp = nums[i + k as usize - 1] - nums[i];
        ret = if tmp < ret { tmp } else { ret };
    }

    return ret;
}

#[test]
fn test_minimum_difference() {
    let example = vec![9, 4, 7, 1];
    let ret = minimum_difference(example, 2);
    println!("{}", ret);
}

/**
 * 2000
 */
fn reverse_prefix(word: String, ch: char) -> String {
    let pos = word.find(ch);
    if pos == None {
        return word;
    }

    return word[0..pos.unwrap() + 1].chars().rev().collect::<String>() + &word[(pos.unwrap() + 1)..];
}

#[test]
fn test_reverse_prefix() {
    let example = String::from("abcdef");
    let ret = reverse_prefix(example, 'd');
    println!("{}", ret);
}

/**
* 2006.
*/
fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut mp = HashMap::new();
    let mut ans = 0;
    nums.iter().for_each(|x| {
        ans += *mp.get(&(*x + k)).unwrap_or(&0) + *mp.get(&(*x-k)).unwrap_or(&0);
        *mp.entry(*x).or_insert(0) += 1;
    });
    ans
}

#[test]
fn test_count_k_difference() {
    let example = vec![3,2,1,5,4];
    // let example = vec![1,2,2,1];
    let ret = count_k_difference(example, 2);
    println!("{}", ret);
}

/**
 * 2016
 */
fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut ret = -1;

    let mut right_max = nums[nums.len() - 1 as usize];
    for i in (0..nums.len() - 1).rev() {
        if nums[i] >= right_max {
            right_max = std::cmp::max(nums[i], right_max);
        } else {
            ret = std::cmp::max(ret, right_max - nums[i]);
        }
    }

    return ret;
}

#[test]
fn test_maximum_difference() {
    let example = vec![999, 997, 980, 976, 948, 940, 938, 928, 924, 917, 907, 907, 881, 878, 864, 862, 859, 857, 848, 840, 824, 824, 824, 805, 802, 798, 788, 777, 775, 766, 755, 748, 735, 732, 727, 705, 700, 697, 693, 679, 676, 644, 634, 624, 599, 596, 588, 583, 562, 558, 553, 539, 537, 536, 509, 491, 485, 483, 454, 449, 438, 425, 403, 368, 345, 327, 287, 285, 270, 263, 255, 248, 235, 234, 224, 221, 201, 189, 187, 183, 179, 168, 155, 153, 150, 144, 107, 102, 102, 87, 80, 57, 55, 49, 48, 45, 26, 26, 23, 15];
    let ret = maximum_difference(example);
    println!("{}", ret);
}

/**
 * find max(A[i] - A[j]) and  i < j
 */
fn find_max_distance(numbers: Vec<i32>) -> i32 {
    let mut ret = i32::MIN;

    // solution 2, O(n)
    let mut left_max = vec![0; numbers.len()];
    let mut right_min = vec![0; numbers.len()];

    left_max[0] = numbers[0];
    for i in 1..numbers.len() {
        left_max[i] = std::cmp::max(left_max[i - 1], numbers[i])
    }

    for i in numbers.len() - 1..1 {
        right_min[i] = std::cmp::min(right_min[i + 1], numbers[i]);
    }

    for i in 0..numbers.len() {
        ret = std::cmp::max(ret, left_max[i] - right_min[i]);
    }

    return ret;
}

#[test]
fn test_find_max_distance() {
    let numbers = vec![1, 2, 7, 4, 5, 10, 6, 7, 8, 9, 2, 3, 1, 0];
    let ret = find_max_distance(numbers);
    assert_eq!(ret, 10);
}

fn main() {
    println!("hello ans");
}
