fn main() {
    let s1 = String::from("Rust");

    // Referencia inmutable (no se puede modificar a través de esta referencia)
    let s2 = &s1;
    println!("s1: {}, s2: {}", s1, s2);

    // Referencia mutable (se puede modificar a través de esta referencia)
    let mut s3 = String::from("Programming");
    let s4 = &mut s3;
    s4.push_str(" is fun!");
    println!("s3: {}", s3);

    // No puedes tener una referencia mutable mientras haya referencias inmutables activas
    // let s5 = &s1;  // Error: no se puede tener una referencia mutable y otra inmutable a la misma vez
}
