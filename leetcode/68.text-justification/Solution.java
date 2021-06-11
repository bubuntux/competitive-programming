import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

class Solution {
    public List<String> fullJustify(String[] words, int maxWidth) {
        List<String> result = new LinkedList<>();

        List<String> buffer = new ArrayList<>();
        int counter = 0;
        for (String word : words) {
            if (counter + word.length() > maxWidth) {
                result.add(justify(buffer, counter, maxWidth));
                buffer.clear();
                counter = 0;
            }
            buffer.add(word);
            counter += word.length() + 1;
        }

        result.add(leftJustify(buffer, maxWidth));

        return result;
    }

    private String justify(List<String> buffer, int counter, int maxWidth) {
        if (buffer.size() == 1) {
            return leftJustify(buffer, maxWidth);
        }

        int spacesToAdd = maxWidth - counter + buffer.size();
        int spacesBetweenWords = spacesToAdd / (buffer.size() - 1);
        int extra = spacesToAdd % (buffer.size() - 1);
        String spaces = String.format("%" + spacesBetweenWords + "s", "");
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < buffer.size(); i++) {
            sb.append(buffer.get(i));
            if (i < buffer.size() - 1) {
                sb.append(spaces);
            }
            if (extra > 0) {
                sb.append(' ');
                extra--;
            }
        }
        return sb.toString();
    }

    private String leftJustify(List<String> buffer, int maxWidth) {
        return String.format("%-" + maxWidth + "s", String.join(" ", buffer));
    }
}

///
class Tester {
    private final static Solution solution = new Solution();

    public static void main(String... args) {
        solution.fullJustify(new String[]{"This", "is", "an", "example", "of", "text", "justification."}, 16).stream().map(s -> '\"' + s + '\"').forEach(System.out::println);
        System.out.println();
        solution.fullJustify(new String[]{"What", "must", "be", "acknowledgment", "shall", "be"}, 16).stream().map(s -> '\"' + s + '\"').forEach(System.out::println);
        System.out.println();
        solution.fullJustify(new String[]{"Science", "is", "what", "we", "understand", "well", "enough", "to", "explain", "to", "a", "computer.", "Art", "is", "everything", "else", "we", "do"}, 20).stream().map(s -> '\"' + s + '\"').forEach(System.out::println);
    }

}
