fn main() {
    let nums  = vec![1,2,3];
    let target = 5;
    let solution = Solution::search_insert(nums, target);
    println!("{}", solution)
}

struct Solution {

}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, v) in nums.iter().enumerate() {
            if target <= *v {
                return i as i32 // return the last one.
            }
        }
        nums.len() as i32 // return the last one.
    }
    // withoud ';', we could return the last value as the function output.
}
