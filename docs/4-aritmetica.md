# Operaciones en Rust

```rust
fn suma(a: f32, b: f32) -> f32 {
    return a + b;
}

fn resta(a: f32, b: f32) -> f32 {
    return a - b;
}

fn producto(a: f32, b: f32) -> f32 {
    return a * b;
}

fn division(a: f32, b: f32) -> f32 {
    if b != 0.0 {
        return a / b;
    }
    return 0.0;
}

fn main() {
    let num1: f32 = 92.5;
    let num2: f32 = 32.0;

    println!("{}", suma(num1, num2));
    println!("{}", resta(num1, num2));
    println!("{}", producto(num1, num2));
    println!("{}", division(num1, num2));
}
```

En el código anterior, se definieron las funciones `suma()`, `resta()`, `producto()`, `division()` para evaluar operaciones aritméticas.

# Imprimiendo valores de retorno de funciones con println!()

Para imprimir valores de variables o valores de torno de funciones podemos hacer lo siguiente:

```rust
println!("{}", funcioncita(...args));
```

> [!NOTE]
> De esta manera, Rust inferirá el tipo de dato y lo mostrará, aunque en el caso de las variables podemos
imprimirlas como se manejó en la sesión anterior. También se puede aplicar a variables.
