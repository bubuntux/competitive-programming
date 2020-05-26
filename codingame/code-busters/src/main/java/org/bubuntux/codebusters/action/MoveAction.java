package org.bubuntux.codebusters.action;

import org.bubuntux.codebusters.point.Point;

public class MoveAction implements Action {

    private Point _target;

    public MoveAction(Point target) {
        _target = target;
    }

    public Point getTarget() {
        return _target;
    }

    @Override
    public String toString() {
        return "MOVE " + _target.getX() + " " + _target.getY();
    }
}
