/* Rust consultará cómo se usa la variable para determinar el tipo. */

fn takes_u32(x: u32){
    println!("u32: {x}");
}

fn takes_i8(y: i8){
    println!("i8: {y}");
}


fn main() {
    let x = 10;
    let y = 20;

    /*No es que sea un tipado dinámico "cualquier tipo de dato",
     * sino que el compilador hace el trabajo por nosotros y nos ayuda a escribir
     * código más conciso.
     *
     * Cuando ningún elemento restringe el tipo de un entero, RUst lo define
     * de forma predeterminada como i32. A veces aparece como integer en los mensajes de error
     * Del mismo modo, los literales de punto flotante se definen como f64 de forma predeterminada.
      */
    takes_u32(x);
    takes_i8(y);


    //assert_eq!(x, y);
    //ERROR: No hay implementación para float === integer
}
