
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
    let mut v: Vec<i32> = Vec::new();
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
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
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
    l1.next.as_mut().unwrap().next= Some(Box::new(ListNode::new(7)));

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
    let mut ret: f64 = 0.0;

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
            std::i32::MIN
        } else {
            short[part_x - 1]
        };

        let short_right = if part_x == short.len() {
            std::i32::MAX
        } else {
            short[part_x]
        };

        // get the value in long array
        let long_left = if part_y == 0 {
            std::i32::MIN
        } else {
            long[part_y - 1]
        };

        let long_right = if part_y == long.len() {
            std::i32::MAX
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

    for i in 0..size {
        flags[i][i] = true;

        for j in 0..i {
            flags[j][i] = (b[j] == b[i] && (i-j < 3 || flags[j+1][i-1]));
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
    let demo1 = String::from("abcda");
    let ret = longest_palindrome(demo1);
    assert_eq!(ret, "a");
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

    if reverse_num > std::i32::MAX as i64 || reverse_num < std::i32::MIN as i64 {
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
    let mut ret: i64  = 0;
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
            if ret > std::i32::MAX as i64 {
                break;
            }
            continue;
        }

        break;
    }

    if is_negative {
        ret = 0 - ret;
    }

    if ret > std::i32::MAX as i64 {
        return std::i32::MAX;
    }

    if ret < std::i32::MIN as i64 {
        return std::i32::MIN;
    }

    return ret as i32;
}

#[test]
fn test_my_atoi() {
    let s = String::from("9223372036854775808");
    assert_eq!(my_atoi(s), std::i32::MAX);
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

    return tmp == recv || tmp == recv/10;
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
    let demo = vec![1,8,6,2,5,4,8,3,7];
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
    let mut table = vec![(1, "I"),
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
    (1000, "M")
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

    let mut val: i32 = 0;
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
    let demo = vec![String::from("flower"), String::from("flow"), String::from("flight")];
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
        let mut v:Vec<Vec<i32>> = Vec::new();

        let mut m = HashMap::new();

        for (index, value) in numbers.iter().enumerate() {
            let num = target - *value;
            if m.contains_key(&num) {
                v.push(vec![m[&num], index as i32]);
            }
            m.insert(value, index as i32);
        }

        return v;
    };

    let pos = nums.len() - 2;

    for i in 0..pos {
        if i != 0 && nums[i] == nums[i-1] {
            continue;
        }

        let ret = two_sum_in(nums[i+1..nums.len()].to_vec(), 0 - nums[i]);
        if ret.len() > 0 {
            for j in &ret {
                println!("{} : {} : {}", nums[i], nums[i + 1 + j[0] as usize], nums[i + 1 + j[1] as usize]);
                result.push(vec![nums[i], nums[i + 1 + j[0] as usize], nums[i + 1 + j[1] as usize]]);
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

            if (tmp - target).abs() < (ret -target).abs() {
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
    assert_eq!(three_sum_closest(demo,1), 2);
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
            paths[i][j] = paths[i-1][j] + paths[i][j-1];
        }
    }

    return paths[m-1][n-1] as i32;        
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
        min_paths[i][0] = min_paths[i-1][0] + min_paths[i][0];
    }

    for i in 1..n {
        min_paths[0][i] = min_paths[0][i-1] + min_paths[0][i];
    }

    for i in 1..m {
        for j in 1..n {
            min_paths[i][j] = min_paths[i][j] + cmp::min(min_paths[i-1][j], min_paths[i][j-1]);
        }
    }

    return min_paths[m - 1][n - 1]; 
}

#[test]
fn test_in_path_sum() {
    let grid:Vec<Vec<i32>> = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
    assert_eq!(min_path_sum(grid), 7);
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
    let mut n = word1.len();
    let mut m = word2.len();

    let mut steps = vec![vec![0; n+1]; m+1];

    for i in 0..m+1 {
        steps[i][0] = i;
    }

    for i in 0..n+1 {
        steps[0][i] = i;
    }

    for i in 0..m+1 {
        println!("{:?}", steps[i]);
    }
    println!("");
    println!("");

    let b1 = word1.as_bytes();
    let b2 = word2.as_bytes();

    for i in 0..m {
        for j in 0..n {
            if b1[j] == b2[i] {
                steps[i+1][j+1] = steps[i][j];
            } else {
                steps[i+1][j+1] = std::cmp::min(std::cmp::min(steps[i][j], steps[i+1][j]), steps[i][j+1]) + 1;
            }

        }

        for i in 0..m+1 {
            println!("{:?}", steps[i]);
        }

        println!("");
    }


    return steps[m][n] as i32;    
}

#[test]
fn test_min_distance() {
    let word1 = String::from("horse");
    let word2 = String::from("ros");

    assert_eq!(min_distance(word1, word2 ), 3);
}

fn main() {
    println!("the answer of leetcode.com using rust");
}
