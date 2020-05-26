import java.util.Stack;

class Solution {

    public String removeKdigits(String input, int k) {
        if (input == null || input.length() <= k) {
            return "0";
        }

        var stack = new Stack<Character>();

        for (char c : input.toCharArray()) {
            while (k > 0 && !stack.isEmpty() && stack.peek() > c) {
                stack.pop();
                k--;
            }
            stack.push(c);
        }

        while (k > 0) {
            stack.pop();
            k--;
        }

        boolean removeZero = true;
        var sb = new StringBuilder();
        for (Character c : stack) {
            if (removeZero && Character.getNumericValue(c) == 0) {
                continue;
            }
            removeZero = false;
            sb.append(c);
        }

        if (sb.length() == 0) {
            return "0";
        }

        return sb.toString();
    }

}