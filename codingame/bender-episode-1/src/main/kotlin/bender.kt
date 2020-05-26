import java.util.*
import kotlin.collections.HashMap

private enum class Direction {
    NORTH, WEST, SOUTH, EAST
}

private data class Position(val x: Int, val y: Int) {
    fun to(direction: Direction): Position {
        return when (direction) {
            Direction.NORTH -> Position(x, y + 1)
            Direction.WEST -> Position(x - 1, y)
            Direction.SOUTH -> Position(x, y - 1)
            Direction.EAST -> Position(x + 1, y)
        }
    }

    override fun toString(): String {
        return "Position(x=$x, y=$y)"
    }
}

private data class Bender(var position: Position,
                          var direction: Direction = Direction.SOUTH,
                          var inverter: Boolean = false, var breaker: Boolean = false,
                          private var reset: Boolean = false) {

    fun rotate() { //TODO improve!
        direction = if (reset) {
            reset = false
            if (inverter) {
                Direction.WEST
            } else {
                Direction.SOUTH
            }
        } else if (inverter) {
            when (direction) {
                Direction.NORTH -> Direction.EAST
                Direction.WEST -> Direction.NORTH
                Direction.SOUTH -> Direction.WEST
                Direction.EAST -> Direction.SOUTH
            }
        } else {
            when (direction) {
                Direction.NORTH -> Direction.WEST
                Direction.WEST -> Direction.SOUTH
                Direction.SOUTH -> Direction.EAST
                Direction.EAST -> Direction.NORTH
            }
        }
    }

    fun moveTo(position: Position) {
        this.position = position
        reset = true
    }
}

private fun getMap(): HashMap<Position, Char> {
    val input = Scanner(System.`in`)
    val lines = input.nextInt()
    val columns = input.nextInt()
    if (input.hasNextLine()) input.nextLine()
    val map = HashMap<Position, Char>(lines * columns)
    for (y in 0 until lines) {
        val nextLine = input.nextLine()
        System.err.println(nextLine)
        nextLine.toCharArray().forEachIndexed { x, c ->
            map.put(Position(x, lines - 1 - y), c)
        }
    }
    return map
}

fun main(args: Array<String>) {
    val map = getMap()
    val bender = Bender(map.entries.find { it.value == '@' }!!.key)
    val teleports = map.entries.filter { it.value == 'T' }.map { it.key }
    var c = ' '
    while (c != '$') {
        val newPosition = bender.position.to(bender.direction)
        c = map.getValue(newPosition)
        System.err.println("$newPosition $c")
        if (c == '#' || (c == 'X' && !bender.breaker)) {
            val currentDir = bender.direction
            bender.rotate()
            System.err.println("rotation from $currentDir to ${bender.direction}")
            continue
        }
        println(bender.direction.name)
        bender.moveTo(newPosition)
        when (c) {
            'N' -> bender.direction = Direction.NORTH
            'W' -> bender.direction = Direction.WEST
            'S' -> bender.direction = Direction.SOUTH
            'E' -> bender.direction = Direction.EAST
            'I' -> bender.inverter = !bender.inverter
            'B' -> bender.breaker = !bender.breaker
            'T' -> bender.position = teleports.find { it != newPosition }!!
            'X' -> map.put(newPosition, ' ')
        }
    }
}