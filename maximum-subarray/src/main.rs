fn main() {
    dbg!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut n = 0;
    let mut max = i32::MIN;
    for i in nums {
        n = n + i;
        if n > max {
            max = n
        }
        if n < 0 {
            n = 0
        } 
    }
    return max;
}
