fn collatz(num : u32) -> () {
    print!("{num} ");
    if num == 1 {
        return;
    }
    else if num % 2 == 0 {
        return collatz(num / 2);
    }
    else{
        return collatz((3 * num) + 1);
    }
}

fn main() {
    collatz(3);
}
