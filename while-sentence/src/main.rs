fn main() {
    /* BUCLE WHILE
     * El bucle while es muy similar a otros lenguajes
     * aunque debemos tomar en cuenta que las variables que
     * controlan el flujo de un bucle deben ser variables mutables
     * es decir, declaradas con la palabra reservada "mut"
     */

     let mut x = 200;
     while x <= 500 {
         if(x % 100 == 0){
            println!("\nEncontré una centena {x}");
         }
         else{
            print!("{x} ");
         }
         x+=1;
     }

     println!();
}
