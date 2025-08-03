# Soluciones Semana 2 - Curso de Estructuras de Datos en Rust

## Laboratorio 1: Simulador de Caché (lab_cache_simulador.rs)

```rust
use std::collections::VecDeque;

fn main() {
    let mut cache: VecDeque<i32> = VecDeque::new();
    let cache_size = 3;
    let access_sequence = vec![1, 2, 3, 2, 4, 1, 5];
    let mut hits = 0;
    let mut misses = 0;

    for access in access_sequence {
        if cache.contains(&access) {
            hits += 1;
            cache.retain(|&x| x != access);
            cache.push_front(access);
        } else {
            misses += 1;
            if cache.len() == cache_size {
                cache.pop_back();
            }
            cache.push_front(access);
        }
        println!("Cache: {:?}", cache);
    }

    println!("Cache hits: {}", hits);
    println!("Cache misses: {}", misses);
}
```

## Laboratorio 2: Calculadora de Promedio (lab_calculadora_promedio.rs)

```rust
fn main() {
    let calificaciones = vec![85.5, 90.0, 78.3, 92.4, 88.0];

    if calificaciones.is_empty() {
        println!("No hay calificaciones para calcular el promedio.");
        return;
    }

    let suma: f64 = calificaciones.iter().sum();
    let promedio = suma / calificaciones.len() as f64;

    println!("Calificaciones: {:?}", calificaciones);
    println!("Promedio: {:.2}", promedio);
}
```
