use std::fmt::Display;

// --- ESTRUCTURAS BÁSICAS DE LA LISTA ENLAZADA ---

/// Representa un nodo en la lista enlazada.
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// Implementa una lista enlazada simple, actuando como una pila (LIFO).
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Crea una nueva lista enlazada vacía.
    fn new() -> Self {
        LinkedList { head: None }
    }

    /// Verifica si la lista está vacía.
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Añade un elemento al frente de la lista.
    fn push(&mut self, data: T) {
        let old_head = self.head.take();
        let new_node = Box::new(Node {
            data,
            next: old_head,
        });
        self.head = Some(new_node);
    }

    /// Elimina y retorna el elemento del frente de la lista.
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    /// Retorna una referencia al elemento del frente sin tomar el ownership.
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Vacía la lista por completo.
    fn clear(&mut self) {
        self.head = None;
    }
}

// --- ESTRUCTURA DEL MÓDULO DE ACCIONES ---

/// Gestiona el historial de acciones de facturación con funcionalidad de deshacer y rehacer.
struct BillingActionHistory<T> {
    current_action: Option<T>,
    undo_stack: LinkedList<T>,
    redo_stack: LinkedList<T>,
}

impl<T: Display + Copy> BillingActionHistory<T> {
    /// Crea un nuevo historial de acciones vacío.
    fn new() -> Self {
        BillingActionHistory {
            current_action: None,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
        }
    }

    /// Realiza una nueva acción, moviendo la acción actual al historial de deshacer.
    ///
    /// # Argumentos
    /// * `action`: El dato de la nueva acción a realizar.
    fn perform_action(&mut self, action: T) {
        if let Some(old_action) = self.current_action.take() {
            self.undo_stack.push(old_action);
        }
        self.current_action = Some(action);
        self.redo_stack.clear();
    }

    /// Deshace la última acción realizada, moviendo la acción actual al historial de rehacer.
    fn undo(&mut self) {
        if let Some(action_to_undo) = self.undo_stack.pop() {
            if let Some(old_action) = self.current_action.take() {
                self.redo_stack.push(old_action);
            }
            self.current_action = Some(action_to_undo);
        }
    }

    /// Rehace una acción previamente deshecha.
    fn redo(&mut self) {
        if let Some(action_to_redo) = self.redo_stack.pop() {
            if let Some(old_action) = self.current_action.take() {
                self.undo_stack.push(old_action);
            }
            self.current_action = Some(action_to_redo);
        }
    }

    /// Muestra el estado actual del historial de acciones.
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