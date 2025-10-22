fn main() {
    /* AMBITOS Y SHADOWING
     * El ambito de una variable se limita al bloque que la contiene
     * Se puede sombrear variables, tanto las de ámbitos externos como las del propio ámbito
     */

     let a = 10;

     println!("Antes: {a}");

     /*La variable "a" puede tomar diferentes valores en los bloques definidos
      * y al salir del bloque seguirá manteniendo su valor anterior antes de entrar
      * al bloque.
      */
     {
         let a = "hola";
         println!("Ambito interno: {a}");

         let a = true;
         println!("Sombreado en el ámbito interno: {a}");
     }

     println!("Después: {}", a);
}
