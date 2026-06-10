use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<_> = nums.iter().collect();
        set.len() != nums.len()
    }
}
