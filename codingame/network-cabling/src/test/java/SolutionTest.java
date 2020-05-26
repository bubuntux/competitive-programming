import org.junit.Test;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.PrintStream;

import static org.junit.Assert.assertEquals;

public class SolutionTest {

    @Test
    public void testCases() throws Exception {
        testSolution("3\n0 0\n1 1\n2 2", "4"); // Case 1
        testSolution("3\n1 2\n0 0\n2 2", "4"); // Case 2
        testSolution("4\n1 2\n0 0\n2 2\n1 3", "5"); // Case 3
        testSolution("1\n1 1", "0"); //Case 4
        testSolution("3\n-5 -3\n-9 2\n3 -4", "18"); //Case 6
        testSolution("8\n" +
                "-28189131 593661218\n" +
                "102460950 1038903636\n" +
                "938059973 -816049599\n" +
                "-334087877 -290840615\n" +
                "842560881 -116496866\n" +
                "-416604701 690825290\n" +
                "19715507 470868309\n" +
                "846505116 -694479954", "6066790161");
    }

    private void testSolution(String input, String output) {
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        final ByteArrayOutputStream outContent = new ByteArrayOutputStream();
        System.setOut(new PrintStream(outContent));

        Solution.main(null);

        assertEquals(output, outContent.toString());
    }

}