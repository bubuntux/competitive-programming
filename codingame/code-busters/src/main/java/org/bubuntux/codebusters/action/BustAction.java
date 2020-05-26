package org.bubuntux.codebusters.action;

import org.bubuntux.codebusters.entity.Ghost;

public class BustAction implements Action {

    private final Ghost _target;

    public BustAction(Ghost target) {
        _target = target;
    }

    @Override
    public String toString() {
        return "BUST " + _target.getId();
    }
}
