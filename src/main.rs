use std::io;

fn main() {
    println!("¡Adivina el numero!");

    println!("Ingresa un numero:");
    let mut eleccion = String::new();

    io::stdin()
        .read_line(&mut eleccion)
        .expect("No se pudo leer la linea");

    println!("Haz adivinado: {eleccion}")
}
