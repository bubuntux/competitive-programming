import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

class Solution {
    public List<String> subdomainVisits(String[] cpdomains) {
        Map<String, Integer> counts = new HashMap<>();

        for (String cpdomain : cpdomains) {
            int index = cpdomain.indexOf(" ");
            if (index < 0) continue;
            int clicks = Integer.parseInt(cpdomain.substring(0, index));
            countVisits(counts, clicks, cpdomain.substring(index + 1));
        }

        return counts.entrySet().stream().map(entry -> String.format("%d %s", entry.getValue(), entry.getKey())).sorted().collect(Collectors.toList());
    }

    private void countVisits(Map<String, Integer> counts, int count, String domain) {
        if (domain == null || domain.length() == 0) return;
        int currentCount = counts.getOrDefault(domain, 0);
        counts.put(domain, currentCount + count);

        int dotIndex = domain.indexOf(".");
        if (dotIndex < 0) return;

        String subDomain = domain.substring(dotIndex + 1);
        countVisits(counts, count, subDomain);
    }
}

///
class Tester {
    private final static Solution solution = new Solution();

    public static void main(String... args) {
        test("9001 discuss.leetcode.com,9001 leetcode.com,9001 com", "9001 discuss.leetcode.com");
    }

    private static void test(String expect, String... input) {
        List<String> strings = solution.subdomainVisits(input);
        System.out.println('[' + expect + "] = " + strings.stream().collect(Collectors.joining(",", "[", "]")));
    }

}