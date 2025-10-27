/*
 * Es como un array pero no se puede modificar
 * Se accede a los elementos por indice (con notación de punto, no de corchetes)
 * Al igual que los arrays, las tuplas tienen un tamaño fijo en tiempo de compilación
 */

fn main() {
    let t: (i8, bool, String) = (7, true, String::from("Hello"));

    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
    println!("t.2: {}", t.2);
}
