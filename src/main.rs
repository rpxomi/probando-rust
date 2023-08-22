use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!();
    println!("¡Adivina el numero!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);
    //println!("El numero secreto es: {numero_secreto}");

    loop {
        println!();
        println!("Ingresa un numero:");
        let mut eleccion = String::new();

        io::stdin()
            .read_line(&mut eleccion)
            .expect("No se pudo leer la linea");

        let eleccion: u32 = match eleccion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!();
        println!("Tu eleccion: {eleccion}");

        match eleccion.cmp(&numero_secreto) {
            Ordering::Less => println!("Muy chico..."),
            Ordering::Greater => println!("Muy grande..."),
            Ordering::Equal => {
                println!("¡Ganaste!");
                break;
            }
        }
    }

    println!();
}
