/*Cuando uno trabaja con tuplas y otros valores estructurados, es común querer extraer valores
interiores a variables locales. Uno puede manualmente acceder los valores interiores:
*/
fn print_tuple(tuple: (i32, i32)) {
    let left = tuple.0;
    let right = tuple.1;
    println!("left: {left}, right: {right}");
}

/*
Rust también provee la coincidencia de patrones para destructurar un valor en sus partes
constituyentes:
*/
fn print_tuple_coincidencia(tuple: (i32, i32)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}

fn print_tuple_2(tuple: (i32, i32, i32)) {
    let (n1, n2, n3) = tuple;
    println!("n1: {n1}, n2: {n2}, n3: {n3}");
}


fn main() {

    let tuple = (1,2,3);

    print_tuple_2(tuple);

}
