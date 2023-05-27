#![warn(dead_code)]
#![allow(unused)]

use std::cmp::max;

pub fn clean_array(array: &mut Vec<usize>) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::with_capacity(array.len());
    storage.extend_from_slice(array);
    storage.sort();
    storage.dedup();
    storage.shrink_to_fit();

    storage
}

pub fn merge_two_arrays_arrange_and_clean(array1: &mut Vec<usize>, array2: &mut Vec<usize>) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::with_capacity(array1.len() + array2.len());
    storage.extend_from_slice(array1);
    storage.extend_from_slice(array2);
    storage.sort();
    storage.dedup();
    storage.shrink_to_fit();
    storage
}

pub fn number_of_factors_one_integer(factor: u32, until: u32) -> u32 {
    until / factor
}

pub fn sum_array_items(array: &mut Vec<usize>) -> usize {
    let mut sum = 0;
    for (index, item) in array.clone().iter().enumerate() {
        sum = sum + array[index];
    }
    sum
}

pub fn sum_of_even_array_items(array: &mut Vec<usize>) -> usize {
    let mut sum = 0;
    for (index, item) in array.clone().iter().enumerate() {
        if item % 2 == 0 {
            sum = sum + array[index];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_arrays_test() {
        let mut array_x: Vec<usize> = vec![3, 6, 9, 12, 15, 18];
        let mut array_y: Vec<usize> = vec![5, 10, 15];
        let result: Vec<usize> = merge_two_arrays_arrange_and_clean(&mut array_x, &mut array_y);
        assert!(
            result == vec![3, 5, 6, 9, 10, 12, 15, 18]
        );
    }

    #[test]
    fn euler1_number_of_factors_test() {
        assert!(
            number_of_factors_one_integer(3, 20) == 6
        );
    }

    #[test]
    fn clean_array_test() {

        let mut array: Vec<usize> = vec![1, 1, 2, 8, 5, 5, 13, 3];
        let correct_array: Vec<usize> = vec![1, 2, 3, 5, 8, 13];

        assert!(
            clean_array(&mut array) == correct_array);
    }

    #[test]
    fn sum_array_items_test() {
        assert!(
            sum_array_items(&mut vec![1, 3, 6, 11]) == 21
        )
    }

    #[test]
    fn sum_of_even_array_items_test() {
        assert!(
            sum_of_even_array_items(&mut vec![1, 3, 6, 11]) == 6
        )
    }

}