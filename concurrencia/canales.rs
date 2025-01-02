use std::sync::mpsc;
use std::thread;

fn main() {
    // Crear un canal
    let (tx, rx) = mpsc::channel();

    // Hilo que envía datos por el canal
    thread::spawn(move || {
        let mensaje = "Hola desde el hilo!";
        tx.send(mensaje).unwrap();
    });

    // Recibir datos en el hilo principal
    let recibido = rx.recv().unwrap();
    println!("Mensaje recibido: {}", recibido);
}
