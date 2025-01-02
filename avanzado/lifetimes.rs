// Función que toma dos referencias con diferentes lifetimes
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let str1 = String::from("Hola");
    let str2 = String::from("Mundo");

    // Llamamos a la función longest con referencias a los strings
    let result = longest(&str1, &str2);
    println!("La cadena más larga es: {}", result);
}
