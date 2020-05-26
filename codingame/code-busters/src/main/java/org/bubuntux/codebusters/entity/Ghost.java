package org.bubuntux.codebusters.entity;

import org.bubuntux.codebusters.action.BustAction;

public class Ghost extends Entity {

    private final int _maxStamina;
    private int _bustersAttacking;
    private int _stamina;
    private boolean _captured;
    private boolean _lost;

    private final BustAction _bust;

    public Ghost(int id, int maxStamina) {
        super(id);
        _maxStamina = maxStamina;
        _bust = new BustAction(this);
    }

    public int getBustersAttacking() {
        return _bustersAttacking;
    }

    public void setBustersAttacking(int bustersAttacking) {
        _bustersAttacking = bustersAttacking;
    }

    public int getMaxStamina() {
        return _maxStamina;
    }

    public int getStamina() {
        return _stamina;
    }

    public void setStamina(int stamina) {
        _stamina = stamina;
    }

    public boolean isCaptured() {
        return _captured;
    }

    public boolean isNotCaptured() {
        return !_captured;
    }

    public void setCaptured(boolean captured) {
        _captured = captured;
    }

    public boolean isNotLost() {
        return !_lost;
    }

    public boolean isLost() {
        return _lost;
    }

    public void setLost(boolean lost) {
        _lost = lost;
    }

    public BustAction getBust() {
        return _bust;
    }
}
