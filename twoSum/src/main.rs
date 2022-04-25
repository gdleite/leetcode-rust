use std::{collections::HashMap};

fn main() {
    println!("{:?}", two_sum_brute_force(vec![2,7,1,15], 9));
    println!("{:?}", two_sum_hashmap(vec![2,7,1,15], 9));
}


pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i+1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    vec![]
}

pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map : HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        map.insert(*num, i as i32);
    }

    for (i, num) in nums.iter().enumerate() {
        let aux= target - num;

        if let Some(&index) = map.get(&aux) {
            if index != i as i32 {
                return vec![i as i32, index];
            }
        }

    }
    vec![]
}

