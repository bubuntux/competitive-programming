import java.util.Iterator;
import java.util.List;
import org.apache.commons.io.IOUtils;
import org.apache.commons.lang3.StringUtils;
import org.junit.Test;

import static org.junit.Assert.assertEquals;


public class BrailleTest {

    @Test
    public void testConvert() {

        char[][][] inputs = {//
            new char[][]{    // H
                {'O', '.'},  //
                {'O', 'O'},  //
                {'.', '.'},  //
            },               //
            new char[][]{    // E
                {'O', '.'},  //
                {'.', 'O'},  //
                {'.', '.'},  //
            },               //
            new char[][]{    // L
                {'O', '.'},  //
                {'O', '.'},  //
                {'O', '.'},  //
            },               //
            new char[][]{    // L
                {'O', '.'},  //
                {'O', '.'},  //
                {'O', '.'},  //
            },               //
            new char[][]{    // O
                {'O', '.'},  //
                {'.', 'O'},  //
                {'O', '.'},  //
            },               //
            new char[][]{    // W
                {'.', 'O'},  //
                {'O', 'O'},  //
                {'.', 'O'},  //
            },               //
            new char[][]{    // O
                {'O', '.'},  //
                {'.', 'O'},  //
                {'O', '.'},  //
            },               //
            new char[][]{    // R
                {'O', '.'},  //
                {'O', 'O'},  //
                {'O', '.'},  //
            },               //
            new char[][]{    // L
                {'O', '.'},  //
                {'O', '.'},  //
                {'O', '.'},  //
            },               //
            new char[][]{    // D
                {'O', 'O'},  //
                {'.', 'O'},  //
                {'.', '.'},  //
            }                //
        };

        assertEquals(Braille.convert(inputs), "helloworld");
    }

    @Test
    public void testCases() throws Exception {
        List<String> lines = IOUtils.readLines(getClass().getResourceAsStream("TestCases.txt"));

        Iterator<String> iterator = lines.iterator();
        while (iterator.hasNext()) {
            String word = iterator.next();
            if (StringUtils.isNotBlank(word)) {
                char[][][] inputs = convertLines(next(iterator), next(iterator), next(iterator));
                assertEquals(Braille.convert(inputs), word);
            }
        }
    }

    private String next(Iterator<String> it) throws Exception {
        if (!it.hasNext()) {
            throw new Exception("Wrong file");
        }
        return it.next();
    }

    private char[][][] convertLines(String line0, String line1, String line2) {
        String[] split0 = StringUtils.split(line0, ' ');
        String[] split1 = StringUtils.split(line1, ' ');
        String[] split2 = StringUtils.split(line2, ' ');

        char[][][] inputs = new char[split0.length][][];

        for (int index = 0; index < inputs.length; index++) {
            char[][] input = inputs[index] = new char[3][];
            input[0] = new char[]{split0[index].charAt(0), split0[index].charAt(1)};
            input[1] = new char[]{split1[index].charAt(0), split1[index].charAt(1)};
            input[2] = new char[]{split2[index].charAt(0), split2[index].charAt(1)};
        }
        return inputs;
    }
}
