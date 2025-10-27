/**
 * La instrucción for permite iterar sobre arrays, pero no sobre tuplas.
 *
 * assert!(expr) : comprobar que una condición es verdadera.
 * assert_ne!(expr1, expr2) : comprobar que dos expresiones son diferentes.
 * assert_eq!(expr1, expr2) : comprobar que dos expresiones son iguales.
 */


fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}
