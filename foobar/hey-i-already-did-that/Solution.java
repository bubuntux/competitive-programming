import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Solution {
    public static int solution(final String n, final int b) {
        final Map<String, Integer> zIteration = new HashMap<>();

        String z = n;
        int iteration = 0;
        do {
            zIteration.put(z, iteration++);
            z = getNextValue(z, b);
        } while (!zIteration.containsKey(z));

        return iteration - zIteration.get(z);
    }

    static String getNextValue(final String n, final int b) {
        final char[] nChars = n.toCharArray();
        Arrays.sort(nChars);
        final String y = new String(nChars);
        reverse(nChars);
        final String x = new String(nChars);

        final int k = n.length();
        return String.format("%" + k + "s", Integer.toString(Integer.parseInt(x, b) - Integer.parseInt(y, b), b))
                .replaceAll(" ", "0");
    }

    static void reverse(final char[] chars) {
        final int size = chars.length;
        for (int i = 0; i < size / 2; i++) {
            final char temp = chars[i];
            chars[i] = chars[size - i - 1];
            chars[size - i - 1] = temp;
        }
    }
}