fn main() {
    // i8: Para temperaturas pequeñas (-128 a 127)
    let temperatura: i8 = -10; // Temperatura en grados Celsius
    println!("Temperatura actual: {}°C", temperatura);

    // Verificamos si la temperatura está dentro de un rango seguro
    if temperatura < -50 || temperatura > 50 {
        println!("¡Alerta! Temperatura fuera de rango seguro.");
    } else {
        println!("Temperatura dentro de rango seguro.");
    }

    // i16: Para altitudes (-32,768 a 32,767)
    let altitud: i16 = 5_200; // Altitud en metros (por ejemplo, una montaña)
    println!("Altitud actual: {} metros", altitud);

    // Verificamos si estamos por encima del nivel del mar
    if altitud > 0 {
        println!("Estamos por encima del nivel del mar.");
    } else if altitud < 0 {
        println!("Estamos por debajo del nivel del mar.");
    } else {
        println!("Estamos al nivel del mar.");
    }

    // i32: Para conteo de eventos
    let lecturas_sensor: i32 = 1_234_567; // Número de lecturas del sensor
    println!("Número de lecturas del sensor: {}", lecturas_sensor);

    // i64: Para marcas de tiempo (timestamps)
    let timestamp: i64 = 1_635_273_849_123; // Marca de tiempo en milisegundos
    println!("Marca de tiempo (milisegundos): {}", timestamp);

    // i128: Para acumuladores muy grandes (por ejemplo, cálculos financieros)
    let acumulador_datos: i128 = 123_456_789_123_456_789_123_456_789;
    println!("Acumulador de datos (i128): {}", acumulador_datos);

    // isize: Para índices de arreglos (depende de la arquitectura)
    let datos: [i32; 5] = [10, 20, 30, 40, 50];
    let indice: isize = 3; // Índice para acceder al arreglo
    if indice >= 0 && (indice as usize) < datos.len() {
        println!("Valor en el índice {}: {}", indice, datos[indice as usize]);
    } else {
        println!("Índice {} fuera de rango.", indice);
    }

    // Ejemplo de desbordamiento con i8 (para aprendizaje)
    let mut pequeño_numero: i8 = 127; // Máximo valor de i8
    println!("Valor inicial de i8: {}", pequeño_numero);
    pequeño_numero = pequeño_numero.wrapping_add(1); // Desborda a -128
    println!("Después de sumar 1 (con wrapping): {}", pequeño_numero);
}