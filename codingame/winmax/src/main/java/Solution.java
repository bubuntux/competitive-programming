import java.util.Arrays;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;
import java.util.stream.Stream;

class Solution {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);

        System.err.println("P1 cards");
        Queue<Card> p1 = getCards(in);
        System.err.println("\nP2 cards");
        Queue<Card> p2 = getCards(in);

        int winner = -1;
        int rounds = 0;

        Queue<Card> playedCardsP1 = new LinkedList<>();
        Queue<Card> playedCardsP2 = new LinkedList<>();

        while (winner == -1) {
            Card p1Card = p1.poll();
            Card p2Card = p2.poll();

            if (p1Card == null) {
                winner = 2;
                continue;
            }
            if (p2Card == null) {
                winner = 1;
                continue;
            }

            playedCardsP1.add(p1Card);
            playedCardsP2.add(p2Card);

            int compareTo = p1Card.compareTo(p2Card);
            if (compareTo == 0) {

                getWarCards(playedCardsP1, p1);
                getWarCards(playedCardsP2, p2);

                if (Stream.concat(playedCardsP1.stream(), playedCardsP2.stream()).anyMatch(card -> card == null)) {
                    winner = 0;
                }

            } else {
                rounds++;
                if (compareTo > 0) {
                    p1.addAll(playedCardsP1);
                    p1.addAll(playedCardsP2);
                } else {
                    p2.addAll(playedCardsP1);
                    p2.addAll(playedCardsP2);
                }
                playedCardsP1.clear();
                playedCardsP2.clear();
            }
        }

        if (winner == 0) {
            System.out.println("PAT");
        } else {
            System.out.println(winner + " " + rounds);
        }
    }

    private static void getWarCards(Queue<Card> playedCards, Queue<Card> player) {
        for (int i = 0; i < 3; i++) {
            Card card = player.poll();
            playedCards.add(card);
        }
    }

    private static Queue<Card> getCards(Scanner in) {
        final Queue<Card> cards = new LinkedList<>();
        final int n = in.nextInt(); // the number of cards
        for (int i = 0; i < n; i++) {
            final String cardString = in.next();
            Card card = Card.get(cardString);
            System.err.println(card);
            cards.add(card);
        }
        return cards;
    }

    private enum Card {
        TWO("2"), THREE("3"), FOUR("4"), FIVE("5"), SIXTH("6"), SEVEN("7"), EIGHT("8"), NINE("9"), TEN("10"), JOKER("J"), QUEEN("Q"), KING("K"), ACE("A");

        private String _prefix;

        Card(String _prefix) {
            this._prefix = _prefix;
        }

        static Card get(String card) {
            return Arrays.stream(Card.values()).filter(c -> card.startsWith(c._prefix)).findFirst().orElse(null);
        }
    }
}