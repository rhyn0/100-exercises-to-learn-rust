// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let static_ref = v.leak();
    let midpoint = static_ref.len() / 2;

    let first_half = &static_ref[..midpoint];
    let second_half = &static_ref[midpoint..];

    let handle_one = thread::spawn(|| first_half.iter().sum::<i32>());
    let handle_two = thread::spawn(|| second_half.iter().sum::<i32>());

    let result_one = handle_one.join().unwrap();
    let result_two = handle_two.join().unwrap();

    result_one + result_two
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
