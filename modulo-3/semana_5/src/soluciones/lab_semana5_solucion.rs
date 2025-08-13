// Filename: lab_semana5_solucion.rs
//
// Solución completa para el Laboratorio 1 de Listas Enlazadas.
// Este archivo contiene la implementación correcta de la Lista Enlazada básica,
// con los métodos `new()`, `push_front()` y `print_list()` resueltos.

use std::fmt::Display;

// --- ESTRUCTURAS ---

// Tarea 1: Estructura `Node`
// Un nodo contiene los datos y un puntero al siguiente nodo.
// Se utiliza `Option<Box<Node<T>>>` para manejar la recursividad y la posibilidad de un nodo nulo.
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Tarea 2: Estructura `LinkedList`
// La lista enlazada es simplemente un puntero a la "cabeza" de la lista.
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

// --- IMPLEMENTACIÓN DE LA LISTA ENLAZADA ---

impl<T: Display> LinkedList<T> {
    // Tarea 3: Implementación del constructor `new()`
    // Crea una lista vacía inicializando la `head` a `None`.
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Tarea 4: Implementación de la inserción al inicio `push_front()`
    // Esta es una operación de tiempo constante O(1).
    fn push_front(&mut self, data: T) {
        // 1. Tomamos la `head` actual de la lista.
        //    `self.head.take()` mueve el valor de `self.head` y lo reemplaza con `None`.
        let old_head = self.head.take();

        // 2. Creamos un nuevo nodo.
        //    Su campo `next` ahora apunta a lo que antes era la `head`.
        let new_node = Box::new(Node {
            data,
            next: old_head,
        });

        // 3. El nuevo nodo se convierte en la nueva `head` de la lista.
        self.head = Some(new_node);
    }

    // Tarea 5: Implementación del recorrido `print_list()`
    // Recorre cada nodo de la lista, imprimiendo su valor.
    fn print_list(&self) {
        println!("--- Recorrido de la lista ---");

        // 1. `current` es un puntero temporal que comienza en la `head`.
        let mut current = &self.head;

        // 2. Usamos un bucle `while let` para iterar mientras `current` no sea `None`.
        //    `while let Some(node) = current` toma prestado el valor de `current`.
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next; // Movemos el puntero `current` al siguiente nodo.
        }
        println!("None");
    }
}

// --- FUNCIÓN PRINCIPAL DE PRUEBA ---

fn main() {
    println!("--- Solución del Laboratorio de Listas Enlazadas ---");

    // Crea una nueva lista vacía.
    let mut lista: LinkedList<i32> = LinkedList::new();
    println!("Lista inicial:");
    lista.print_list();

    // Inserta elementos al inicio.
    lista.push_front(10);
    lista.push_front(20);
    lista.push_front(30);

    println!("\nLista después de insertar 30, 20, 10:");
    lista.print_list();

    // Añade más pruebas.
    lista.push_front(5);
    println!("\nLista después de insertar 5:");
    lista.print_list();
}