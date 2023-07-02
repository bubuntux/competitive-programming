import java.util.*;
import java.util.function.IntUnaryOperator;

public class Solution {
    public static void main(String... args) {
        final Monkey[] monkeys = {
                new Monkey(Arrays.asList(75, 63), old -> old * 3, value -> value % 11 == 0 ? 7 : 2),
                new Monkey(Arrays.asList(65, 79, 98, 77, 56, 54, 83, 94), old -> old + 3, value -> value % 2 == 0 ? 2 : 0),
                new Monkey(List.of(66), old -> old + 5, value -> value % 5 == 0 ? 7 : 5),
                new Monkey(Arrays.asList(51, 89, 90), old -> old * 19, value -> value % 7 == 0 ? 6 : 4),
                new Monkey(Arrays.asList(75, 94, 66, 90, 77, 82, 61), old -> old + 1, value -> value % 17 == 0 ? 6 : 1),
                new Monkey(Arrays.asList(53, 76, 59, 92, 95), old -> old + 2, value -> value % 19 == 0 ? 4 : 3),
                new Monkey(Arrays.asList(81, 61, 75, 89, 70, 92), old -> old * old, value -> value % 3 == 0 ? 0 : 1),
                new Monkey(Arrays.asList(81, 86, 62, 87), old -> old + 8, value -> value % 13 == 0 ? 3 : 5)
        };
        System.out.println("Monkey business for 20 rounds, common divisor 3\n" + calculateMonkeyBusiness(20, 3, monkeys));
        System.out.println("Monkey business for 10000 rounds, common divisor 1\n" + calculateMonkeyBusiness(10_000, 1, monkeys));
    }

    public static String calculateMonkeyBusiness(final int rounds, final int commonDivisor, final Monkey... monkeys) {
        for (int i = 0; i < rounds; i++) {
            for (final Monkey monkey : monkeys) {
                monkey.forEachRemaining(item -> {
                    item = item / commonDivisor;
                    final int nextMonkey = monkey.nextMonkey(item);
                    monkeys[nextMonkey].addItem(item);
                });
            }
        }

        final StringBuilder sb = new StringBuilder();
        for (int i = 0; i < monkeys.length; i++) {
            sb.append('[');
            sb.append(i);
            sb.append(']');
            sb.append('=');
            sb.append(monkeys[i].inspectedItems);
            sb.append('\n');
        }
        Arrays.sort(monkeys, Comparator.comparingInt(Monkey::getInspectedItems).reversed());
        sb.append("Result=");
        sb.append(monkeys[0].getInspectedItems() * monkeys[1].getInspectedItems());
        sb.append('\n');
        return sb.toString();
    }

    static class Monkey implements Iterator<Integer> {
        private int inspectedItems;
        private final Queue<Integer> items;
        private final IntUnaryOperator operation;
        private final IntUnaryOperator nextMonkeyOperator;

        public Monkey(Collection<Integer> items, IntUnaryOperator operation, IntUnaryOperator nextMonkeyOperator) {
            this.items = new LinkedList<>(items);
            this.operation = operation;
            this.nextMonkeyOperator = nextMonkeyOperator;
        }

        public int nextMonkey(final int item) {
            return nextMonkeyOperator.applyAsInt(item);
        }

        public void addItem(final int item) {
            items.add(item);
        }

        public int getInspectedItems() {
            return inspectedItems;
        }

        @Override
        public boolean hasNext() {
            return !items.isEmpty();
        }

        @Override
        public Integer next() {
            inspectedItems++;
            final Integer item = items.poll();
            if (item == null) {
                throw new IllegalStateException("Invalid item");
            }
            return operation.applyAsInt(item);
        }

    }
}
