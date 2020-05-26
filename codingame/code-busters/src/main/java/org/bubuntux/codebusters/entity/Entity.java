package org.bubuntux.codebusters.entity;

import org.bubuntux.codebusters.point.Point;

public abstract class Entity extends Point implements Comparable<Entity> {

    private final int _id;
    private boolean _inRange;

    Entity(int id) {
        _id = id;
    }

    public int getId() {
        return _id;
    }

    public boolean isInRange() {
        return _inRange;
    }

    public void setInRange(boolean inRange) {
        _inRange = inRange;
    }

    @Override
    public int compareTo(Entity other) {
        return Integer.compare(_id, other._id);
    }

}
