use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hola desde un hilo!");
    });

    // Esperar a que el hilo termine
    handle.join().unwrap();

    println!("Hilo principal terminado.");
}
