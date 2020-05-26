import java.util.*;
import java.util.stream.Collectors;

class Player {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);
        final int playerId = in.nextInt(); // your id (0 to 4)

        final Map<Integer, Chip> chips = new HashMap<>();

        // game loop
        while (true) {
            chips.values().forEach(chip -> chip.inGame = false);

            in.nextInt(); // The number of chips under your control
            final int entityCount = in.nextInt(); // The total number of entities on the table, including your chips
            for (int i = 0; i < entityCount; i++) {
                Chip chip = chips.computeIfAbsent(in.nextInt(), Chip::new);
                chip.playerId = in.nextInt();
                chip.radius = in.nextFloat();
                chip.x = in.nextFloat();
                chip.y = in.nextFloat();
                chip.vx = in.nextFloat();
                chip.vy = in.nextFloat();
                chip.inGame = true;
                chip.action = "WAIT";
                chip.target = null;
            }

            ListIterator<Chip> toProcess = chips.values().stream()
                    .filter(chip -> chip.playerId == playerId)  //
                    .filter(chip -> chip.inGame)                //
                    .collect(Collectors.toList()).listIterator();
            while (toProcess.hasNext()) {
                final Chip chip = toProcess.next();
                final float nr = chip.radius * 14 / 15;
                List<Chip> targets = chips.values().stream()
                        .filter(c -> c.playerId != playerId)                                            //
                        .filter(c -> c.inGame)                                                          //
                        .filter(c -> c.radius < nr)                                                     //
                        .sorted((c1, c2) -> Double.compare(chip.distanceTo(c1), chip.distanceTo(c2)))   //
                        .collect(Collectors.toList());
                if (targets.isEmpty()) {
                    continue; //TODO run away?
                }
                for (Chip target : targets) {
                    if (target.target == null) {
                        target.target = chip;
                        chip.target = target;
                        chip.action = target.x + " " + target.y;
                        break;
                    }
                    if (target.distanceTo(target.target) > target.distanceTo(chip)) {
                        toProcess.add(target.target);
                        target.target = chip;
                        chip.target = target;
                        chip.action = target.x + " " + target.y;
                        break;
                    }
                }

            }

            //print actions
            chips.values().stream()
                    .filter(chip -> chip.playerId == playerId)                  //
                    .filter(chip -> chip.inGame)                                //
                    .sorted((c1, c2) -> Integer.compare(c1.id, c2.id))          //
                    .forEachOrdered(chip -> System.out.println(chip.action));
        }
    }
}

class Chip {

    //from input
    final int id;
    int playerId;
    float radius;
    float x;
    float y;
    float vx;
    float vy;

    //processed
    boolean inGame;
    String action;
    Chip target;

    Chip(int id) {
        this.id = id;
    }

    double distanceTo(Chip other) {
        return Math.sqrt(Math.pow(other.x - x, 2) + Math.pow(other.y - y, 2));
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Chip chip = (Chip) o;
        return id == chip.id;
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }
}