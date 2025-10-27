fn transpuesta(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    return result;
}

#[test]

fn test_transpose() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    let matriz_transpuesta = transpuesta(matrix);
    assert_eq!(matriz_transpuesta, [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ]);
}

fn main(){
    let matriz = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Matriz: {:#?}", matriz);
    let matriz_transpuesta = transpuesta(matriz);
    println!("Transpuesta: {:#?}", matriz_transpuesta);
}
