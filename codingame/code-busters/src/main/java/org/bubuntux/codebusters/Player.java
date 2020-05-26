package org.bubuntux.codebusters;

import org.bubuntux.codebusters.action.Action;
import org.bubuntux.codebusters.action.MoveAction;
import org.bubuntux.codebusters.action.RadarAction;
import org.bubuntux.codebusters.action.ReleaseAction;
import org.bubuntux.codebusters.entity.Buster;
import org.bubuntux.codebusters.entity.Entity;
import org.bubuntux.codebusters.entity.Ghost;
import org.bubuntux.codebusters.point.CheckPoint;
import org.bubuntux.codebusters.point.Point;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

class Player {

    private static final int WIDTH = 16000;
    private static final int HEIGHT = 9000;

    private static final int VISIBILITY = 2200;
    private static final int HALF_VISIBILITY = VISIBILITY / 2;
    private static final int CAPTURE_RANGE = 1760;
    private static final int TO_CLOSE = 900;
    private static final int RELEASE_RANGE = 1600;
    private static final int BUST_RANGE = 1760;
    private static final int MOVES_PER_TURN = 800;

    private static final Action RELEASE = new ReleaseAction();
    private static final Action RADAR = new RadarAction();

    private final Scanner _in;
    private final int _bustersPerPlayer;
    private final int _bustersCount;
    private final int _ghostCount;
    private final int _myTeamId;

    private final Point _home;
    private final Point _enemyHome;
    final List<CheckPoint> _checkPoints;

    private final Ghost[] _ghosts;
    private final Buster[] _busters;

    public static void main(String args[]) throws Exception {
        Player player = new Player();
        while (true) {
            player.update();
        }
    }

    private Player() {
        _in = new Scanner(System.in);

        _bustersPerPlayer = _in.nextInt();
        _ghostCount = _in.nextInt();
        _myTeamId = _in.nextInt();

        _bustersCount = _bustersPerPlayer * 2;
        _busters = new Buster[_bustersCount];

        _ghosts = new Ghost[_ghostCount];

        _home = _myTeamId == 0 ? new Point(0, 0) : new Point(WIDTH, HEIGHT);
        _enemyHome = _myTeamId != 0 ? new Point(0, 0) : new Point(WIDTH, HEIGHT);

        _checkPoints = getCheckPoints(_enemyHome);
    }

    private List<CheckPoint> getCheckPoints(Point enemyHome) {
        final List<CheckPoint> checkPoints = new ArrayList<>();
        final int maxCheckpointY = Math.abs(enemyHome.getY() - HALF_VISIBILITY);
        for (int i = HALF_VISIBILITY; i < WIDTH; i += VISIBILITY) {
            checkPoints.add(new CheckPoint(i, maxCheckpointY));
        }
        final int maxCheckpointX = Math.abs(enemyHome.getX() - HALF_VISIBILITY);
        for (int i = HALF_VISIBILITY; i < HEIGHT; i += VISIBILITY) {
            checkPoints.add(new CheckPoint(maxCheckpointX, i));
        }
        return checkPoints;
    }

    void update() {
        System.err.println("Updating entities ...");
        Stream.concat(ghosts(), Arrays.stream(_busters)).filter(e -> e != null).forEach(entity -> entity.setInRange(false));
        final int entityCount = _in.nextInt();
        for (int i = 0; i < entityCount; i++) {
            final int id = _in.nextInt();
            final int x = _in.nextInt();
            final int y = _in.nextInt();
            final int entityType = _in.nextInt();
            final int state = _in.nextInt();
            final int value = _in.nextInt();

            Entity entity;
            if (entityType == -1) { // ghost
                Ghost ghost = _ghosts[id];
                if (ghost == null) {
                    ghost = new Ghost(id, state);
                    _ghosts[id] = ghost;
                }
                ghost.setStamina(state);
                ghost.setBustersAttacking(value);
                ghost.setLost(false);
                entity = ghost;
            } else { // buster
                Buster buster = _busters[id];
                if (buster == null) {
                    boolean friend = entityType == _myTeamId;
                    buster = new Buster(id, friend);
                    _busters[id] = buster;
                }
                buster.setState(state);
                buster.setValue(value);
                entity = buster;
            }
            entity.setX(x);
            entity.setY(y);
            entity.setInRange(true);
            System.err.println(entity);
        }

        System.err.println("taking actions..");
        myBuster().sorted().peek(System.err::println).map(this::action).forEach(System.out::println);
    }

    private Stream<Ghost> ghosts() {
        return Arrays.stream(_ghosts).filter(g -> g != null);
    }

    private Stream<Buster> enemies() {
        return Arrays.stream(_busters).filter(b -> b != null).filter(Buster::isEnemy);
    }

    private Stream<Buster> myBuster() {
        return Arrays.stream(_busters).filter(b -> b != null).filter(Buster::isFriendly);
    }

    private String action(Buster buster) {
        buster.increaseCharge();

        if (buster.isCarryingGhost()) {
            if (buster.distanceTo(_home) <= RELEASE_RANGE) {
                _ghosts[buster.carryingGhostId()].setCaptured(true);
                buster.setAction(RELEASE);
                return "RELEASE";
            }
            List<Buster> enemies = enemies()
                    .filter(Buster::isInRange)
                    .filter(b -> !b.isStunned())
                    .filter(b -> buster.distanceTo(b) <= BUST_RANGE).collect(Collectors.toList());
            if (buster.canStun() && enemies.size() == 1) { // if more, run and ask for help?
                buster.resetCharge();
                Buster enemy = enemies.get(0);
                enemy.setState(2); //stunned
                return "STUN " + enemy.getId();
            }
            return "MOVE " + _home.getX() + " " + _home.getY();
        }

        Optional<Buster> enemyOptional = enemies() //need to re-filter, TODO chase enemies carrying ghost?
                .filter(Buster::isInRange)
                .filter(Buster::isCarryingGhost)
                .filter(Buster::isCapturing)
                .filter(b -> buster.distanceTo(b) <= BUST_RANGE)
                .sorted((b1, b2) -> {
                    int carryingCompare = Boolean.compare(b1.isCarryingGhost(), b2.isCarryingGhost());
                    if (carryingCompare != 0) {
                        return carryingCompare;
                    }
                    return Boolean.compare(b1.isCapturing(), b2.isCapturing());
                }).findFirst();
        if (buster.canStun() && enemyOptional.isPresent()) {
            buster.resetCharge();
            Buster enemy = enemyOptional.get();
            enemy.setState(2); //stunned
            return "STUN " + enemy.getId();
        }

        Optional<Ghost> ghostOptional;
        do {
            ghostOptional = ghosts()
                    //.filter(Ghost::isInRange)
                    .filter(Ghost::isNotCaptured)
                    .filter(Ghost::isNotLost)
                    .sorted((g1, g2) -> {
                        int d1 = buster.distanceTo(g1);
                        int d2 = buster.distanceTo(g2);
                        if (Math.abs(d1 - d2) <= MOVES_PER_TURN) {
                            return Integer.compare(g1.getStamina(), g2.getStamina());
                        }
                        return Integer.compare(d1, d2);
                    }).findFirst();

            if (ghostOptional.isPresent()) {
                Ghost ghost = ghostOptional.get();
                int distance = buster.distanceTo(ghost);
                if (distance > CAPTURE_RANGE) { // move closer
                    return "MOVE " + ghost.getX() + " " + ghost.getY();
                }
                if (distance < TO_CLOSE) { // move away
                    if (ghost.isInRange()) {
                        int xDiff = Math.abs(buster.getX() - ghost.getX());
                        int x = buster.getX() > ghost.getX() ? buster.getX() + xDiff : buster.getX() - xDiff;
                        if (x < 0) {
                            x = 0;
                        } else if (x > WIDTH) {
                            x = WIDTH;
                        }
                        int yDiff = Math.abs(buster.getY() - ghost.getY());
                        int y = buster.getY() > ghost.getY() ? buster.getY() + yDiff : buster.getY() - yDiff;
                        if (y < 0) {
                            y = 0;
                        } else if (y > HEIGHT) {
                            y = HEIGHT;
                        }
                        return "MOVE " + x + " " + y;
                    } else {
                        ghost.setLost(true);
                    }
                }
                if (ghost.isInRange()) {
                    return "BUST " + ghost.getId();
                } else {
                    ghost.setLost(true);
                }
            }
        } while (ghostOptional.isPresent());

        Action action = buster.getAction();
        if (action == null || (action instanceof MoveAction && buster.distanceTo(((MoveAction) action).getTarget()) <= TO_CLOSE)) {
            CheckPoint checkPoint = _checkPoints.stream().sorted((c1, c2) -> {
                int counterCompare = Integer.compare(c1.getCounter(), c2.getCounter());
                if (counterCompare != 0) {
                    return counterCompare;
                }
                return Integer.compare(buster.distanceTo(c2), buster.distanceTo(c1));
            }).findFirst().get();
            checkPoint.increaseCounter();
            buster.setAction(new MoveAction(checkPoint));
        }

        return "MOVE ";// + movingTo.getX() + " " + movingTo.getY();
    }
}
