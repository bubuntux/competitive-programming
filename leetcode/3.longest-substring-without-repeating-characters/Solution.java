import java.util.HashMap;
import java.util.Map;

class Solution {
    public int lengthOfLongestSubstring(String s) {
        int ans = 0;
        int left = 0;
        Map<Character, Integer> lastIndexes = new HashMap<>();
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            int lastIndex = lastIndexes.getOrDefault(c, -1);
            if (lastIndex >= left) {
                ans = Math.max(ans, right - left);
                left = lastIndex + 1;
            }
            lastIndexes.put(c, right);
        }
        return Math.max(ans, s.length() - left);
    }
}

// https://leetcode.com/problems/longest-substring-without-repeating-characters/

class Tester {
    private final static Solution solution = new Solution();

    public static void main(String... args) {
        test(3, "abcabcbb");
        test(1, "bbbbb");
        test(3, "pwwkew");
        test(0, "");
        test(1, " ");
        test(3, "dvdf");
    }

    private static void test(int expect, String input) {
        int got = solution.lengthOfLongestSubstring(input);
        if (got != expect) {
            throw new AssertionError('"' + input + "\" Expected " + expect + " got " + got);
        }
    }

}