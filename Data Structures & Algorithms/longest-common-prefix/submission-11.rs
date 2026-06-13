impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);

        for i in 0..min_len {
            for j in 1..strs.len() {
                if strs[j].as_bytes()[i] != strs[0].as_bytes()[i] {
                    return prefix;
                }
            }
            prefix.push(strs[0].as_bytes()[i] as char);
        }

        prefix
    }
}