import java.util.*;
import java.util.function.LongUnaryOperator;
import java.util.stream.Collectors;

public class Solution {
    public static void main(String... args) {
        final Monkey[] monkeys = {
                new Monkey(Arrays.asList(75, 63), old -> old * 3, 11, 7, 2),
                new Monkey(Arrays.asList(65, 79, 98, 77, 56, 54, 83, 94), old -> old + 3, 2, 2, 0),
                new Monkey(List.of(66), old -> old + 5, 5, 7, 5),
                new Monkey(Arrays.asList(51, 89, 90), old -> old * 19, 7, 6, 4),
                new Monkey(Arrays.asList(75, 94, 66, 90, 77, 82, 61), old -> old + 1, 17, 6, 1),
                new Monkey(Arrays.asList(53, 76, 59, 92, 95), old -> old + 2, 19, 4, 3),
                new Monkey(Arrays.asList(81, 61, 75, 89, 70, 92), old -> old * old, 3, 0, 1),
                new Monkey(Arrays.asList(81, 86, 62, 87), old -> old + 8, 13, 3, 5)
        };
     //   System.out.println("Monkey business for 20 rounds, common divisor 3\n" + calculateMonkeyBusiness(20, monkeys));
        System.out.println("Monkey business for 10000 rounds, common divisor 1\n" + calculateMonkeyBusiness(10_000, monkeys));
    }

    public static String calculateMonkeyBusiness(final int rounds, final Monkey... monkeys) {
        int commonDivisor = Arrays.stream(monkeys).mapToInt(monkey -> monkey.testValue).reduce((i, i1) -> i * i1).getAsInt();
        for (int i = 0; i < rounds; i++) {
            for (final Monkey monkey : monkeys) {
                monkey.forEachRemaining(item -> {
                    item %= commonDivisor;
                    final int nextMonkey = monkey.nextMonkey(item);
                    monkeys[nextMonkey].addItem(item);
                });
            }
        }

        return getResult(monkeys);
    }

    private static String getResult(Monkey[] monkeys) {
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
        sb.append((long) monkeys[0].getInspectedItems() * (long) monkeys[1].getInspectedItems());
        sb.append('\n');
        return sb.toString();
    }

    static class Monkey implements Iterator<Long> {
        private int inspectedItems;
        private final Queue<Long> items;
        private final LongUnaryOperator operation;

        private final int testValue;
        private final int trueMI;
        private final int falseMI;

        public Monkey(Collection<Number> items, LongUnaryOperator operation, int testValue, int trueMI, int falseMI) {
            this.items = items.stream().map(Number::longValue).collect(Collectors.toCollection(LinkedList::new));
            this.operation = operation;
            this.testValue = testValue;
            this.trueMI = trueMI;
            this.falseMI = falseMI;
        }

        public int nextMonkey(final long item) {
            return (item % testValue == 0) ? trueMI : falseMI;
        }

        public void addItem(final long item) {
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
        public Long next() {
            final Long item = items.poll();
            if (item == null) {
                throw new IllegalStateException("Invalid item");
            }
            inspectedItems++;
            return operation.applyAsLong(item);
        }

    }
}
