fn encontrar_numero(vec: Vec<i32>, valor: i32) -> Option<i32> {
    for &numero in vec.iter() {
        if numero == valor {
            return Some(numero);  // Si encuentra el valor, lo devuelve dentro de Some
        }
    }
    None  // Si no lo encuentra, retorna None
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    
    match encontrar_numero(numeros.clone(), 3) {
        Some(numero) => println!("Número encontrado: {}", numero),
        None => println!("Número no encontrado"),
    }

    match encontrar_numero(numeros.clone(), 6) {
        Some(numero) => println!("Número encontrado: {}", numero),
        None => println!("Número no encontrado"),
    }
}
