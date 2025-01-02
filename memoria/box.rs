fn main() {
    let b = Box::new(5);  // `b` es un puntero inteligente Box que apunta a un valor en el heap
    println!("Valor de b: {}", b);
}
