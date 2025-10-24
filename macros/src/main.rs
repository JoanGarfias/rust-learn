/*
 * MACROS en Rust
 *
 * Las macros se amplían a código de Rust durante la implementación y pueden adoptar un número variable
 * de argumentos. Se distinguen por utilizar el simbolo ! al final. La blbioteca estándar de Rust
 * incluye una serie de macros útiles.
 *
 * println!(format, ..) imprime una linea a la salida estándar (stdout) indicando
 * el formato descrito en stf:fmt.
 *
 * format!(format, ...) funciones igual que println!, pero devuelve el resultado en forma de cadena.
 *
 * dbg!(expression) registra el valor de la expresión y lo devuelve
 *
 * todo!() marca un fragmento de código como no implementado todavía. Si se ejecuta, activará un error pánico.
 *
 * unreachable!() marca un fragmento de código como innacesible. Si se activa, activará un error de pánico.
 */


fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i); //heramienta de depuración
    }

    return product;
}

fn fizzbuzz(_n: u32) -> u32 { //Función no implementada
    todo!()
}



fn main() {
    println!("{}", factorial(5));
}
