fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("No se puede dividir por cero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let resultado = dividir(10, 2);
    match resultado {
        Ok(valor) => println!("El resultado de la división es: {}", valor),
        Err(e) => println!("Error: {}", e),
    }

    // Otro ejemplo con división por cero
    let resultado_erroneo = dividir(10, 0);
    match resultado_erroneo {
        Ok(valor) => println!("El resultado de la división es: {}", valor),
        Err(e) => println!("Error: {}", e),
    }
}
