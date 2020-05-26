package org.bubuntux.codebusters.entity;

import org.bubuntux.codebusters.action.Action;
import org.bubuntux.codebusters.action.StunAction;

public class Buster extends Entity {

    private static final int MAX_STUN_CHARGE = 20;

    private final boolean _friendly;
    private int _state;
    private int _value;

    private int _stunCharge;
    private Action _action;

    public Buster(int id, boolean friendly) {
        super(id);
        _friendly = friendly;
        _stunCharge = MAX_STUN_CHARGE;
        if (isEnemy()) {
            _action = new StunAction(this);
        }
    }

    public void setState(int state) {
        _state = state;
    }

    public void setValue(int value) {
        _value = value;
    }

    public boolean isAvailable() {
        return _state == 0;
    }

    public boolean isCarryingGhost() {
        return _state == 1;
    }

    public boolean isStunned() {
        return _state == 2;
    }

    public boolean isCapturing() {
        return _state == 3;
    }

    public boolean isEnemy() {
        return !_friendly;
    }

    public boolean isFriendly() {
        return _friendly;
    }

    public int carryingGhostId() {
        return isCarryingGhost() ? _value : -1;
    }

    public int turnsImmobile() {
        return isStunned() ? _value : 0;
    }

    public boolean canStun() {
        return _stunCharge >= MAX_STUN_CHARGE;
    }

    public void increaseCharge() {
        _stunCharge++;
    }

    public void resetCharge() {
        _stunCharge = 0;
    }

    public void takeAction(){
        //TODO
    }

    public Action getAction() {
        return _action;
    }

    public void setAction(Action action) {
        _action = action;
    }
}
