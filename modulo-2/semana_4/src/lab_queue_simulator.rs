// lab_queue_simulator.rs

// ¡No olvides importar VecDeque! La necesitarás para crear tu cola.
use std::collections::VecDeque;

fn main() {
    println!("--- Simulador de Fila de la Soda del Colegio ---");

    // PARTE 1: La Fila Vacía - ¡Creando nuestra Cola!
    // Tarea 1.1: Declara aquí tu VecDeque para 'fila_clientes'.
    // Recuerda que debe ser mutable y guardar Strings.
    // TU CÓDIGO AQUÍ:


    println!("\nLa fila al inicio está vacía: {:?}", fila_clientes);


    // PARTE 2: ¡Llegan los Clientes! - Encolando Elementos
    println!("\n--- Llegada de Clientes (Encolando / push_back) ---");
    // Tarea 2.1: Añade a "Pedro", "María" y "Juan" a 'fila_clientes', en ese orden.
    // Usa `push_back()` y `String::from()`.
    // TU CÓDIGO AQUÍ:


    println!("Después de que llegan los clientes: {:?}", fila_clientes);


    // PARTE 3: ¿Quién es el Siguiente? - Viendo el Frente de la Cola
    println!("\n--- Quien está de primero (front) ---");
    // Tarea 3.1: Mira quién está al frente de la cola SIN removerlo.
    // Usa `front()` y maneja el `Option` con `if let Some(...)`.
    // TU CÓDIGO AQUÍ:


    println!("La fila después de solo mirar al frente (no cambia): {:?}", fila_clientes);


    // PARTE 4: ¡Atendiendo Clientes! - Desencolando Elementos
    println!("\n--- Atendiendo Clientes (Desencolando / pop_front) ---");
    // Tarea 4.1: Atiende al primer cliente (sácalo de la fila).
    // Usa `pop_front()` y maneja el `Option`.
    // TU CÓDIGO AQUÍ:


    println!("La fila después de atender al primer cliente: {:?}", fila_clientes);

    // Tarea Opcional 4.2: Atiende a otro cliente.
    // TU CÓDIGO AQUÍ:


    println!("La fila después de atender a otro cliente: {:?}", fila_clientes);


    // PARTE 5: ¡La Fila Termina! - Vaciando la Cola
    println!("\n--- Vaciando la Fila por Completo ---");
    // Tarea 5.1: Usa un bucle `while let Some(...)` para atender a todos los clientes
    // que queden en la fila hasta que esté vacía.
    // TU CÓDIGO AQUÍ:


    println!("\n¿La fila está completamente vacía ahora? {}", fila_clientes.is_empty());
    println!("--- Fin del Simulador ---");
}

/*
    Las "Preguntas para Reflexionar" para este laboratorio se encuentran en el archivo
    'README.md' de esta misma carpeta. ¡Asegúrate de leerlas y pensarlas!
*/