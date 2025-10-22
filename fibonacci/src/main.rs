fn fib(x: u32) -> u32{
    if x < 2 {
        return x;
    }
    return fib(x - 1 ) + fib(x - 2);
}


fn main() {
    //Se infiere el tipo de dato, lo que se vió en la sesión de inferencia de tipos
    let _n = 2;
    println!("{}", fib(2));
}
