fn main() {

    /* LOOP
     * LOOP termina hasta que encuentre una sentencia break
     */

    let mut i = 0;

    println!();
    loop {
        i += 1;

        if i % 2 == 0 { //Si es par omitimos su impresión
            continue;
        }

        if i >= 100 { //Detenemos el loop cuando lleguemos a 100
            break;
        }

        print!("{} ", i);
    }
    println!();
}
