fn main() {
    let x = 10;

    // Estructura condicional (if-else)
    if x > 5 {
        println!("x es mayor que 5");
    } else {
        println!("x es menor o igual a 5");
    }

    // Bucle 'for'
    for i in 1..5 {  // Rango de 1 a 4
        println!("i: {}", i);
    }

    // Bucle 'while'
    let mut count = 0;
    while count < 3 {
        println!("count: {}", count);
        count += 1;
    }

    // Bucle 'loop'
    let mut counter = 0;
    loop {
        println!("contador: {}", counter);
        counter += 1;
        if counter == 3 {
            break; // Rompe el bucle cuando el contador llega a 3
        }
    }
}
