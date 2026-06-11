class Solution {
    /**
     * @param {string} s
     * @param {string} t
     * @return {boolean}
     */
    isAnagram(s: string, t: string): boolean {
        if (s.length !== t.length) return false;

        const sMap = new Map<string, number>();
        const tMap = new Map<string, number>();

        for (const char of s) {
            sMap.set(char, (sMap.get(char) || 0) + 1);
        }
        for (const char of t) {
            tMap.set(char, (tMap.get(char) || 0) + 1);
        }

        for (const [char, count] of sMap) {
            if (tMap.get(char) !== count) return false;
        }

        return true;
    }
}
