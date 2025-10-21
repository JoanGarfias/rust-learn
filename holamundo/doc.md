Esta carpeta esta creada con cargo.

# Crear proyecto de Rust con cargo

> cargo new <proyecto>

# Compilar archivos de Rust sin optimización
> cargo build

# Compilar de forma optimizada (más tardada, ideal para productivo)
> cargo build --release


# Ejecutar proyecto de Rust
> [!NOTE]
> Este comando compila y ejecuta el proyecto de Rust.
> cargo run

# Limpiar proyecto de Rust
> [!NOTE]
> Este comando elimina los archivos generados por el compilador.
> cargo clean

# Revisar sintaxis del código sin tener que generar el ejecutable

> cargo check


# Cargo.toml (Tom's Obvious, Minimal Language)

Contiene una especificación de paquetes, información del proyecto y el conjunto de dependencias.

# Diferencias entre macro y función

El simbolo **!** significa que es una macro, por ejemplo `println!()` y `print!()`
