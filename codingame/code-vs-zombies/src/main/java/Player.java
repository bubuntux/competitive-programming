import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;
import java.util.stream.Stream;

public class Player {

    private static final short WIDTH = 16000;
    private static final short HEIGHT = 9000;

    private static final short MOVE_UNITS = 1000;

    private static final short ZOMBIE_SHOT_RANGE = 2000;
    private static final short ZOMBIE_MOVE_UNITS = 400;

    public static void main(String args[]) {
        final Player player = new Player();
        while (true) {
            player.updatePositions();
            System.out.println(player.moveAsh());
        }
    }

    private final Scanner _in;
    private final Ash _ash;
    private final Map<Short, Human> _humans;
    private final Map<Short, Zombie> _zombies;

    public Player() {
        _in = new Scanner(System.in);
        _ash = new Ash();
        _humans = new HashMap<>();
        _zombies = new HashMap<>();
    }

    public void updatePositions() {
        _ash.updatePosition(_in.nextShort(), _in.nextShort());

        Stream.concat(_humans.values().stream(), _zombies.values().stream()).forEach(Entity::dead);

        final short humanCount = _in.nextShort();
        for (short i = 0; i < humanCount; i++) {
            final short humanId = _in.nextShort();
            Human human = _humans.get(humanId);
            if (human == null) {
                human = new Human();
                _humans.put(humanId, human);
            }
            human.alive();
            human.updatePosition(_in.nextShort(), _in.nextShort());
        }

        final short zombieCount = _in.nextShort();
        for (short i = 0; i < zombieCount; i++) {
            final short zombieId = _in.nextShort();
            Zombie zombie = _zombies.get(zombieId);
            if (zombie == null) {
                zombie = new Zombie();
                _zombies.put(zombieId, zombie);
            }
            zombie.alive();
            zombie.updatePosition(_in.nextShort(), _in.nextShort());
            zombie.updateNextPosition(_in.nextShort(), _in.nextShort());
        }
    }

    public Point moveAsh() {

        return null;
    }


}