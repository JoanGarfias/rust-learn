fn suma(a: f32, b: f32) -> f32 {
    return a + b;
}

fn resta(a: f32, b: f32) -> f32 {
    return a - b;
}

fn producto(a: f32, b: f32) -> f32 {
    return a * b;
}

fn division(a: f32, b: f32) -> f32 {
    if b != 0.0 {
        return a / b;
    }
    return 0.0;
}

fn main() {
    let num1: f32 = 92.5;
    let num2: f32 = 32.0;

    println!("{}", suma(num1, num2));
    println!("{}", resta(num1, num2));
    println!("{}", producto(num1, num2));
    println!("{}", division(num1, num2));
}
