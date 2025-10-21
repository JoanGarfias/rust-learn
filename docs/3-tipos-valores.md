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
