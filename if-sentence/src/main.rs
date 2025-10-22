fn main() {
    /*Las expresiones if se pueden usar de la misma forma que en otros lenguajes */
    let x = 29;

    if x == 0 {
        println!("Cero");
    }
    else if x < 100 { //De negativo a 99 es muy grande
        println!("Muy grande");
    }
    else{ //De 100 en adelante es enorme
        println!("Enorme");
    }
}
