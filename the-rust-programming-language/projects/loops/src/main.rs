fn main() {
    for number in (1..10).rev()
    {
        println!("{}! = {}", number, fib(number));
    }
}

fn fib(n: i32) -> i32
{
    if n == 0 { return 1 };
    return n * fib(n-1);
}