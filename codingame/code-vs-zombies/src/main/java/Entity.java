public class Entity {

    private final Point _position;
    private boolean _alive;

    public Entity() {
        _position = new Point();
    }

    public void updatePosition(short x, short y) {
        _position.update(x, y);
    }

    public void dead() {
        _alive = false;
    }

    public void alive() {
        _alive = true;
    }

    public boolean isDead() {
        return !_alive;
    }

    public boolean isAlive() {
        return _alive;
    }

}
