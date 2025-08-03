/*!
 * Laboratorio: Calculadora de Promedio
 *
 * Descripción:
 * Este laboratorio permite al usuario ingresar un conjunto de números de tipo `f64`
 * y calcula su promedio. Es un ejemplo perfecto del uso de `Vec<f64>` y de la función `iter()`.
 *
 *  Objetivo:
 * - Reforzar el uso de vectores (`Vec<T>`)
 * - Practicar iteración y sumatoria de valores
 * - Entender cómo leer entradas desde la terminal
 *
 * ▶Ejecución:
 * Compila con: `rustc lab_calculadora_promedio.rs`
 * Ejecuta con: `./lab_calculadora_promedio`
 */

use std::io;

fn main() {
    println!("Sistemita para Calcular el promedio de varios valores ingresados ");
    println!("Introduce números (uno por línea). Escribe 'fin' para terminar.");

    let mut numeros: Vec<f64> = Vec::new();

    loop {
        // Pedimos entrada al usuario
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer");

        let entrada = entrada.trim();

        if entrada.eq_ignore_ascii_case("fin") {
            break;
        }

        // Convertimos la entrada a número
        match entrada.parse::<f64>() {
            Ok(num) => numeros.push(num),
            Err(_) => println!("⚠️ Entrada inválida. Intenta con un número o 'fin'."),
        }
    }

    if numeros.is_empty() {
        println!("No se ingresaron números.");
    } else {
        let suma: f64 = numeros.iter().sum();
        let promedio = suma / numeros.len() as f64;
        println!("📊 Promedio: {:.2}", promedio);
    }
}
