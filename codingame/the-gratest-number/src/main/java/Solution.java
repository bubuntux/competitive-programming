import java.util.Collections;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

class Solution {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);
        System.err.println("Input ***********");

        final int N = in.nextInt();
        System.err.println(N);
        in.nextLine();
        final String line = in.nextLine();
        System.err.println(line);
        final List<Character> input = line.chars().mapToObj(value -> (char) value).filter(character -> character != Character.valueOf(' ')).collect(Collectors.toList());


        boolean negative = input.remove(Character.valueOf('-'));
        boolean dot = input.remove(Character.valueOf('.'));
        Collections.sort(input);
        StringBuilder sb = new StringBuilder();
        if (negative) {
            sb.append('-');
        } else {
            Collections.reverse(input);
        }
        for (int i = 0; i < input.size(); i++) {
            if (dot) {
                if (negative) {
                    if (i == 1) {
                        sb.append('.');
                    }
                } else if (i == input.size() - 1) {
                    if (input.get(i) == Character.valueOf('0')) {
                        continue;
                    }
                    sb.append('.');
                }
            }
            sb.append(input.get(i));
        }

        System.err.println("Output ***********");

        if (Double.valueOf(sb.toString()) == 0.0) {
            System.out.println(0);
        } else {
            System.out.println(sb.toString());
        }
    }
}