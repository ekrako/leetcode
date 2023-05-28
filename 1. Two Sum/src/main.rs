#[allow(dead_code)]
// Code Snippet: https://leetcode.com/problems/two-sum/
// imperative approach with O(n^2) time complexity
#[allow(dead_code)]
fn two_sum_naive_solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

// imperative approach with O(n^2) time complexity using enumerate
fn two_sum_with_enumerate(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i,v) in nums.iter().enumerate() {
        for (j,u) in nums[i+1..].iter().enumerate() {
            if u + v == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

// imperative approach with O(n) time complexity using hash map
fn two_sum_with_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i,v) in nums.iter().enumerate() {
        let complement = target - v;
        if map.contains_key(&complement) {
            return vec![*map.get(&complement).unwrap() as i32, i as i32];
        }
        map.insert(v, i);
    }
    vec![]
}

// solution using itertools crate with O(n^2) time complexity
fn two_sum_itertools_naive_solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use itertools::Itertools;
    let (location,_) = nums.iter().combinations(2).enumerate().find(|(_,v)| v[0] + v[1] == target).unwrap();
    (0..nums.len() as i32)
    .into_iter()
    .collect::<Vec<i32>>()
    .into_iter()
    .combinations(2)
    .nth(location)
    .unwrap()
}

// solution using itertools crate with O(n) time complexity
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use itertools::Itertools;
    nums.into_iter().enumerate().combinations(2)
        .map(|item| {
            (
                item.iter().fold(0, |acc, (_, v)| acc + v),
                item.into_iter().map(|(i, _)| i as i32),
            )
        })
        .find(|(x, _)| x == &target)
        .unwrap()
        .1
        .collect()
}

fn main() {
    let nums = vec![11, 15, 2, 7, 34];
    let target = 9;
    println!("{:?}", two_sum(nums, target));
}
