fn main() {
    let mut s = String::from("Hola");

    // Concatenar cadenas
    s.push_str(", Mundo!");  // Agregar una cadena
    println!("Cadena concatenada: {}", s);

    // Convertir a mayúsculas
    let upper = s.to_uppercase();
    println!("Cadena en mayúsculas: {}", upper);

    // Obtener longitud de la cadena
    let len = s.len();
    println!("Longitud de la cadena: {}", len);

    // Convertir un String a un slice (referencia a una porción de la cadena)
    let slice = &s[0..4]; // "Hola"
    println!("Primer palabra: {}", slice);

    // Iterar sobre los caracteres de la cadena
    for c in s.chars() {
        println!("{}", c);
    }
}
