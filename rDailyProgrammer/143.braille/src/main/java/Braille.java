import java.security.InvalidParameterException;

public abstract class Braille {

    public static String convert(char[][][] inputs) {
        StringBuilder stringBuilder = new StringBuilder();
        for (char[][] input : inputs) {
            stringBuilder.append(convert(input));
        }
        return stringBuilder.toString();
    }

    public static char convert(char[][] input) {
        if (input.length != 3 || input[0].length != 2 || input[1].length != 2 || input[2].length != 2) {
            throw new InvalidParameterException("invalid input");
        }

        StringBuilder sb = new StringBuilder();
        for (char[] rows : input) {
            for (char c : rows) {
                if (c != 'O' && c != '.') {
                    throw new InvalidParameterException("invalid input");
                }
                sb.append(c);
            }
        }

        return convert(sb.toString());
    }

    private static char convert(String input) {
        switch (input) {
            case "O.....":
                return 'a';
            case "O.O...":
                return 'b';
            case "OO....":
                return 'c';
            case "OO.O..":
                return 'd';
            case "O..O..":
                return 'e';
            case "OOO...":
                return 'f';
            case "OOOO..":
                return 'g';
            case "O.OO..":
                return 'h';
            case ".OO...":
                return 'i';
            case ".OOO..":
                return 'j';
            case "O...O.":
                return 'k';
            case "O.O.O.":
                return 'l';
            case "OO..O.":
                return 'm';
            case "OO.OO.":
                return 'n';
            case "O..OO.":
                return 'o';
            case "OOO.O.":
                return 'p';
            case "OOOOO.":
                return 'q';
            case "O.OOO.":
                return 'r';
            case ".OO.O.":
                return 's';
            case ".OOOO.":
                return 't';
            case "O...OO":
                return 'u';
            case "O.O.OO":
                return 'v';
            case ".OOO.O":
                return 'w';
            case "OO..OO":
                return 'x';
            case "OO.OOO":
                return 'y';
            case "O..OOO":
                return 'z';
            default:
                return ' ';
        }
    }
}