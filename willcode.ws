fun fib(x) {
	if (x < 2) {
		return 1;
	}

	var ans = fib(x-1) + fib(x-2);
	return ans;
}

fun loop(x) {
	if (x > 1000) {
		return fib(x);
	}
	var cool = fib(x);
	var cooler = loop(x+1);
	return cooler;
}

fun main() {
	var ans = loop(0);
	print ans;
	return 0;
}
