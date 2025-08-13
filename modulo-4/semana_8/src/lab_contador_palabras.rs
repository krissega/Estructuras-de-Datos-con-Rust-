use std::collections::HashMap;
use std::io;

fn main() {
    println!("==============================");
    println!(" Laboratorio: Contador de palabras con HashMap");
    println!("==============================");
    println!("Instrucciones:");
    println!("  Escribe o pega el texto a analizar y presiona Enter para procesar.");
    println!("------------------------------");

    // Paso 1: Leer texto de entrada
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error al leer línea");
    if buffer.trim().is_empty() {
        println!("⚠ No se ingresó texto. Finalizando.");
        return;
    }

    // Paso 2: Normalizar el texto
    // TODO: Implementar función para convertir a minúsculas y reemplazar caracteres no alfanuméricos por espacio
    let limpio = buffer; // <- reemplazar por llamada a tu función

    // Paso 3: Contar frecuencias con HashMap
    let mut frec: HashMap<String, usize> = HashMap::new();
    // TODO: Recorrer las palabras y contar ocurrencias

    // Paso 4: Convertir el HashMap en un vector de pares (palabra, frecuencia)
    // TODO: Usar into_iter().collect() para crear un Vec<(String, usize)>

    // Paso 5: Ordenar el vector por frecuencia descendente y luego alfabéticamente
    // TODO: Usar sort_by con b.1.cmp(&a.1).then_with(...)

    // Paso 6: Imprimir resultados
    // TODO: Mostrar "=== Frecuencias ===" y luego cada par frecuencia-palabra
}
