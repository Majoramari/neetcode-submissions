class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.size() != t.size()) return false;

        unordered_map<char, int> sMap;
        unordered_map<char, int> tMap;

        for (char c : s) {
            sMap[c]++;
        }

        for (char c : t) {
            tMap[c]++;
        }

        for (auto [c, count] : sMap) {
            if (tMap[c] != count) return false;
        }

        return true;
    }
};
