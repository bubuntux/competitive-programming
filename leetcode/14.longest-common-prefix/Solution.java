import java.util.Arrays;

//https://leetcode.com/problems/longest-common-prefix/
class Solution {

    public String longestCommonPrefix(String[] strs) {
        Arrays.sort(strs);
        final String first = strs[0];
        final String last = strs[strs.length - 1];

        final int minLength = Math.min(first.length(), last.length());
        final StringBuilder result = new StringBuilder();
        for (int i = 0; i < minLength; i++) {
            final char fc = first.charAt(i);
            if (fc != last.charAt(i)) break;
            result.append(fc);
        }
        return result.toString();
    }
}

