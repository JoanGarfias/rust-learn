fn main() {
    /* BLOQUES
     * En Rust, un bloque contiene una secuencia de expresiones rodeados por llaves {}.
     * Cada bloque tiene el tipo y valor de la última expresión del bloque.
     */

     let z = 13;
     let x = {
         let y = 10;
         println!("y: {y}");
         z - y //Se queda con este último valor, es decir 13 - 10
     };

     println!("x: {x}");

     /*
      * Esto nos podría servir cuando queramos que el valor de una variable
      * dependa del valor de otras variables aplicando operaciones sobre ellas.
      */
}
