// lab_queue_simulator.rs

use std::collections::VecDeque; // ¡Necesitamos VecDeque para nuestra cola!

fn main() {
    println!("--- Simulador de Fila de la Soda del Colegio ---");

    // PARTE 1: La Fila Vacía - ¡Creando nuestra Cola!
    // Tarea 1.1: Crea una nueva VecDeque<String> llamada 'fila_clientes' para simular la fila.
    // Asegúrate de que sea 'mut' porque vamos a añadir y quitar elementos.
    // TU CÓDIGO AQUÍ:
    let mut fila_clientes: VecDeque<String> = VecDeque::new();


    println!("\nLa fila al inicio está vacía: {:?}", fila_clientes);


    // PARTE 2: ¡Llegan los Clientes! - Encolando Elementos
    println!("\n--- Llegada de Clientes (Encolando/push_back) ---");
    // Tarea 2.1: Añade a los clientes a la fila en el siguiente orden:
    // "Pedro"
    // "María"
    // "Juan"
    // Usa la operación `push_back()` para cada cliente.
    // TU CÓDIGO AQUÍ:
    fila_clientes.push_back(String::from("Pedro"));
    fila_clientes.push_back(String::from("María"));
    fila_clientes.push_back(String::from("Juan"));


    println!("Después de que llegan los clientes: {:?}", fila_clientes);


    // PARTE 3: ¿Quién es el Siguiente? - Viendo el Frente de la Cola
    println!("\n--- Quien está de primero (front) ---");
    // Tarea 3.1: Mira quién está al frente de la fila SIN removerlo.
    // Usa `fila_clientes.front()` y maneja el `Option` con un `if let Some(...)`.
    // Si la cola está vacía, imprime un mensaje adecuado.
    // TU CÓDIGO AQUÍ:
    if let Some(siguiente_cliente) = fila_clientes.front() {
        println!("El siguiente cliente a ser atendido es: {}", siguiente_cliente);
    } else {
        println!("No hay nadie en la fila para ver.");
    }


    println!("La fila después de solo mirar al frente (no cambia): {:?}", fila_clientes);


    // PARTE 4: ¡Atendiendo Clientes! - Desencolando Elementos
    println!("\n--- Atendiendo Clientes (Desencolando/pop_front) ---");
    // Tarea 4.1: Atiende al primer cliente (sácalo de la fila) e imprime su nombre.
    // Usa `fila_clientes.pop_front()` y maneja el `Option`.
    // TU CÓDIGO AQUÍ:
    if let Some(cliente_atendido_1) = fila_clientes.pop_front() {
        println!("Se atendió a: {}", cliente_atendido_1);
    } else {
        println!("No había clientes para atender.");
    }

    println!("La fila después de atender al primer cliente: {:?}", fila_clientes);

    // Tarea Opcional 4.2: Atiende a otro cliente y vuelve a imprimir la fila.
    // TU CÓDIGO AQUÍ (si quieres):
    if let Some(cliente_atendido_2) = fila_clientes.pop_front() {
        println!("Se atendió a: {}", cliente_atendido_2);
    } else {
        println!("No había más clientes para atender.");
    }

    println!("La fila después de atender a otro cliente: {:?}", fila_clientes);


    // PARTE 5: ¡La Fila Termina! - Vaciando la Cola
    println!("\n--- Vaciando la Fila por Completo ---");
    // Tarea 5.1: Usa un bucle `while let Some(...)` para atender a todos los clientes
    // que queden en la fila hasta que esté completamente vacía.
    // Imprime a quién se atiende cada vez.
    // TU CÓDIGO AQUÍ:
    while let Some(cliente_atendido_final) = fila_clientes.pop_front() {
        println!("Se atendió al último cliente: {}", cliente_atendido_final);
    }


    println!("\n¿La fila está completamente vacía ahora? {}", fila_clientes.is_empty());
    println!("--- Fin del Simulador ---");
}

/*
    PREGUNTAS PARA REFLEXIONAR (NO HAY QUE ESCRIBIR CÓDIGO, SOLO PENSAR):
    Estas preguntas son para que pienses un poco más allá del código.
    1.  ¿Por qué es importante usar la palabra clave 'mut' al declarar 'fila_clientes'? ¿Qué pasaría si la quitáramos?
    2.  Si 'fila_clientes' estuviera vacía al inicio, ¿qué pasaría si intentáramos llamar a 'pop_front()' o 'front()'?
        ¿Cómo nos ayuda Rust con el tipo `Option` a manejar esa situación sin que el programa "explote"?
    3.  Imagina que quieres simular una fila donde los clientes VIP (que pagaron más) son atendidos primero, sin importar cuándo llegaron.
        ¿Sería una cola (Queue) normal la mejor estructura de datos para esto? ¿Por qué sí o por qué no?
*/