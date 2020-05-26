import java.util.*;
import java.util.stream.Collectors;

class Solution {

    private static final Map<Integer, Set<Character>> _values = new HashMap<>();

    static {
        _values.put(1, Collections.unmodifiableSet(new HashSet<>(Arrays.asList('e', 'a', 'i', 'o', 'n', 'r', 't', 'l', 's', 'u'))));
        _values.put(2, Collections.unmodifiableSet(new HashSet<>(Arrays.asList('d', 'g'))));
        _values.put(3, Collections.unmodifiableSet(new HashSet<>(Arrays.asList('b', 'c', 'm', 'p'))));
        _values.put(4, Collections.unmodifiableSet(new HashSet<>(Arrays.asList('f', 'h', 'v', 'w', 'y'))));
        _values.put(5, Collections.unmodifiableSet(new HashSet<>(Collections.singletonList('k'))));
        _values.put(8, Collections.unmodifiableSet(new HashSet<>(Arrays.asList('j', 'x'))));
        _values.put(10, Collections.unmodifiableSet(new HashSet<>(Arrays.asList('q', 'z'))));
    }

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);
        final int N = in.nextInt();
        in.nextLine();
        List<Word> words = new LinkedList<>();
        for (int i = 0; i < N; i++) {
            final String W = in.nextLine();
            final Word word = new Word(W);
            System.err.println(word);
            words.add(word);
        }
        final String LETTERS = in.nextLine();
        System.err.println(LETTERS);
        final String bestWord = words.stream().filter(w -> w.containsAll(LETTERS)).max((o1, o2) -> Integer.compare(o1.value, o2.value)).get().getWord();
        System.out.println(bestWord);
    }

    private static class Word {
        final List<Character> word;
        final int value;

        private Word(String w) {
            word = Collections.unmodifiableList(w.chars().mapToObj(c -> (char) c).collect(Collectors.toList()));
            value = word.stream().mapToInt(letter ->
                    _values.entrySet().stream().filter(entry -> entry.getValue().contains(letter)
                    ).findFirst().get().getKey()).sum();
        }

        @Override
        public String toString() {
            return getWord() + " " + value;
        }

        private String getWord() {
            return word.stream().map(String::valueOf).collect(Collectors.joining());
        }

        private boolean containsAll(String letters) {//Can be optimized sorting the words collections
            LinkedList<Character> characters = letters.chars().mapToObj(c -> (char) c).
                    collect(Collectors.toCollection(LinkedList::new));
            return word.stream().allMatch(characters::removeFirstOccurrence);
        }
    }
}