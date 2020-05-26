package org.bubuntux.codebusters.action;

import org.bubuntux.codebusters.entity.Buster;

public class StunAction implements Action {

    private final Buster _target;

    public StunAction(Buster target) {
        _target = target;
    }

    @Override
    public String toString() {
        return "STUN " + _target.getId();
    }
}
