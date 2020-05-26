import org.junit.Test;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.Collection;
import java.util.function.UnaryOperator;
import java.util.stream.Stream;

import static org.junit.Assert.assertEquals;

interface Fibonacci {

	BigInteger fibonacci(int n);

}

public class FibonacciTest {

	private final Collection<Fibonacci> _fibonacciImpls = Arrays.asList( //
			//new RecursiveFibonacci() // very inefficient
			new IterativeFibonacci() //
			, new CorecursionFibonacci() //
			, new DivideAndConquerFibonacci() //
	);

	@Test
	public void mainTest() throws Exception {
		check(0, 0);
		check(1, 1);
		check(2, 1);
		check(3, 2);
		check(4, 3);
		check(5, 5);
		check(6, 8);
		check(7, 13);
		check(8, 21);
		check(9, 34);
		check(10, 55);

		check(50, 12586269025L);
		check(60, 1548008755920L);
		check(70, 190392490709135L);
		check(80, 23416728348467685L);
		check(90, 2880067194370816120L);
		check(100, "354224848179261915075");

		check(8181, "239001090710360059034248200673803309562195124933438825088385870209105768309267224930066773271004303009695857056812050426322722227488483596969330539198412751609689113829755775066752844437629935556689908621747058520170917953833076673228939285877150494526386620300621280749499924952199516712960736433814553231958282333619656314497995824452475174641352224677997408976231194557854106641619031011172157654286916061043356523159334857136487352779804235483277506977454306460042287968212874761824582897118739286429568840003151050106146828835563160817912048376040050029809912293013734791567749471727392937824065261113177259783202662957881148637632338195187490758787735996699022778723575367214258563034452504094360966531897568256418608645465915444745840473934322871426418866598642747848660145342643755366760919516317387477526541252807114293921792114970905075434450564838742451198888345673434700068960962172644679947794329807611771708249033661865248799511661306285140477533559743999464574932871122125066107105911374614630965320293086278694399936369060752395531165804412176996135810584035128447884802662630006754418904791563798389799016017336123177492245220148295507234160487497059285034564989541608419857951981398972834439558266427410836592525389894745439937417033358839088886819050208294080514041113275997534122520735761635971975621605403703050984275628628811283403426936742851082726036123336764016240562071825096262121405587203818756266733130406345518134166312225673215071500009165695469591411166981267241101113735558997083171850461315680070428706983814819412637005375477590183910679020180492817106735246177201410250973608332090435177967936901320342366183865669056306257798108871942566285065496557591483743343454453933506");
	}

	private void check(int n, String expected) {
		check(n, new BigInteger(expected));
	}

	private void check(int n, long expected) {
		check(n, BigInteger.valueOf(expected));
	}

	private void check(int n, BigInteger expected) {
		System.out.println();
		_fibonacciImpls.forEach(fibonacci -> {
			System.out.print("Checking " + fibonacci.getClass().getSimpleName() + "  n=" + n + "  expected=" + expected);
			final long startTime = System.nanoTime();
			BigInteger value = fibonacci.fibonacci(n);
			final long endTime = System.nanoTime();
			System.out.println("  time=" + (endTime - startTime) + "ns");
			assertEquals(expected, value);
		});
	}
}

class RecursiveFibonacci implements Fibonacci {

	@Override
	public BigInteger fibonacci(int n) {
		if (n < 2) {
			return BigInteger.valueOf(n);
		}
		return fibonacci(n - 1).add(fibonacci(n - 2));
	}

}

class IterativeFibonacci implements Fibonacci {

	@Override
	public BigInteger fibonacci(int n) {
		BigInteger i = BigInteger.ONE;
		BigInteger j = BigInteger.ZERO;
		for (int k = 0; k < n; k++) {
			BigInteger t = i.add(j);
			j = i;
			i = t;
		}
		return j;

	}
}

class CorecursionFibonacci implements Fibonacci {

	private static final Tuple<BigInteger, BigInteger> seed = new Tuple<>(BigInteger.ONE, BigInteger.ONE);
	private static final UnaryOperator<Tuple<BigInteger, BigInteger>> operator = x -> new Tuple<>(x._2, x._1.add(x._2));

	@Override
	public BigInteger fibonacci(int n) {
		if (n == 0) {
			return BigInteger.ZERO;
		}
		return Stream.iterate(seed, operator).map(x -> x._1).skip(n - 1).findFirst().get();
	}

	static class Tuple<T, U> {
		final T _1;
		final U _2;

		Tuple(T t, U u) {
			_1 = t;
			_2 = u;
		}
	}
}

class DivideAndConquerFibonacci implements Fibonacci {

	@Override
	public BigInteger fibonacci(int n) {
		if (n <= 0) {
			return BigInteger.ZERO;
		}
		int i = n - 1;
		BigInteger aux1 = BigInteger.ZERO;
		BigInteger aux2 = BigInteger.ONE;
		BigInteger a = aux2;
		BigInteger b = aux1;
		BigInteger c = aux1;
		BigInteger d = aux2;
		while (i > 0) {
			if (i % 2 != 0) {
				aux1 = d.multiply(b).add(c.multiply(a));
				aux2 = d.multiply(b.add(a)).add(c.multiply(b));
				a = aux1;
				b = aux2;
			}
			aux1 = c.pow(2).add(d.pow(2));
			aux2 = d.multiply(c.multiply(BigInteger.valueOf(2)).add(d));
			c = aux1;
			d = aux2;
			i = i / 2;
		}
		return a.add(b);
	}
}