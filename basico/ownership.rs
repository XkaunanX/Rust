fn main() {
    // Propiedad (Ownership)
    let s1 = String::from("Rust");  // s1 posee la cadena
    let s2 = s1;  // Aquí, la propiedad de s1 se mueve a s2, y s1 ya no puede usarse
    // println!("{}", s1);  // Esto causaría un error porque s1 ya no es válido

    // Préstamo (Borrowing)
    let s3 = String::from("Rust");
    let len = longitud(&s3);  // Referencia prestada, s3 sigue siendo válida
    println!("La longitud de la cadena es: {}", len);
}

fn longitud(s: &String) -> usize {
    s.len()  // El préstamo solo permite leer, no modificar
}
