fn main() {
    let s = String::from("Hello, Rust!");

    // Slice de una cadena
    let hello = &s[0..5];  // Toma el substring "Hello"
    println!("slice: {}", hello);

    // Slice de un vector
    let v = vec![1, 2, 3, 4, 5];
    let v_slice = &v[1..4];  // Toma el slice [2, 3, 4]
    println!("vector slice: {:?}", v_slice);
}
