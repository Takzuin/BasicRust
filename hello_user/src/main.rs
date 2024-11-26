use std::io;

fn main() {
    let mut nombre = String::new(); // Variable mutable
    println!("¿Cuál es tu nombre?:");
    io::stdin() // Entrada estándar
        .read_line(&mut nombre) // Lee la entrada
        .expect("Error al leer la entrada"); // Manejo de errores
    println!("Hola {}", nombre.trim()); // Imprime el nombre limpio
}