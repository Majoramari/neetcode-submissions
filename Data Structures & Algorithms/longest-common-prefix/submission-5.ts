class Solution {
    /**
     * @param {string[]} strs
     * @return {string}
     */
    longestCommonPrefix(strs: string[]): string {
        let prefix: string = "";

        for (let i = 0; i < strs[0].length; i++) {
            const char = strs[0][i];

            for (let j = 1; j < strs.length; j++) {
                if (strs[j][i] !== char) {
                    return prefix;
                }
            }

            prefix += char;
        }

        return prefix;
    }
}
