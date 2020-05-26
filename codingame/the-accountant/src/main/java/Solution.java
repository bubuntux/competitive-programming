import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

class Player {

    public static void main(String args[]) {
        Player player = new Player();
        while (true) {
            player.update();
            System.out.println(player.action());
        }
    }

    private final Scanner in;
    private final Wolf wolf;
    private final Map<Integer, Data> dataMap;
    private final Map<Integer, Enemy> enemies;

    private Player() {
        in = new Scanner(System.in);
        wolf = new Wolf();
        dataMap = new HashMap<>();
        enemies = new HashMap<>();
    }

    private void update() {
        System.err.println("********** Updating Entities **********");
        Stream.concat(dataMap.values().stream(), enemies.values().stream()).forEach(entity -> entity.inPlay = false);

        wolf.position.x = in.nextInt();
        wolf.position.y = in.nextInt();
        System.err.println("********** Wolf **********");
        System.err.println(wolf);

        IntStream.range(0, in.nextInt()).forEach(ignored -> {
            final Data data = dataMap.computeIfAbsent(in.nextInt(), Data::new);
            data.position.x = in.nextInt();
            data.position.y = in.nextInt();
            data.inPlay = true;
        });
        System.err.println("********** Data **********");
        dataMap.values().forEach(System.err::println);

        IntStream.range(0, in.nextInt()).forEach(ignored -> {
            final Enemy enemy = enemies.computeIfAbsent(in.nextInt(), Enemy::new);
            enemy.position.x = in.nextInt();
            enemy.position.y = in.nextInt();
            enemy.life = in.nextInt();
            if (wolf.isMoving()) {
                enemy.distanceToWolf = -1;
                enemy.distanceToTarget = -1;
                enemy.path = Helper.EMPTY_PATH;
            } else {
                enemy.distanceToWolf = enemy.distanceTo(wolf);
                final Point target = data().min((left, right) -> {
                    final int distanceCompare = Integer.compare(enemy.distanceTo(left), enemy.distanceTo(right));
                    if (distanceCompare != 0) {
                        return distanceCompare;
                    }
                    return Integer.compare(left.id, right.id);
                }).map(data -> data.position).orElseThrow(RuntimeException::new);
                enemy.distanceToTarget = enemy.position.distanceTo(target);
                enemy.path = Helper.path(enemy.position, target, Enemy.MOVE_UNITS);
            }
            enemy.inPlay = true;
        });
        System.err.println("********** Enemies **********");
        enemies.values().forEach(System.err::println);

        System.err.println("********** Entities Updated **********\n\n");
    }

    private String action() {
        if (wolf.isMoving()) {
            final String action = "MOVE " + wolf.movingTo;
            if (wolf.distanceMovingTo() == 0) { //TODO
                wolf.movingTo = null;
            }
            return action;
        }

        //final Set<Enemy> enemiesToAvoid = enemies().filter(enemy -> enemy.canShoot).collect(Collectors.toSet());

        List<Enemy> prioritizedEnemies = enemies().sorted((left, right) -> {
            final int turnsToTargetCompare = Integer.compare(left.path.size(), right.path.size());
            if (turnsToTargetCompare != 0) {
                return turnsToTargetCompare;
            }
            final int lifeCompare = Integer.compare(left.life, right.life);
            if (lifeCompare != 0) {
                return lifeCompare;
            }
            return Integer.compare(left.distanceToWolf, right.distanceToWolf);
        }).collect(Collectors.toList());

        System.err.println("prioritizedEnemies: " + prioritizedEnemies.stream().map(enemy -> String.valueOf(enemy.id)).collect(Collectors.joining()));


       /* System.err.println("\n\n");
        if (Helper.shootDamage(enemyToKill.distanceToWolf) < enemyToKill.life) {
            return "MOVE " + enemyToKill.path.getFirst();
        }


        return "SHOOT " + enemyToKill.id;*/
       return "";
    }

    private Stream<Data> data() {
        return dataMap.values().stream().filter(data -> data.inPlay);
    }

    private Stream<Enemy> enemies() {
        return enemies.values().stream().filter(enemy -> enemy.inPlay);
    }

}

abstract class Zone {
    static final int WIDTH = 16000;
    static final int HEIGHT = 9000;
}

class Point {
    int x;
    int y;

    Point() {
    }

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    int distanceTo(Point point) {
        return (int) Math.round(Math.sqrt(Math.pow(point.x - x, 2) + Math.pow(point.y - y, 2)));
    }

    @Override
    public String toString() {
        return x + " " + y;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Point point = (Point) o;
        return x == point.x && y == point.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}

class Entity {

    final Point position;

    Entity() {
        position = new Point();
    }

    int distanceTo(Entity entity) {
        return position.distanceTo(entity.position);
    }

    @Override
    public String toString() {
        return "Entity{" +
                "position=" + position +
                '}';
    }
}

class Wolf extends Entity {

    static final int MOVE_UNITS = 1000;

    Point movingTo;

    boolean isMoving() {
        return movingTo != null;
    }

    int distanceMovingTo(){
        return movingTo.distanceTo(position);
    }

}

class NPC extends Entity {

    final int id;
    boolean inPlay;

    NPC(int id) {
        this.id = id;
    }

    @Override
    public String toString() {
        return "NPC{" +
                "id=" + id +
                "inPlay=" + inPlay +
                "} " + super.toString();
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        NPC npc = (NPC) o;
        return id == npc.id;
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }
}

class Data extends NPC {
    Data(int id) {
        super(id);
    }
}

class Enemy extends NPC {

    static final int MOVE_UNITS = 500;
    static final int SHOOT_RANGE = 2000;

    int life;
    int distanceToWolf;
    int distanceToTarget;
    LinkedList<Point> path;

    Enemy(int id) {
        super(id);
    }

    @Override
    public String toString() {
        return "Enemy{" +
                "life=" + life +
                ", distanceToWolf=" + distanceToWolf +
                ", distanceToTarget=" + distanceToTarget +
                ", path=" + path.stream().map(String::valueOf).collect(Collectors.joining(",", "{", "}")) +
                "} " + super.toString();
    }
}

abstract class Helper {

    static final LinkedList<Point> EMPTY_PATH = new LinkedList<>();

    static LinkedList<Point> path(Point origin, Point target, int interval) {
        final List<Point> path = new LinkedList<>();

        final int totalDistance = origin.distanceTo(target);

        final double theta = Math.atan2(target.y - origin.y, target.x - origin.x);
        final double cosTheta = Math.cos(theta);
        final double sinTheta = Math.sin(theta);

        int distance = interval;
        for (; distance < totalDistance; distance += interval) {
            final int x = (int) Math.round((distance * cosTheta) + origin.x);
            final int y = (int) Math.round((distance * sinTheta) + origin.y);
            path.add(new Point(x, y));
        }

        if (distance >= totalDistance) {
            path.add(target);
        }

        return path.stream().distinct().collect(Collectors.toCollection(LinkedList::new));
    }

    static Map<Point, Integer> shootSimulation(Point shooter, Enemy enemy) {
        Map<Point, Integer> simulation = new HashMap<>();

        simulation.put(shooter, shootDamage(shooter.distanceTo(enemy.position)));
        for (int i = 0; i < enemy.path.size(); i++) {

        }
        return simulation;
    }

    static int shootDamage(int distance) {
        return (int) Math.round(125000 / Math.pow(distance, 1.2));
    }

    private int distanceToDamage(int damage) {
        return (int) Math.round(Math.pow(125000 / damage, 1 / 1.2));
    }
}