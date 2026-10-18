import java.util.TreeSet;

public class SmallestInfiniteSet {

    private int counter;
    private final TreeSet<Integer> treeSet;

    public SmallestInfiniteSet() {
        counter = 1;
        treeSet = new TreeSet<>();
    }

    public int popSmallest() {
        if (treeSet.isEmpty() || treeSet.first() > counter) {
            return counter++;
        }
        if (treeSet.first() == counter) {
            counter++;
        }
        return treeSet.pollFirst();
    }

    public void addBack(int num) {
        treeSet.add(num);
    }

}
