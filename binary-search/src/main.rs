fn main() {
    let vec1 = vec![-1,0,3,5,9,12];
    dbg!(search(vec1, 9));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = nums[mid_index];
        if val == target {
            return mid_index as i32;
        }
        if val < target {
            low = mid + 1;
        }
        if val > target {
            high = mid - 1;
        }
    }
    return -1
}
