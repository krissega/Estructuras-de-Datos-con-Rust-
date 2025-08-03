/// # Memory Game en Rust 🧠🎮
/// Este juego simple consiste en adivinar una lista de palabras secretas.
/// Los jugadores escriben palabras una por una y reciben retroalimentación inmediata.
/// El objetivo es adivinar todas las palabras antes de agotar el número máximo de intentos.

use std::io;

fn main() {
    // Lista de palabras que el jugador debe adivinar.
    // Todas deben estar en minúsculas para comparación correcta.
    let mut palabras = vec!["manzana", "pera", "uva", "melón"];

    // Vector paralelo que indica cuáles palabras ya han sido adivinadas.
    // Comienza todo en `false`.
    let mut adivinadas: Vec<bool> = vec![false; palabras.len()];

    // Contador de intentos realizados por el jugador.
    let mut intentos = 0;

    /// Número máximo de intentos permitidos. Si se alcanza, el juego termina.
    let max_intentos = 10;

    println!("🍒 Bienvenido al Memory Game con Rust!");
    println!(
        "🔤 Adivina las palabras ocultas. Tienes un máximo de {} intentos.\n",
        max_intentos
    );

    // Bucle principal del juego: continúa mientras haya palabras por adivinar
    // y no se supere el número máximo de intentos.
    while adivinadas.iter().any(|&x| !x) && intentos < max_intentos {
        // Mostrar progreso del jugador con las palabras adivinadas hasta ahora.
        println!("📜 Palabras adivinadas:");
        for (i, &adivinada) in adivinadas.iter().enumerate() {
            if adivinada {
                print!("[{}] ", palabras[i]);
            } else {
                print!("[???] ");
            }
        }
        println!("\n");

        // Solicitar palabra al usuario por consola.
        println!("⌨️ Escribe una palabra:");
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer entrada");

        // Limpieza y normalización de entrada (eliminar espacios y poner en minúscula).
        let entrada = entrada.trim().to_lowercase();

        // Variable para saber si se acertó una palabra en este intento.
        let mut acierto = false;

        // Comparar entrada con palabras no adivinadas aún.
        for (i, palabra) in palabras.iter().enumerate() {
            if palabra == &entrada && !adivinadas[i] {
                adivinadas[i] = true;
                println!("✅ ¡Correcto!\n");
                acierto = true;
                break;
            }
        }

        // Si no se acertó ninguna palabra.
        if !acierto {
            println!("❌ Incorrecto o palabra ya adivinada.\n");
        }

        // Aumentar el número de intentos.
        intentos += 1;
        println!("📊 Intento {}/{}\n", intentos, max_intentos);
    }

    // Verificar si se ganaron todas las palabras o se alcanzó el límite.
    if adivinadas.iter().all(|&x| x) {
        println!(
            "🎉 ¡Felicidades! Has adivinado todas las palabras en {} intentos.",
            intentos
        );
    } else {
        println!("🚫 Has alcanzado el máximo de intentos. Inténtalo de nuevo.");
    }
}