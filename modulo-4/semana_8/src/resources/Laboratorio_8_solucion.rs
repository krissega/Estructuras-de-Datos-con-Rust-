use std::collections::HashMap;
use std::io;

fn normalizar(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_alphanumeric() {
            for lc in ch.to_lowercase() {
                out.push(lc);
            }
        } else {
            out.push(' ');
        }
    }
    out
}

fn main() {
    println!("==============================");
    println!(" Laboratorio: Contador de palabras con HashMap");
    println!("==============================");
    println!("Objetivo:");
    println!("  - Leer una línea de texto desde la entrada estándar.");
    println!("  - Normalizar (minúsculas, quitar puntuación).");
    println!("  - Contar frecuencia de cada palabra.");
    println!("  - Mostrar resultados ordenados por frecuencia.");
    println!();
    println!("Instrucciones:");
    println!("  Escribe o pega el texto a analizar y presiona Enter para procesar.");
    println!("------------------------------");

    // Paso 1: leer una sola línea
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error al leer línea");

    if buffer.trim().is_empty() {
        println!("⚠ No se ingresó texto. Finalizando.");
        return;
    }
    println!("✔ Texto recibido.");

    // Paso 2: normalización
    println!("Paso 2: Normalizando texto...");
    let limpio = normalizar(&buffer);
    println!("✔ Texto normalizado.");

    // Paso 3: conteo de frecuencias
    println!("Paso 3: Contando frecuencias...");
    let mut frec: HashMap<String, usize> = HashMap::new();
    for palabra in limpio.split_whitespace() {
        let entry = frec.entry(palabra.to_string()).or_insert(0);
        *entry += 1;
    }
    println!("✔ Conteo completado.");

    // Paso 4: ordenar resultados
    println!("Paso 4: Ordenando resultados...");
    let mut pares: Vec<(String, usize)> = frec.into_iter().collect();
    pares.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    println!("✔ Resultados ordenados.\n");

    // Paso 5: mostrar resultados
    println!("=== Frecuencias ===");
    for (pal, cnt) in pares {
        println!("{:>6}  {}", cnt, pal);
    }
}