import java.util.Collections;
import java.util.LinkedList;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

class Solution {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);
        final int R = in.nextInt();
        final int L = in.nextInt();

        List<Integer> currentLevel = Collections.singletonList(R);
        for (int i = 1; i < L; i++) {
            System.err.println(toString(currentLevel));
            currentLevel = nextLevel(currentLevel);
        }

        System.out.println(toString(currentLevel));
    }

    private static String toString(List<Integer> currentLevel) {
        return currentLevel.stream().map(Object::toString).collect(Collectors.joining(" "));
    }

    private static List<Integer> nextLevel(List<Integer> prevLevel) {
        List<Integer> level = new LinkedList<>();

        int counter = 1;
        Integer prevValue = null;
        for (Integer x : prevLevel) {
            if (x.equals(prevValue)) {
                counter++;
            } else {
                if (prevValue != null) {
                    level.add(counter);
                    level.add(prevValue);
                }
                prevValue = x;
                counter = 1;
            }
        }
        level.add(counter);
        level.add(prevValue);
        return level;
    }

}