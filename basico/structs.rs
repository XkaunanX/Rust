// Definición de una estructura
struct Persona {
    nombre: String,
    edad: u32,
}

impl Persona {
    // Método asociado a la estructura
    fn nuevo(nombre: &str, edad: u32) -> Persona {
        Persona {
            nombre: String::from(nombre),
            edad,
        }
    }

    fn saludo(&self) {
        println!("¡Hola, mi nombre es {} y tengo {} años!", self.nombre, self.edad);
    }
}

fn main() {
    let persona1 = Persona::nuevo("Juan", 30);  // Crear una nueva instancia de Persona
    persona1.saludo();  // Llamar al método de la estructura
}
