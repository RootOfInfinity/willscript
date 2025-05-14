fun fib(x) {
	if x < 2 {
		return 1;
	}

	var ans = fib(x-1) + fib(x-2);
	return ans;
}

fun loop(x) {
	if x > 10 {
		return fib(x);
	}
	var cool = fib(x);
	return loop(x+1);
}

fun main() {
	var ans = bestestLoop(1000000000);
	print ans;
	return 0;
}

fun bestestLoop(repeats) {
	var i = 0;
	while i < repeats {
		print i;
		i = i + 1;
	}
}

fun coolerLoop(number) {
	if number > 1000000000 {
		return number;
	}
	print number;
	return coolerLoop(number + 1);
}
