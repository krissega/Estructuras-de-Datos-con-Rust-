

use std::fmt::Display;

// --- ESTRUCTURAS BÁSICAS DE LA LISTA ENLAZADA (REUTILIZADAS) ---

// Un nodo de la lista enlazada. Contiene una acción y una referencia al siguiente nodo.
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// La lista enlazada simple, que servirá como nuestra "pila" para el historial de acciones.
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Inserta un nodo al inicio de la lista (simula la operación `push` de una pila).
    fn push(&mut self, data: T) {
        let old_head = self.head.take();
        let new_node = Box::new(Node {
            data,
            next: old_head,
        });
        self.head = Some(new_node);
    }

    // Elimina y devuelve el nodo del inicio de la lista (simula la operación `pop` de una pila).
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // Devuelve una referencia al dato en la cabeza de la lista sin tomar la propiedad.
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // Vacía la lista por completo.
    fn clear(&mut self) {
        self.head = None;
    }
}

// --- ESTRUCTURA DEL MÓDULO DE ACCIONES ---

// Representa el estado del módulo de facturación, con las acciones actuales y los historiales
// para deshacer y rehacer.
struct BillingActionHistory<T> {
    current_action: Option<T>,
    undo_stack: LinkedList<T>,
    redo_stack: LinkedList<T>,
}

impl<T: Display + Copy> BillingActionHistory<T> {
    fn new() -> Self {
        BillingActionHistory {
            current_action: None,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
        }
    }

    // Tarea 1: Implementar la ejecución de una nueva acción.
    // ** ESTE CÓDIGO CONTIENE ERRORES DE OWNERSHIP. DEBE SER CORREGIDO. **
    fn perform_action(&mut self, action: T) {
        let current_action_ref = &self.current_action;
        let old_action = self.current_action.take();

        if let Some(old) = old_action {
            self.undo_stack.push(old);
        }
        self.current_action = Some(action);
        self.redo_stack.clear();
    }

    // Tarea 2: Implementar la función de 'deshacer' (undo).
    // ** ESTE CÓDIGO CONTIENE ERRORES DE OWNERSHIP. DEBE SER CORREGIDO. **
    fn undo(&mut self) {
        if let Some(action_to_undo) = self.undo_stack.pop() {
            let current_action = &mut self.current_action;
            if let Some(old) = current_action {
                self.redo_stack.push(*old);
            }
            self.current_action = Some(action_to_undo);
        }
    }

    // Tarea 3: Implementar la función de 'rehacer' (redo).
    // ** ESTE CÓDIGO CONTIENE ERRORES DE OWNERSHIP. DEBE SER CORREGIDO. **
    fn redo(&mut self) {
        if let Some(action_to_redo) = self.redo_stack.pop() {
            let current_action = &self.current_action;
            if let Some(old) = current_action {
                self.undo_stack.push(*old);
            }
            self.current_action = Some(action_to_redo);
        }
    }

    // Muestra el estado actual del sistema para fines de depuración.
    fn print_state(&self) {
        let current = self.current_action.as_ref().map_or("None".to_string(), |p| format!("{}", p));
        println!("--------------------------------------");
        println!("Acción actual: {}", current);
        println!("Historial 'Deshacer': ");
        let mut back_ptr = &self.undo_stack.head;
        while let Some(node) = back_ptr {
            print!("{} -> ", node.data);
            back_ptr = &node.next;
        }
        println!("None");
        println!("Historial 'Rehacer': ");
        let mut forward_ptr = &self.redo_stack.head;
        while let Some(node) = forward_ptr {
            print!("{} -> ", node.data);
            forward_ptr = &node.next;
        }
        println!("None");
        println!("--------------------------------------");
    }
}

fn main() {
    println!("--- Laboratorio 6.2: Módulo de Deshacer/Rehacer de 'Los Patitos' ---");
    let mut module = BillingActionHistory::new();

    println!("\n> Realizando acción: 'Factura A'");
    module.perform_action('A');
    module.print_state();

    println!("\n> Realizando acción: 'Factura B'");
    module.perform_action('B');
    module.print_state();

    println!("\n> Realizando acción: 'Factura C'");
    module.perform_action('C');
    module.print_state();

    println!("\n> Deshaciendo la última acción");
    module.undo();
    module.print_state();

    println!("\n> Deshaciendo de nuevo");
    module.undo();
    module.print_state();

    println!("\n> Rehaciendo una acción");
    module.redo();
    module.print_state();

    println!("\n> Rehaciendo de nuevo");
    module.redo();
    module.print_state();

    println!("\n> Deshaciendo y realizando una nueva acción (Factura D)");
    module.undo();
    module.perform_action('D');
    module.print_state();
}