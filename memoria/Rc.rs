use std::rc::Rc;

fn main() {
    let s = Rc::new(String::from("Hola, Rust!"));

    let t = Rc::clone(&s);  // Clonación de referencia (no crea una copia profunda, solo cuenta las referencias)
    println!("s: {}", s);
    println!("t: {}", t);

    println!("Número de referencias a s: {}", Rc::strong_count(&s));  // Cuenta las referencias
}
