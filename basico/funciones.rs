// Función sin parámetros y sin valor de retorno
fn saludo() {
    println!("¡Hola, Rust!");
}

// Función con parámetros y valor de retorno
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    saludo();  // Llamada a la función saludo
    let resultado = sumar(5, 7);  // Llamada a la función sumar
    println!("La suma es: {}", resultado);
}
