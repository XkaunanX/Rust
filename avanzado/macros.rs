// Definición de una macro simple
macro_rules! saludar {
    () => {
        println!("¡Hola desde la macro!");
    };
}

fn main() {
    saludar!();  // Llamada a la macro sin parámetros

    // Macro con parámetros
    macro_rules! suma {
        ($x:expr, $y:expr) => {
            $x + $y
        };
    }
    let resultado = suma!(5, 3);  // Llamada a la macro con parámetros
    println!("La suma es: {}", resultado);
}
