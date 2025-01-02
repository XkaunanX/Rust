fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    // Crear un iterador sobre el vector
    let mut iter = vec.iter();

    // Usar el método next() para obtener los elementos uno por uno
    while let Some(val) = iter.next() {
        println!("Valor del iterador: {}", val);
    }

    // Usar un iterador para realizar transformaciones
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("Valores duplicados: {:?}", doubled);

    // Usar un iterador con filtros
    let even_numbers: Vec<i32> = vec.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Números pares: {:?}", even_numbers);
}
