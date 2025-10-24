/**
 * ARREGLOS en Rust
 *
 * Un valor del tipo array [T; N] contiene N (una constante en tiempo de compilación)
 * elemntos del mismo tipo T. Ten en cuenta que la longitud del array es parte de su tipo
 * Lo que significa que [u8; 3] y [u8: 4] se consideran dos tipos diferentes. Los slices,
 * que tienen un tamaño determinado al tiempo de ejecución, serán discutidos más tarde.
 *
 */

fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;

    println!("a: {a:?}");
    //a[11] = 9; ESTO DA ERROR AL COMPILAR

    //Impresión con formatos separado por saltos de línea.
    println!("a: {a:#?}");
}
