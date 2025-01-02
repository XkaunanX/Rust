use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    // Insertar elementos en el HashMap
    map.insert("clave1", 10);
    map.insert("clave2", 20);

    // Acceder a los valores por clave
    let valor = map.get("clave1");
    match valor {
        Some(v) => println!("Valor para 'clave1': {}", v),
        None => println!("No se encontró la clave"),
    }

    // Iterar sobre el HashMap
    for (key, value) in &map {
        println!("Clave: {}, Valor: {}", key, value);
    }

    // Eliminar un valor por su clave
    map.remove("clave2");
    println!("Después de eliminar clave2, HashMap: {:?}", map);
}
