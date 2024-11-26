use std::io;

fn main() {
    let mut entrada1 = String::new();
    let mut entrada2 = String::new();

    println!("Introduce el primer numero: ");
    io::stdin().read_line(&mut entrada1).expect("Error al leer");
    println!("Introduce el segundo numero: ");
    io::stdin().read_line(&mut entrada1).expect("Error al leer");

    let num1: i32 = entrada1.trim().parse().expect("Por favor ingresa un numero valido");
    let num2: i32 = entrada2.trim().parse().expect("Por favor ingresa un numero valido");

    println!("Suma: {}", num1 + num2);
    println!("Resta: {}", num1 - num2);
    println!("Multiplicacion: {}", num1 * num2);
    println!("Division: {}", num1 / num2);
    println!("Modulo: {}", num1 % num2);
}
