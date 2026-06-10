#include <unordered_set>

class Solution {
public:
    bool hasDuplicate(vector<int>& nums) {
        unordered_set<int> seen;

        for (int num : nums) {
            auto [it, inserted] = seen.insert(num);
            if(!inserted) return true;
        }
        
        return false;
    }
};