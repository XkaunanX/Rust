fn main() {
    // Declaración de variables
    let x = 5;  // Variable inmutable
    let mut y = 10;  // Variable mutable

    println!("x: {}", x);
    println!("y: {}", y);

    // Modificar variable mutable
    y += 5;
    println!("Nuevo valor de y: {}", y);
}
