/**
 * FUNCIONES EN RUST
 * Los parámetros de declaración van seguidos de un tipo (al contrario que en algunos lenguajes
 * de programación como Python y JavaScript) y, a continuación, de un tipo de resultado devuelto.
 *
 * La última expresión del cuerpo de una función (o de cualquier bloque se convierte en el valor devuelto.
 * Basta con omitir el carácteres ; al final de la expresión. La palbra clave return
 * puede ser utilizado para devolver valores antes del fin de la función,
 * pero la sintaxis de "valor desnudo" es idiomático al fin de una función.
 *
 * Algunas funciones no devuelven nigún valor, devuelven el "tipo unitario", (), el compilador
 * deducirá esto si se omite el tipo de retorno -> ().
 *
 * En Rust, NO EXISTE la sobrecarga de funciones. Cada función tiene una única implementación.
 *  - siempre toma un núero fijo de parámetros. No se admite nargumentos predeterminados. Las macros se pueden
 *   utilizar para admitir funciones variáticas.
 *  - Siempre se utiliza un solo conjunto de tipos de parámetros. Estos tipos pueden ser genéricos.
 */


fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        return gcd(b, a % b);
    }
    else{
        return a;
    }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}
