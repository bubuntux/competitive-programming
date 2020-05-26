package org.bubuntux.codebusters.point;

public class CheckPoint extends Point {
    private int _counter;

    public CheckPoint(int x, int y) {
        super(x, y);
        _counter = 0;
    }

    public int getCounter() {
        return _counter;
    }

    public void increaseCounter() {
        _counter++;
    }

}
