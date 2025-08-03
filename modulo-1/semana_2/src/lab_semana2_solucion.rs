
// ==============================================
// Laboratorio Semana 2: Soluciones Completas
// ==============================================

// ----------------------------------------------
// Solución 1: Calculadora de Promedio
// ----------------------------------------------

/*!
 * Laboratorio: Calculadora de Promedio
 *
 * Este programa permite ingresar múltiples números flotantes (f64)
 * y calcula su promedio.
 */

use std::io;

fn main_calculadora_promedio() {
    println!("Calculadora de Promedio");
    println!("Introduce números (uno por línea). Escribe 'fin' para terminar.");

    let mut numeros: Vec<f64> = Vec::new();

    loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer");
        let entrada = entrada.trim();

        if entrada.eq_ignore_ascii_case("fin") {
            break;
        }

        match entrada.parse::<f64>() {
            Ok(num) => numeros.push(num),
            Err(_) => println!("Entrada inválida. Intenta con un número o 'fin'."),
        }
    }

    if numeros.is_empty() {
        println!("No se ingresaron números.");
    } else {
        let suma: f64 = numeros.iter().sum();
        let promedio = suma / numeros.len() as f64;
        println!("Promedio: {:.2}", promedio);
    }
}

// ----------------------------------------------
// Solución 2: Simulador de Cache
// ----------------------------------------------

/*!
 * Laboratorio: Simulador de Cache FIFO
 *
 * Simula una cache de tamaño fijo (3 elementos) que reemplaza el
 * elemento más antiguo cuando se excede la capacidad.
 */

fn main_simulador_cache() {
    const TAMANIO_CACHE: usize = 3;
    let mut cache: Vec<i32> = Vec::new();

    println!("Simulador de Cache FIFO (capacidad: {})", TAMANIO_CACHE);
    println!("Ingresa números enteros (Ctrl+C para salir)");

    loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer");

        let entrada = entrada.trim();

        match entrada.parse::<i32>() {
            Ok(valor) => {
                if !cache.contains(&valor) {
                    if cache.len() == TAMANIO_CACHE {
                        cache.remove(0);
                    }
                    cache.push(valor);
                }
                println!("Cache actual: {:?}", cache);
            }
            Err(_) => println!("Entrada inválida. Intenta con un número entero."),
        }
    }
}
