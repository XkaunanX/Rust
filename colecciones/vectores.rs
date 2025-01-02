fn main() {
    // Crear un vector vacío
    let mut vec: Vec<i32> = Vec::new();

    // Agregar elementos al vector
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Acceder a los elementos usando índices
    println!("Primer elemento: {}", vec[0]);
    println!("Segundo elemento: {}", vec[1]);

    // Iterar sobre los elementos del vector
    for val in &vec {
        println!("Valor en el vector: {}", val);
    }

    // Eliminar el último elemento
    let last = vec.pop();
    match last {
        Some(value) => println!("Eliminado el valor: {}", value),
        None => println!("El vector estaba vacío"),
    }
}
