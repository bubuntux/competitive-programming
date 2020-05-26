package org.bubuntux.codebusters.point;

public class Point {

    private int _x;
    private int _y;

    protected Point() {
    }

    public Point(Point point) {
        _x = point._x;
        _y = point._y;
    }

    public Point(int x, int y) {
        _x = x;
        _y = y;
    }

    public int getX() {
        return _x;
    }

    public void setX(int x) {
        _x = x;
    }

    public int getY() {
        return _y;
    }

    public void setY(int y) {
        _y = y;
    }

    public int distanceTo(Point point) {
        return (int) Math.round(Math.sqrt(Math.pow(point._x - _x, 2) + Math.pow(point._y - _y, 2)));
    }

}
