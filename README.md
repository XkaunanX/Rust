# RUST

Rust es un lenguaje de programacion de sistemas enfocado en el rendimiento, la seguridad y la concurrencia. Se destaca por su capacidad para prevenir errores comunes en el manejo de memoria sin perder el control sobre el rendimiento. Rust ofrece una sintaxis moderna y segura, permitiendo escribir programas altamente optimizados sin sacrificar la seguridad o la facilidad de uso.

## En qué contexto nació Rust

Rust fue creado por Mozilla en 2010 como un lenguaje de programación de sistemas, con el objetivo de ofrecer un rendimiento similar al de C y C++, pero sin los riesgos asociados al manejo manual de memoria. Rust se diseñó con un enfoque en la seguridad y la concurrencia, resolviendo problemas comunes en la programación de sistemas como la gestión de memoria, las condiciones de carrera y los errores en tiempo de ejecución.

## Características Principales

### Bajo o Alto Nivel

Rust es considerado un lenguaje de bajo nivel debido a su cercanía al hardware y su capacidad para interactuar con el sistema operativo a nivel de memoria. Sin embargo, ofrece características de alto nivel, como un sistema de tipos avanzado, manejo de errores y concurrencia, que lo hacen adecuado para tareas de alto rendimiento y programación de sistemas complejos.

### Uso de Punteros

Rust no permite punteros nulos o punteros colgantes, evitando así muchos errores comunes en la manipulación de memoria. Sin embargo, permite el uso controlado de punteros mediante el sistema de "referencias", lo que ayuda a garantizar la seguridad sin perder el control del rendimiento.

```rust
fn main() {
    let x = 5;
    
    // Referencia inmutable
    let y = &x;
    println!("y: {}", y);
    
    // Referencia mutable
    let mut a = 10;
    let b = &mut a;
    *b += 5; // Desreferenciamos para modificar el valor
    println!("a: {}", a);
}
```

### Paradigma y Explicación del Paradigma

Rust adopta un enfoque multi-paradigma, combinando programación imperativa, orientada a objetos y funcional. Permite escribir código modular y reutilizable, favoreciendo el uso de expresiones en lugar de sentencias. Además, enfatiza el control explícito sobre el flujo de ejecución y el manejo eficiente de los recursos del sistema.

```rust
// Ejemplo de Programación Imperativa
let mut sum = 0;
for i in 1..=5 {
    sum += i;
}
println!("Suma: {}", sum);

// Ejemplo de Programación Orientada a Objetos con Structs y Traits
struct Persona {
    nombre: String,
}

impl Persona {
    fn saludar(&self) {
        println!("Hola, mi nombre es {}", self.nombre);
    }
}

// Ejemplo de Programación Funcional
let numeros = vec![1, 2, 3, 4, 5];
let resultado: i32 = numeros.iter().map(|x| x * 2).sum();
println!("Resultado: {}", resultado);
```

### Tipo de Tipado

Rust utiliza tipado estático y fuerte, lo que significa que los tipos de datos son verificables en tiempo de compilación, evitando muchos errores comunes en la ejecución. Además, el sistema de tipos es inferido automáticamente en muchas situaciones, pero también permite especificar tipos explícitamente cuando es necesario.

### Compilado

Rust es un lenguaje compilado, lo que significa que el código fuente es transformado en un ejecutable nativo antes de ser ejecutado. Esto permite una ejecución rápida y controlada, optimizando al máximo el uso de los recursos del sistema.

### Rust en la Web

Rust se está utilizando cada vez más en el desarrollo web, principalmente mediante WebAssembly (Wasm), lo que permite ejecutar código Rust en navegadores web con un rendimiento cercano al nativo.

# ¿Qué es WebAssembly?

WebAssembly (Wasm) es una tecnología que permite ejecutar código binario de bajo nivel de manera eficiente en los navegadores web. Está diseñado para ser un formato portátil, seguro y rápido que puede ser ejecutado en la mayoría de los navegadores modernos.

## Propósito de WebAssembly

WebAssembly fue creado para abordar las limitaciones de rendimiento de JavaScript en el navegador. Aunque JavaScript es un lenguaje extremadamente popular para el desarrollo web, en ciertas aplicaciones de alto rendimiento, como juegos, edición de imágenes o procesamiento de datos pesados, JavaScript puede no ser suficiente. WebAssembly resuelve este problema al permitir que el código compilado en otros lenguajes, como C, C++ o Rust, se ejecute en el navegador con un rendimiento cercano al nativo.

## Características Clave

### 1. **Alto rendimiento**
WebAssembly permite ejecutar código de bajo nivel de manera más rápida y eficiente que JavaScript. Los motores de WebAssembly están optimizados para el rendimiento, lo que lo convierte en una excelente opción para aplicaciones que requieren procesamiento intensivo.

### 2. **Portabilidad**
El código WebAssembly es portable y puede ejecutarse en cualquier plataforma que soporte los navegadores modernos. Esto incluye dispositivos de escritorio, móviles, y otros dispositivos con un motor compatible de WebAssembly.

### 3. **Compatibilidad con lenguajes de alto nivel**
Aunque WebAssembly no es un lenguaje de programación en sí mismo, permite que el código escrito en lenguajes como C, C++, Rust, y otros, sea compilado a WebAssembly y ejecutado en el navegador.

### 4. **Seguridad**
WebAssembly está diseñado para ejecutarse en un entorno aislado (sandboxed), lo que significa que no tiene acceso directo al sistema de archivos del usuario ni a recursos fuera del entorno web sin restricciones. Esto ayuda a garantizar la seguridad en la ejecución del código.

### 5. **Integración con JavaScript**
WebAssembly no reemplaza a JavaScript, sino que lo complementa. Puede integrarse fácilmente con JavaScript para aprovechar las capacidades del navegador, lo que permite a los desarrolladores escribir aplicaciones híbridas que combinan lo mejor de ambos mundos.

## ¿Cómo Funciona WebAssembly?

WebAssembly funciona tomando código escrito en lenguajes como C, C++, o Rust y lo compilando a un formato binario optimizado (Wasm). Este archivo binario es cargado por el navegador, y el motor de WebAssembly del navegador lo ejecuta de manera eficiente. En muchos casos, el código WebAssembly se ejecuta en paralelo con el código JavaScript, lo que permite una integración fluida entre ambos.

### Manejo de Memoria

Rust tiene un sistema de manejo de memoria único que evita la necesidad de un recolector de basura. A través de conceptos como ownership y borrowing, Rust asegura que la memoria se gestione de manera eficiente y segura sin riesgo de fugas de memoria o condiciones de carrera.

```rust
fn main() {
    let s1 = String::from("Hola, mundo!");  // Ownership de la variable s1
    let s2 = &s1;  // Borrowing: prestamos la referencia de s1 a s2

    println!("s1: {}", s1);  // s1 todavía es válido, ya que prestamos solo la referencia
    println!("s2: {}", s2);  // s2 es una referencia válida de s1

    let s3 = s1;  // s1 transfiere su propiedad a s3
    // println!("s1: {}", s1);  // Esto causaría un error porque s1 ya no es válido después de la transferencia de propiedad
    println!("s3: {}", s3);  // s3 tiene la propiedad de la cadena
}
```

### Manejo de Errores

Rust promueve el uso de tipos de error explícitos, como `Result` y `Option`, en lugar de depender de excepciones. Esto permite un manejo de errores más predecible y explícito, reduciendo la probabilidad de fallos inesperados en tiempo de ejecución.

```rust
// Usamos Result para manejar operaciones que pueden fallar
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("No se puede dividir entre cero".to_string())  // Retornamos un error
    } else {
        Ok(a / b)  // Retornamos el resultado si la división es exitosa
    }
}

// Usamos Option para representar valores que pueden ser nulos
fn obtener_elemento(vec: Vec<i32>, index: usize) -> Option<i32> {
    if index < vec.len() {
        Some(vec[index])  // Retornamos el valor si el índice es válido
    } else {
        None  // Retornamos None si el índice está fuera de rango
    }
}

fn main() {
    // Ejemplo con Result
    let resultado = dividir(10, 0);
    match resultado {
        Ok(valor) => println!("Resultado de la división: {}", valor),
        Err(e) => println!("Error: {}", e),
    }

    // Ejemplo con Option
    let vec = vec![1, 2, 3];
    let elemento = obtener_elemento(vec, 1);
    match elemento {
        Some(valor) => println!("Elemento encontrado: {}", valor),
        None => println!("Elemento no encontrado"),
    }
}
```

### Concurrencia

Rust permite la programación concurrente de manera segura gracias a su sistema de ownership y borrowing. Esto permite escribir programas que aprovechen múltiples hilos sin riesgo de condiciones de carrera, lo que facilita la creación de aplicaciones altamente concurrentes.

```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    // Creamos 10 hilos que imprimen un mensaje
    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("Hola desde el hilo {}", i);
        });
        handles.push(handle);
    }

    // Esperamos a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Todos los hilos han terminado.");
}
```

## Propiedades y Conceptos Clave

### Ownership

El sistema de ownership es una característica clave en Rust que garantiza que solo haya un propietario de un recurso en un momento dado. Esto permite la liberación automática de memoria cuando un recurso deja de ser utilizado, evitando fugas de memoria.

### Borrowing

El borrowing permite que una función acceda a los datos de otra sin tomar posesión de ellos, a través de referencias. Existen referencias inmutables y mutables, cada una con sus restricciones para asegurar la seguridad y evitar conflictos de acceso concurrente.

### Estructuras (Structs)

Las estructuras (`structs`) son tipos de datos compuestos que permiten agrupar varios valores de diferentes tipos. Son muy útiles para representar entidades complejas en un programa.

### Canales

Los canales son una herramienta fundamental para la comunicación entre hilos en Rust. Permiten pasar mensajes de manera segura entre diferentes hilos de ejecución, facilitando la programación concurrente.

### Clonación de Referencia

En Rust, es posible clonar referencias mediante el uso de `Clone`. Sin embargo, la clonación no implica copiar la memoria, sino que crea una nueva referencia a los mismos datos.

### RefCell

`RefCell` es un tipo que permite mutar datos incluso cuando la estructura que lo contiene es inmutable, pero garantiza que las referencias mutables sean controladas en tiempo de ejecución para asegurar la seguridad.

### Referencias Inmutables y Mutables

Rust distingue entre referencias inmutables (que permiten leer pero no modificar los datos) y mutables (que permiten modificar los datos). Esto es crucial para el control de acceso seguro a los datos en programas concurrentes.

### Boxes

Un `Box` es un tipo que permite almacenar datos en el heap en lugar de la pila, proporcionando un control más flexible sobre el ciclo de vida de los datos.

### Lifetimes

Los lifetimes en Rust son una forma de garantizar que las referencias sean válidas durante el tiempo adecuado, evitando accesos a memoria inválida.

### Macros

Las macros en Rust permiten la metaprogramación, generando código en tiempo de compilación. Son útiles para escribir código más flexible y reutilizable.

### Traits

Los `traits` permiten definir comportamientos comunes que pueden ser implementados por diferentes tipos. Son similares a las interfaces en otros lenguajes y facilitan la programación orientada a objetos.

### Unsafe

El bloque `unsafe` en Rust permite realizar operaciones que no son verificadas por el compilador, lo que proporciona más control sobre el sistema. Sin embargo, el uso de `unsafe` debe ser limitado y debe ser cuidadosamente gestionado para evitar introducir errores de memoria o concurrencia.
