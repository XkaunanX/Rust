fn main() {
    // Ejemplo de código inseguro que manipula punteros
    let x = 10;
    let r = &x as *const i32; // Crear un puntero crudo

    unsafe {
        // Acceder al valor del puntero de manera insegura
        println!("Valor de x: {}", *r);
    }

    // Usar unsafe para llamar a una función externa no segura
    unsafe fn saludar() {
        println!("¡Hola desde una función unsafe!");
    }

    unsafe {
        saludar();
    }
}
