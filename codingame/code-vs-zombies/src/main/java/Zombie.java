public class Zombie extends Entity {

    private final Point _nextPosition;

    public Zombie() {
        super();
        _nextPosition = new Point();
    }

    public void updateNextPosition(short x, short y) {
        _nextPosition.update(x, y);
    }
}
