// Filename: lab_linkedlist_box.rs
//
// Laboratorio 1 de Listas Enlazadas.
// Objetivo: Implementar una Lista Enlazada simple en Rust usando Box<T> y Option<T>.
//
// Tareas a completar:
// 1. Definir la estructura `Node`.
// 2. Definir la estructura `LinkedList`.
// 3. Implementar el método `new()` para crear una lista vacía.
// 4. Implementar el método `push_front()` para insertar un nuevo nodo al inicio.
// 5. Implementar un método `print_list()` para recorrer e imprimir la lista.

// HINT: Necesitas usar `Box<T>` y `Option<T>` para manejar la recursividad y la posibilidad de un valor nulo.

use std::fmt::Display;

// --- ESTRUCTURAS ---

// Tarea 1: Define la estructura `Node`.
// Debe contener un `data` de tipo genérico `T` y un `next` que sea un puntero al siguiente nodo.
// Pista: Usa `Option<Box<Node>>`.
struct Node<T> {
    // TODO: Define el campo `data`.
    // TODO: Define el campo `next`.
}

// Tarea 2: Define la estructura `LinkedList`.
// Debe contener la `head`, que es el punto de entrada a la lista.
struct LinkedList<T> {
    // TODO: Define el campo `head`.
}

// --- IMPLEMENTACIÓN DE LA LISTA ENLAZADA ---

impl<T: Display> LinkedList<T> {
    // Tarea 3: Implementa el constructor para crear una lista vacía.
    fn new() -> Self {
        // TODO: Inicializa una lista enlazada con una `head` que apunta a `None`.
        unimplemented!()
    }

    // Tarea 4: Implementa la inserción al inicio (prepend).
    // Esta función toma la `head` actual, la convierte en el `next` del nuevo nodo y
    // luego actualiza la `head` de la lista para que apunte al nuevo nodo.
    fn push_front(&mut self, data: T) {
        // 1. Crea un nuevo nodo con `Box::new()`. Su `next` debe ser la `head` actual.
        // 2. Actualiza la `head` de la lista para que sea este nuevo nodo.
        unimplemented!()
    }

    // Tarea 5: Implementa la función para recorrer e imprimir la lista.
    // Esta función debe iterar a través de cada nodo, imprimiendo su `data`.
    fn print_list(&self) {
        println!("--- Recorrido de la lista ---");
        // 1. Inicializa un puntero temporal (`current`) en la `head`.
        // 2. Usa un bucle `while let` para avanzar por la lista.
        // 3. En cada iteración, imprime la `data` del nodo actual.
        // 4. Mueve el puntero `current` al siguiente nodo.
        unimplemented!()
    }
}

// --- FUNCIÓN PRINCIPAL DE PRUEBA ---

fn main() {
    println!("--- Laboratorio de Listas Enlazadas ---");

    // Crea una nueva lista vacía.
    let mut lista: LinkedList<i32> = LinkedList::new();
    println!("Lista inicial:");
    lista.print_list(); // Debería imprimir "Lista vacía."

    // Inserta algunos elementos al inicio.
    lista.push_front(10);
    lista.push_front(20);
    lista.push_front(30);

    println!("\nLista después de insertar 30, 20, 10:");
    lista.print_list(); // Debería imprimir 30 -> 20 -> 10 -> None

    // Puedes añadir más pruebas aquí si lo deseas.
    lista.push_front(5);
    println!("\nLista después de insertar 5:");
    lista.print_list(); // Debería imprimir 5 -> 30 -> 20 -> 10 -> None
}