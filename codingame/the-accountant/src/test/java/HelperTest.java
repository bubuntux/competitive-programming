import org.junit.Test;

import java.util.List;

import static org.junit.Assert.assertEquals;

public class HelperTest {

    @Test
    public void pathOnX() throws Exception {
        List<Point> path = Helper.path(new Point(0, 0), new Point(50, 0), 10);
        assertEquals(5, path.size());
        assertEquals(new Point(10, 0), path.get(0));
        assertEquals(new Point(20, 0), path.get(1));
        assertEquals(new Point(30, 0), path.get(2));
        assertEquals(new Point(40, 0), path.get(3));
        assertEquals(new Point(50, 0), path.get(4));
    }

    @Test
    public void pathOnX2() throws Exception {
        List<Point> path = Helper.path(new Point(0, 0), new Point(55, 0), 10);
        assertEquals(6, path.size());
        assertEquals(new Point(10, 0), path.get(0));
        assertEquals(new Point(20, 0), path.get(1));
        assertEquals(new Point(30, 0), path.get(2));
        assertEquals(new Point(40, 0), path.get(3));
        assertEquals(new Point(50, 0), path.get(4));
        assertEquals(new Point(55, 0), path.get(5));
    }

    @Test
    public void pathOnY() throws Exception {
        List<Point> path = Helper.path(new Point(0, 0), new Point(0, 60), 10);
        assertEquals(6, path.size());
        assertEquals(new Point(0, 10), path.get(0));
        assertEquals(new Point(0, 20), path.get(1));
        assertEquals(new Point(0, 30), path.get(2));
        assertEquals(new Point(0, 40), path.get(3));
        assertEquals(new Point(0, 50), path.get(4));
        assertEquals(new Point(0, 60), path.get(5));
    }

    @Test
    public void pathOnY2() throws Exception {
        List<Point> path = Helper.path(new Point(0, 0), new Point(0, 45), 10);
        assertEquals(5, path.size());
        assertEquals(new Point(0, 10), path.get(0));
        assertEquals(new Point(0, 20), path.get(1));
        assertEquals(new Point(0, 30), path.get(2));
        assertEquals(new Point(0, 40), path.get(3));
        assertEquals(new Point(0, 45), path.get(4));
    }

    @Test
    public void path() throws Exception {
        List<Point> path = Helper.path(new Point(0, 0), new Point(30, 30), 10);
        assertEquals(5, path.size());

        assertEquals(new Point(7, 7), path.get(0));
        assertEquals(new Point(14, 14), path.get(1));
        assertEquals(new Point(21, 21), path.get(2));
        assertEquals(new Point(28, 28), path.get(3));
        assertEquals(new Point(30, 30), path.get(4));
    }

    @Test
    public void pathSmall() throws Exception {
        List<Point> path = Helper.path(new Point(0, 0), new Point(3, 3), 1);
        assertEquals(3, path.size());
        assertEquals(new Point(1, 1), path.get(0));
        assertEquals(new Point(2, 2), path.get(1));
        assertEquals(new Point(3, 3), path.get(2));
    }
}