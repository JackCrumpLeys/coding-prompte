#![feature(int_roundings)]

use std::fmt::format;
use std::ops::{Add, Range};
use std::str::Chars;
use std::thread::scope;

fn main() {}

#[test]
fn military_time() {
    let military_time: i32 = 2300;
    let hours: i32 = military_time.div_floor(100);
    let minutes: i32 = military_time - hours * 100;
    let secs_total: i32 = hours * 60 * 60 + minutes * 60;
    let secs_til_midnight: i32 = 86400 - secs_total;

    println!("{}", secs_til_midnight);
    assert_eq!(secs_til_midnight, 3600)
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode::new(1))),
            Some(Box::new(ListNode::new(1)))
        ),
        Some(Box::new(ListNode::new(2)))
    );
}

#[test]
fn test_roman_to_int() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    println!("{}", Solution::roman_to_int("JJJJJJJ".to_string()).to_string());
}

#[test]
fn test_longest_common_prefix() {
    assert_eq!(Solution::longest_common_prefix(vec!["iiii".to_string(),"ii".to_string()]), "ii".to_string());
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Numeral {
    letter: char,
    number: i128,
}

impl Numeral {
    fn new(letter: char, number: i128) -> Self {
        Self { letter, number }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut num_one = &mut l1.unwrap();
        let mut num_two = &mut l2.unwrap();

        let mut carry: bool = false;

        let mut added = num_one.val + num_two.val;

        if added >= 10 {
            added -= 10;
            carry = true;
        }

        let mut list = Box::new(ListNode::new(added));
        let mut indexed_list: &mut Box<ListNode> = &mut list;

        loop {
            let mut digit: Option<i32> = None;
            if carry {
                digit = Some(1);
                carry = false;
            }
            if num_one.next.is_some() || num_two.next.is_some() {
                num_one = num_one.next.get_or_insert(Box::new(ListNode::new(0)));
                num_two = num_two.next.get_or_insert(Box::new(ListNode::new(0)));
                let mut added = num_one.val + num_two.val + *digit.get_or_insert(0);

                if added >= 10 {
                    added -= 10;
                    carry = true;
                }
            } else if num_one.next.is_some() {
                *digit.get_or_insert(0) += num_one.next.clone().unwrap().val;
            } else if num_two.next.is_some() {
                *digit.get_or_insert(0) += num_two.next.clone().unwrap().val;
            }
            if let Some(int) = digit {
                indexed_list.next = Some(Box::new(ListNode::new(int)));
                indexed_list = indexed_list.next.as_mut().unwrap()
            } else {
                break;
            }
        }

        return Some(list);
    }

    pub fn roman_to_int(s: String) -> i128 {
        let numerals = [
            Numeral::new('I', 1),
            Numeral::new('V', 5),
            Numeral::new('X', 10),
            Numeral::new('L', 50),
            Numeral::new('C', 100),
            Numeral::new('D', 500),
            Numeral::new('M', 1000),
            Numeral::new('J', 1000000000000000000009000000000000000)
        ];

        let numeral_array = s
            .chars()
            .map(|char| numerals.iter().find(|x| x.letter == char).unwrap())
            .collect::<Vec<&Numeral>>();

        let mut skip = false;
        let mut number = 0;
        for (i, numeral) in numeral_array.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }
            if numeral.number < numeral_array[(i + 1).clamp(0, numeral_array.len() - 1)].number {
                skip = true;
                number += numeral_array[i + 1].number - numeral.number;
                println!("{}{} = {}", numeral.letter, numeral_array[i + 1].letter, numeral_array[i + 1].number - numeral.number);
            } else {
                number += numeral.number;
                println!("{} = {}", numeral.letter, numeral.number);
            }
        }

        number
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut shortest_len = 200usize;
        for str in &strs {
            if str.len() < shortest_len {
                shortest_len = str.len()
            }
        }

        let str_chars = &strs.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let mut prefix = String::new();
        for i in 0..shortest_len {
            let mut letter:Option<char> = None;
            let mut done = false;
            for str in str_chars {
                if letter == None {
                    letter = Some(str[i]);
                }
                if letter.unwrap() != str[i] {
                    done = true;
                    break
                }
            }
            if done {
                break
            } else {
                prefix = prefix + &*letter.unwrap().to_string()
            }
        }
        prefix
    }
}
