
// Necesitamos importar VecDeque, que es la estructura de datos ideal para implementar colas en Rust.
// VecDeque significa "Double-Ended Queue" (Cola de Doble Extremo) y nos permite
// añadir y quitar elementos eficientemente de ambos extremos, lo cual es perfecto para una cola FIFO.
use std::collections::VecDeque;

fn main() {
    println!("--- Demostración del Simulador de Fila Simple (Cola) ---");
    println!("¡Imaginemos la fila para comprar empanadas en la soda del colegio!");

    // 1. Crear una nueva cola vacía
    // Declaramos 'fila_clientes' como mutable (mut) porque vamos a añadir y quitar elementos.
    // Usamos VecDeque<String> para que nuestra cola guarde nombres de clientes (texto).
    let mut fila_clientes: VecDeque<String> = VecDeque::new();
    println!("\nEstado inicial de la fila: {:?}", fila_clientes);
    println!("¿La fila está vacía al principio? {}", fila_clientes.is_empty());

    // 2. Clientes llegan a la fila (Operación: enqueue / push_back)
    println!("\n--- Llegan clientes a la fila ---");
    println!("Llega 'Pedro' y se pone al final.");
    fila_clientes.push_back(String::from("Pedro"));
    println!("Fila actual: {:?}", fila_clientes);

    println!("Llega 'María' y se pone al final.");
    fila_clientes.push_back(String::from("María"));
    println!("Fila actual: {:?}", fila_clientes);

    println!("Llega 'Juan' y se pone al final.");
    fila_clientes.push_back(String::from("Juan"));
    println!("Fila actual: {:?}", fila_clientes);

    // 3. Ver quién es el siguiente sin atenderlo (Operación: front / peek)
    println!("\n--- ¿Quién es el siguiente a ser atendido? ---");
    // Usamos 'front()' para ver el elemento al frente. Retorna un 'Option<&String>'.
    // 'if let Some(cliente)' nos permite manejar si hay un cliente o si la cola está vacía.
    if let Some(siguiente_cliente) = fila_clientes.front() {
        println!("¡El siguiente cliente a ser atendido es: {}!", siguiente_cliente);
    } else {
        println!("La fila está vacía, no hay nadie para atender.");
    }
    // La cola no debe haber cambiado después de 'front()'.
    println!("Fila después de solo mirar al frente: {:?}", fila_clientes);


    // 4. Atender clientes (Operación: dequeue / pop_front)
    println!("\n--- La panadera empieza a atender clientes ---");
    // Usamos 'pop_front()' para quitar y devolver el elemento del frente de la cola.
    // También retorna un 'Option<String>'.
    if let Some(cliente_atendido) = fila_clientes.pop_front() {
        println!("Se atendió a: {}. ¡Disfruta tus empanadas!", cliente_atendido);
    } else {
        println!("¡Ups! No había clientes en la fila para atender.");
    }
    println!("Fila actual: {:?}", fila_clientes);

    // Atendemos a otro cliente
    if let Some(cliente_atendido) = fila_clientes.pop_front() {
        println!("Se atendió a: {}. ¡Listo!", cliente_atendido);
    } else {
        println!("No quedan más clientes para atender.");
    }
    println!("Fila actual: {:?}", fila_clientes);

    // 5. Vaciar completamente la cola
    println!("\n--- Atendiendo a todos los clientes restantes ---");
    // Un bucle 'while let Some(...) = ...' es perfecto para procesar todos los elementos
    // de una cola hasta que esté vacía.
    while let Some(cliente_atendido_final) = fila_clientes.pop_front() {
        println!("Se atendió al último cliente: {}.", cliente_atendido_final);
        println!("Fila restante: {:?}", fila_clientes);
    }

    // 6. Verificar si la cola está vacía al final
    println!("\n--- Fin del día en la soda ---");
    println!("¿La fila está completamente vacía ahora? {}", fila_clientes.is_empty());
    println!("¡Todos los clientes han sido atendidos!");
}