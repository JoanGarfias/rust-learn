/**
 * Una referencia ofrece una forma de acceder a otro valor si nasumir la responsabilidad
 * del valor. También se denomina "préstamo". Las referencias compartidas son de SOLO LECTURA
 * y los datos a los que se hace referencia no pueden cambiar.
 */


 /*
  * Una referencia compartida a un tipo T tiene el tipo &T. Se crea un valor
  * de referencia con el operador &. El operador * "desreferencia" a una referencia
  * dando lugar a su valor.
  *
  * Rust prohibirá estáticamente las referencias colgantes
  */

fn x_axis(x: &i32) -> &(i32, i32){
    let point = (*x, 0);
    return &point;
}

fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", r);
    r = &b; //r es igual a la dirección de memoria de b
    println!("r: {}", *r); //El contenido de r
}
