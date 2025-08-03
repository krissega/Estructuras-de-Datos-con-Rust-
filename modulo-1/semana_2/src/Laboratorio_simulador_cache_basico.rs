/*!
 * Laboratorio: Simulador de Cache (Vec<T>)
 *
 * Descripción:
 * Este laboratorio simula el acceso a una memoria cache simple utilizando un vector.
 * Cada vez que el usuario ingresa un valor, se almacena en la cache hasta alcanzar
 * su capacidad máxima. Si se supera, el elemento más antiguo es removido (FIFO).
 *
 * Objetivo:
 * - Comprender el comportamiento de un buffer con capacidad limitada
 * - Utilizar operaciones como push y remove con Vec<T>
 * - Aplicar lógica condicional para simular reemplazo de memoria
 *
 * Instrucciones:
 * 1. Compila con: `rustc lab_cache_simulador.rs`
 * 2. Ejecuta con: `./lab_cache_simulador`
 * 3. Ingresa números (identificadores de acceso) y observa el estado de la cache
 */

use std::io;

fn main() {
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
                        cache.remove(0); // Remueve el más antiguo
                    }
                    cache.push(valor);
                }
                println!("Cache actual: {:?}", cache);
            }
            Err(_) => println!("Entrada inválida. Intenta con un número entero."),
        }
    }
}