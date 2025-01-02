fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("¡Error! Intentando dividir por cero.");
    } else {
        a / b
    }
}

fn main() {
    // Usando panic
    println!("Resultado: {}", dividir(10, 2));
    
    // Esto causará un panic porque intentamos dividir por cero
    // println!("Resultado: {}", dividir(10, 0));

    // Usando unwrap con Option
    let numero: Option<i32> = Some(10);
    println!("El valor de numero es: {}", numero.unwrap());  // Desempaqueta el valor

    // Usando unwrap con un None, lo que causará un panic
    let ninguno: Option<i32> = None;
    // println!("El valor de ninguno es: {}", ninguno.unwrap());  // Esto causará panic
}
