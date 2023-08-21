use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("¡Adivina el numero!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);
    println!("El numero secreo es: {numero_secreto}");

    println!("Ingresa un numero:");
    let mut eleccion = String::new();

    io::stdin()
        .read_line(&mut eleccion)
        .expect("No se pudo leer la linea");

    let eleccion: u32 = eleccion.trim().parse().expect("Por favor ingresar un numero");
    println!("Tu eleccion: {eleccion}");

    match eleccion.cmp(&numero_secreto) {
        Ordering::Less => println!("¡Muy chico!"),
        Ordering::Greater => println!("¡Muy grande!"),
        Ordering::Equal => println!("¡Has ganado!")
    }
}
