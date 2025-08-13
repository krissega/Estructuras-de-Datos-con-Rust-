// Filename: lab_listas_avanzadas.rs
//
// Laboratorio 6.1 de Listas Enlazadas - EJERCICIO GUIADO.
// Objetivo: Demostrar la implementación de las operaciones de inserción y eliminación
//           al final de una lista enlazada simple. El código está completo y
//           documentado para que puedas analizarlo y entender su funcionamiento.
//
// En este laboratorio, nos enfocamos en dos operaciones clave:
// 1. `push_back()`: Agregar un nodo al final de la lista.
// 2. `pop_back()`: Eliminar y devolver el último nodo de la lista.
//
// Ambas operaciones tienen una complejidad de tiempo de O(n) porque requieren
// recorrer la lista desde el principio para llegar al final o al penúltimo nodo.

use std::fmt::Display;

// --- ESTRUCTURAS BÁSICAS DE LA LISTA ENLAZADA (REUTILIZADAS) ---

// Un nodo de la lista enlazada. Contiene un dato y una referencia al siguiente nodo.
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// La estructura de la lista enlazada. Contiene el puntero a la "cabeza" o inicio de la lista.
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display> LinkedList<T> {
    // Constructor para crear una nueva lista enlazada vacía.
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Inserta un nodo al inicio de la lista. Esta es una operación O(1).
    fn push_front(&mut self, data: T) {
        let old_head = self.head.take();
        let new_node = Box::new(Node {
            data,
            next: old_head,
        });
        self.head = Some(new_node);
    }

    // Elimina y devuelve el nodo del inicio de la lista. Esta es una operación O(1).
    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // Comprueba si la lista está vacía.
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Función de ayuda para imprimir los elementos de la lista.
    fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }

    // --- Tarea 1: Implementación del método `push_back()` ---
    // Agrega un nuevo nodo al final de la lista.
    //
    // Lógica:
    // 1. Si la lista está vacía, el nuevo nodo se convierte en la head.
    // 2. Si no está vacía, recorremos la lista hasta encontrar el último nodo.
    // 3. Una vez en el último nodo, actualizamos su puntero `next` para que apunte al nuevo nodo.
    fn push_back(&mut self, data: T) {
        // Creamos el nuevo nodo que queremos agregar. Su puntero `next` es `None`
        // porque será el nuevo final de la lista.
        let new_node = Box::new(Node { data, next: None });

        // `current_ptr` es una referencia mutable al `Option<Box<Node<T>>>`
        // que vamos recorriendo. Inicialmente apunta a la cabeza de la lista.
        let mut current_ptr = &mut self.head;

        // El bucle avanza el puntero `current_ptr` a través de los punteros `next`
        // hasta que `current_ptr` sea la referencia al `None` final.
        // La sintaxis se ha simplificado para cumplir con las reglas de vinculación de Rust.
        while let Some(node) = current_ptr {
            current_ptr = &mut node.next;
        }

        // Ahora, `current_ptr` es una referencia mutable a la `Option` vacía al final.
        // Asignamos el nuevo nodo directamente a esa `Option`.
        *current_ptr = Some(new_node);
    }

    // --- Tarea 2: Implementación del método `pop_back()` ---
    // Elimina y devuelve el último nodo de la lista.
    //
    // Lógica:
    // 1. Manejar el caso de una lista vacía o de un solo elemento.
    // 2. Si hay más de un elemento, recorrer la lista hasta encontrar el penúltimo nodo.
    //    El penúltimo nodo es aquel cuyo puntero `next` no es `None`, pero el puntero
    //    `next` de su siguiente nodo sí lo es.
    // 3. Tomar el último nodo, establecer el `next` del penúltimo a `None` y devolver
    //    el dato del nodo eliminado.
    fn pop_back(&mut self) -> Option<T> {
        // Caso 1: La lista está vacía. No hay nada que eliminar.
        if self.is_empty() {
            return None;
        }

        // Caso 2: La lista tiene un solo elemento.
        // Si `head.next` es `None`, significa que solo hay un nodo.
        // Tomamos el nodo y lo reemplazamos con `None`, devolviendo su dato.
        // La llamada a `unwrap()` es segura aquí porque ya comprobamos que no está vacío.
        if self.head.as_ref().unwrap().next.is_none() {
            return self.head.take().map(|node| node.data);
        }

        // Caso 3: La lista tiene varios elementos.
        // Necesitamos encontrar el penúltimo nodo.
        let mut current = self.head.as_mut().unwrap();

        // Recorremos la lista hasta que el nodo actual tenga un `next` cuyo `next`
        // sea `None`. En este punto, `current` es el penúltimo nodo.
        while current.next.as_ref().unwrap().next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        // Ahora que `current` es el penúltimo nodo, usamos `take()` para "tomar"
        // la propiedad del último nodo. Esto establece el `next` de `current` a `None`.
        let last_node = current.next.take();

        // Devolvemos el dato del nodo que acabamos de eliminar.
        last_node.map(|node| node.data)
    }
}

fn main() {
    println!("--- Laboratorio 6.1: Operaciones Avanzadas en Listas Enlazadas ---");

    let mut lista: LinkedList<i32> = LinkedList::new();
    lista.push_front(1);
    lista.push_front(2);
    lista.push_front(3);
    println!("\nLista inicial (con push_front):");
    lista.print_list(); // Expected: 3 -> 2 -> 1 -> None

    println!("\n--- Probando push_back(4) ---");
    lista.push_back(4);
    lista.print_list(); // Expected: 3 -> 2 -> 1 -> 4 -> None

    println!("\n--- Probando pop_back() ---");
    let ultimo = lista.pop_back();
    println!("Elemento eliminado: {:?}", ultimo); // Expected: Some(4)
    lista.print_list(); // Expected: 3 -> 2 -> 1 -> None

    println!("\n--- Probando pop_back() varias veces ---");
    lista.pop_back();
    lista.pop_back();
    lista.print_list(); // Expected: 3 -> None

    println!("\n--- Probando pop_back() en una lista de un solo elemento ---");
    lista.pop_back();
    lista.print_list(); // Expected: None

    println!("\n--- Probando pop_back() en una lista vacía ---");
    let ultimo_vacio = lista.pop_back();
    println!("Elemento eliminado: {:?}", ultimo_vacio); // Expected: None
    lista.print_list(); // Expected: None
}