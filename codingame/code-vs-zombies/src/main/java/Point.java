public class Point { //TODO immutable ?

    private short _x;
    private short _y;

    public short getX() {
        return _x;
    }

    public short getY() {
        return _y;
    }

    public void update(short x, short y) {
        _x = x;
        _y = y;
    }

    public int distanceTo(Point point) {
        return (int) Math.ceil(Math.sqrt(Math.pow(point._x - _x, 2) + Math.pow(point._y - _y, 2)));
    }

    @Override
    public String toString() {
        return _x + " " + _y;
    }
}
