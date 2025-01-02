use tokio::time::Duration;

async fn tarea_asincrona() {
    println!("Tarea asincrónica iniciada");
    tokio::time::sleep(Duration::from_secs(2)).await; // Simula trabajo asincrónico
    println!("Tarea asincrónica terminada");
}

#[tokio::main]
async fn main() {
    let tarea1 = tarea_asincrona();
    let tarea2 = tarea_asincrona();

    // Ejecutar las tareas asincrónicas concurrentemente
    tokio::join!(tarea1, tarea2);

    println!("Ambas tareas han terminado.");
}
