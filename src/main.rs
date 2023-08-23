fn main() {
    const VALOR_CONSTANTE: u32 = 60 * 60 *3;
    println!("{}", VALOR_CONSTANTE);
    println!();


    let mut x = 5;
    println!("El valor de x es: {x}");
    x = 6;
    println!("El valor de x es: {x} <---");
    println!();


    {
        let x = x * 2;
        println!("[DENTRO] El valor de x es: {x}");
        
    }
    println!("[FUERA ] El valor de x es: {x} <---");
    println!();
}
