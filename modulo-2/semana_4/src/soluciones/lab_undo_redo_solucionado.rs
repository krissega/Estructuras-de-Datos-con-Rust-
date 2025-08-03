// lab_undo_redo.rs

// --- Reutilización de la estructura Pila<T> ---
pub struct Pila<T> {
    elementos: Vec<T>,
    // Tarea 3 (Opcional): Campo para capacidad máxima
    capacidad_maxima: Option<usize>,
}

impl<T> Pila<T> {
    pub fn new() -> Self {
        Pila {
            elementos: Vec::new(),
            capacidad_maxima: None, // Por defecto, sin límite de capacidad
        }
    }

    // Tarea 3 (Opcional): Constructor con capacidad máxima
    pub fn with_capacity(capacity: usize) -> Self {
        Pila {
            elementos: Vec::with_capacity(capacity),
            capacidad_maxima: Some(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        // Tarea 3 (Opcional): Lógica para capacidad máxima
        if let Some(max_cap) = self.capacidad_maxima {
            if self.elementos.len() >= max_cap {
                // Opción 1: No añadir y mostrar advertencia
                // println!("Advertencia: Pila llena, no se puede añadir más elementos.");
                // return;

                // Opción 2: Eliminar el elemento más antiguo (FIFO) para hacer espacio (más común para historial)
                self.elementos.remove(0); // Elimina el primer elemento
            }
        }
        self.elementos.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elementos.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elementos.len()
    }
}

// --- EDITOR DE TEXTO SIMPLIFICADO con funcionalidad Undo/Redo ---

struct EditorTexto {
    contenido: String,
    pila_undo: Pila<String>, // Guarda los estados anteriores
    pila_redo: Pila<String>, // Guarda los estados que pueden ser rehechos
}

impl EditorTexto {
    // Constructor del editor
    fn new(contenido_inicial: &str) -> Self {
        let mut editor = EditorTexto {
            contenido: contenido_inicial.to_string(),
            // Tarea 3 (Opcional): Usar pila con capacidad
            pila_undo: Pila::new(), // O Pila::with_capacity(5)
            pila_redo: Pila::new(), // O Pila::with_capacity(5)
        };
        // Guarda el estado inicial para poder deshacer desde el principio
        editor.pila_undo.push(contenido_inicial.to_string());
        editor
    }

    // Muestra el estado actual del editor y las pilas
    fn mostrar_contenido(&self) {
        println!("Contenido actual: \"{}\"", self.contenido);
        // La pila undo siempre tiene el estado actual + historial, por eso -1 para acciones previas
        println!("  (Acciones Undo disponibles: {}, Acciones Redo disponibles: {})",
                 self.pila_undo.size().saturating_sub(1), // saturating_sub para evitar underflow si solo hay 1
                 self.pila_redo.size());
    }

    // Método general para realizar una acción de edición
    fn realizar_accion(&mut self, nueva_contenido: String, accion_desc: &str) {
        // Antes de cualquier modificación, guarda el estado actual para "deshacer"
        self.pila_undo.push(self.contenido.clone());
        self.contenido = nueva_contenido;
        // Tarea 1: Cualquier nueva acción invalida (vacía) la pila de rehacer
        self.pila_redo = Pila::new();
        println!("\n--- {} ---", accion_desc);
        self.mostrar_contenido();
    }

    // Simula una acción de edición: añadir texto
    fn escribir(&mut self, texto: &str) {
        let mut nuevo_contenido = self.contenido.clone();
        nuevo_contenido.push_str(texto);
        self.realizar_accion(nuevo_contenido, &format!("Escribiendo: \"{}\"", texto));
    }

    // Simula una acción de edición: borrar n caracteres
    fn borrar(&mut self, n: usize) {
        if self.contenido.len() >= n {
            let mut nuevo_contenido = self.contenido.clone();
            nuevo_contenido.truncate(nuevo_contenido.len() - n);
            self.realizar_accion(nuevo_contenido, &format!("Borrando {} caracteres", n));
        } else {
            println!("\nNo se pueden borrar {} caracteres. Contenido muy corto.", n);
        }
    }

    // Tarea 2: Implementa la acción de reemplazar la última palabra
    fn reemplazar_ultima_palabra(&mut self, nueva_palabra: &str) {
        let palabras: Vec<&str> = self.contenido.split_whitespace().collect();
        if let Some(ultima_palabra_original) = palabras.last() {
            let mut nuevo_contenido = self.contenido.clone();
            let inicio_ultima_palabra = nuevo_contenido.rfind(ultima_palabra_original).unwrap_or(0);
            nuevo_contenido.replace_range(inicio_ultima_palabra.., nueva_palabra);
            self.realizar_accion(nuevo_contenido, &format!("Reemplazando \"{}\" por \"{}\"", ultima_palabra_original, nueva_palabra));
        } else {
            println!("\nNo hay palabras para reemplazar.");
        }
    }

    // Deshace la última acción
    fn undo(&mut self) {
        // La pila de undo debe tener al menos el estado inicial + 1 acción para poder deshacer
        if self.pila_undo.size() > 1 {
            let estado_actual_para_redo = self.pila_undo.pop().unwrap(); // Sacamos el estado actual
            self.pila_redo.push(estado_actual_para_redo); // Lo movemos a la pila de rehacer
            self.contenido = self.pila_undo.peek().unwrap().clone(); // Restauramos el estado anterior
            println!("\n--- Deshaciendo ---");
            self.mostrar_contenido();
        } else {
            println!("\nNo hay nada que deshacer.");
        }
    }

    // Rehace la última acción deshecha
    fn redo(&mut self) {
        if let Some(estado_rehecho) = self.pila_redo.pop() {
            // Guardamos el estado actual antes de rehacer para poder deshacer este "redo"
            self.pila_undo.push(self.contenido.clone());
            self.contenido = estado_rehecho;
            println!("\n--- Rehaciendo ---");
            self.mostrar_contenido();
        } else {
            println!("\nNo hay nada que rehacer.");
        }
    }
}

fn main() {
    println!("--- Simulador de Editor de Texto con Undo/Redo ---");

    let mut editor = EditorTexto::new("Hola");
    editor.mostrar_contenido();

    editor.escribir(" mundo"); // Hola mundo
    editor.escribir("!");      // Hola mundo!
    editor.escribir(" ¿Qué tal?"); // Hola mundo! ¿Qué tal?

    editor.undo(); // Deshace "¿Qué tal?" -> Hola mundo!
    editor.undo(); // Deshace "!"       -> Hola mundo
    editor.redo(); // Rehace "!"      -> Hola mundo!
    editor.escribir(" Adiós!"); // Hola mundo! Adiós! -> Tarea 1: Esta nueva acción vacía la pila de redo
    editor.redo(); // Intento de rehacer, no debería pasar nada (pila_redo vacía)

    editor.undo(); // Deshace " Adiós!" -> Hola mundo!
    editor.borrar(1); // Borra "!" -> Hola mundo
    editor.undo(); // Deshace el borrado -> Hola mundo!
    editor.redo(); // Rehace el borrado -> Hola mundo

    // Tarea 2: Prueba de la nueva acción reemplazar_ultima_palabra
    editor.escribir(" Rust"); // Hola mundo Rust
    editor.reemplazar_ultima_palabra(" Programación"); // Hola mundo Programación
    editor.undo(); // Deshace " Programación" -> Hola mundo Rust
    editor.redo(); // Rehace " Programación" -> Hola mundo Programación
    editor.escribir(" avanzada."); // Hola mundo Programación avanzada.
    editor.undo(); // Deshace " avanzada."
    editor.undo(); // Deshace " Programación"
}