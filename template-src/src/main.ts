function fib(n: number): number  {
    if (n == 1) {
        return 1;
    }

    return fib(n - 1);
}

fib(17);