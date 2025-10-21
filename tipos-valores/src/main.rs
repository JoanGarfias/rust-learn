fn main() {
    let x: i32 = 10;
    println!("x inmutable: {x}");
    let mut y: i32 = 20;
    println!("y mutable: {y}");
    y = 82;
    println!("y mutable: {y}");

    //Abarca desde -128 hasta 127
    let entero_8: i8 = 127;
    println!("Entero de 8 bits: {entero_8}");

    //Abarca desde -32768 hasta 32767
    let entero_16: i16 = 32767;
    println!("Entero de 16 bits: {entero_16}");

    //Abarca desde -2147483648 hasta 2147483647
    let entero_32: i32 = 2147483647;
    println!("Entero de 32 bits: {entero_32}");

    let entero_64: i64 = 932932328382;
    println!("Entero de 64 bits: {entero_64}");

    let entero_128: i128 = -8828328392939293;
    println!("Entero de 128 bits: {entero_128}");


    let flotante_32: f32 = 292.329;
    println!("Flotante de 32: {:.}", flotante_32);

    let flotante_64: f64 = 293293.3293;
    println!("Flotante de 64: {:.}", flotante_64);

    let escalar1: char = 'j';
    let escalar2: char = 'o';
    let escalar3: char = 'a';
    let escalar4: char = 'n';

    println!("{escalar1}{escalar2}{escalar3}{escalar4}");

    let booleano: bool = false;
    println!("{booleano}");

}
