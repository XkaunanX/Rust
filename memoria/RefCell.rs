use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);

    // Prestando mutablemente a través de RefCell
    *x.borrow_mut() = 10;
    println!("Valor de x: {}", x.borrow());
}
