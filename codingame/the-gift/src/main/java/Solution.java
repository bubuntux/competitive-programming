import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

class Solution {

    public static void main(String args[]) {
        Scanner in = new Scanner(System.in);
        final int numberOfParticipants = in.nextInt();
        System.err.println(numberOfParticipants);
        final List<Integer> budgets = new ArrayList<>(numberOfParticipants);
        final int totalPrice = in.nextInt();
        System.err.println(totalPrice);
        for (int i = 0; i < numberOfParticipants; i++) {
            final int budget = in.nextInt();
            //System.err.println(budget);
            budgets.add(budget);
        }

        Collections.sort(budgets);

        int price = totalPrice;
        int participants = numberOfParticipants;
        final List<Integer> solutions = new ArrayList<>(numberOfParticipants);

        int average = price / participants;
        for (int budget : budgets) {
            participants--;
            final int share = budget <= average ? budget : average;
            solutions.add(share);
            price -= share;
            if (participants > 0) {
                average = price / participants;
            }
        }

        System.err.println("Solution...");

        if (price > 0) {
            System.out.println("IMPOSSIBLE");
            return;
        }
        Collections.sort(solutions);
        solutions.forEach(System.out::println);
    }
}