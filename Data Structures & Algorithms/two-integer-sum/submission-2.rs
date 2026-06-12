impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];

            if seen.contains_key(&complement) {
                return vec![*seen.get(&complement).unwrap(), i as i32];
            }

            seen.insert(nums[i], i as i32);
        }

        vec![]
    }
}
