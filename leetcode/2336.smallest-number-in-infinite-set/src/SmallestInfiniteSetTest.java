import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class SmallestInfiniteSetTest {

    @Test
    void example1() {
        final var obj = new SmallestInfiniteSet();
        obj.addBack(2);
        Assertions.assertEquals(1, obj.popSmallest());
        Assertions.assertEquals(2, obj.popSmallest());
        Assertions.assertEquals(3, obj.popSmallest());
        obj.addBack(1);
        Assertions.assertEquals(1, obj.popSmallest());
        Assertions.assertEquals(4, obj.popSmallest());
        Assertions.assertEquals(5, obj.popSmallest());
    }
}