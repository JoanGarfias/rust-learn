# Variables

Rust ofrece seguridad de tipos mediante tipado estático. Las variables se declaran con
`let`.

Ejemplo:

```rust
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    /*ESTO DARÁ ERROR SL COMPILAR */
    //x = 32;
    //println!("x: {x}");
}
```

# Mutabilidad e Inmutabilidad

Por defecto, en Rust **las variables no pueden cambiar su valor**.
Por lo que en el código anterior, la definición de la variable `x` dos veces dará error al intentar compilar.

Para poder hacer que las variables sean mutables, es decir, que se pueda cambiar su valor en diferentes partes del código se utiliza la palabra reservada `mut`.


# Tipos de Datos
Tipo de dato | Literales | Ejemplo |
-------------|----------|------------
Enteros con signo | i8, i16, i32, i64, i128, isize | -10, 0, 1_000, 123_i64
Enteros sin signo | u8, u16, u32, u64, u128, usize | 0, 123, 10_u16
Números de coma flotante | f32, f64 | 3.14, -10.0e20, 2_f32
Valores escalares Unicode | char | 'a', 'α', '∞'
Booleanos | bool | true, false

# Sobre los tipos de datos

- `iN`, `uN` y `fN` son N bits de capacidad
- `isize` y `usize` tienen el ancho de un puntero
- `char` tiene un tamaño de 32 bits
- `bool` tiene 8 bits de ancho

> [!NOTE]
> Todos guiones bajos en los números pueden no utilizarse, ya que solo sirven para facilitar
la lectura. Por lo tanto, 1_000 se puede escribir como 1000 (o 10_00), y 123_i64 se
puede escribir como 123i64.

# Ejemplo de tipos de datos y valores

```rust
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
```
