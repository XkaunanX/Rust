// Definición de un trait
trait Describir {
    fn describir(&self) -> String;
}

// Estructura que implementa el trait Describir
struct Persona {
    nombre: String,
    edad: u32,
}

impl Describir for Persona {
    fn describir(&self) -> String {
        format!("{} tiene {} años.", self.nombre, self.edad)
    }
}

fn main() {
    let persona = Persona {
        nombre: String::from("Juan"),
        edad: 30,
    };

    // Llamar al método describir() implementado por el trait
    println!("{}", persona.describir());
}
