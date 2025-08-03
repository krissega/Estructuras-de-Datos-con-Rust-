// Reutilizamos nuestra estructura Pila<T> del tutorial base
pub struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    pub fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elementos.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elementos.len()
    }
}

// --- EDITOR DE TEXTO SIMPLIFICADO ---

struct EditorTexto {
    contenido: String,
    pila_undo: Pila<String>, // Guarda los estados anteriores
    pila_redo: Pila<String>, // Guarda los estados que pueden ser rehechos
}

impl EditorTexto {
    fn new(contenido_inicial: &str) -> Self {
        let mut editor = EditorTexto {
            contenido: contenido_inicial.to_string(),
            pila_undo: Pila::new(),
            pila_redo: Pila::new(),
        };
        // Guarda el estado inicial para poder deshacer si es necesario
        editor.pila_undo.push(contenido_inicial.to_string());
        editor
    }

    fn mostrar_contenido(&self) {
        println!("Contenido actual: \"{}\"", self.contenido);
        println!("  (Undo: {}, Redo: {})", self.pila_undo.size() -1, self.pila_redo.size());
    }

    // Simula una acción de edición: añadir texto
    fn escribir(&mut self, texto: &str) {
        // Antes de modificar, guarda el estado actual para "deshacer"
        self.pila_undo.push(self.contenido.clone());
        self.contenido.push_str(texto);
        // Cualquier nueva acción invalida la pila de rehacer
        self.pila_redo = Pila::new();
        println!("\n--- Escribiendo: \"{}\" ---", texto);
        self.mostrar_contenido();
    }

    // Simula una acción de edición: borrar n caracteres
    fn borrar(&mut self, n: usize) {
        if self.contenido.len() >= n {
            self.pila_undo.push(self.contenido.clone());
            self.contenido.truncate(self.contenido.len() - n);
            self.pila_redo = Pila::new();
            println!("\n--- Borrando {} caracteres ---", n);
            self.mostrar_contenido();
        } else {
            println!("\nNo se pueden borrar {} caracteres. Contenido muy corto.", n);
        }
    }

    // Deshace la última acción
    fn undo(&mut self) {
        if self.pila_undo.size() > 1 { // Debe haber al menos un estado anterior al actual
            let estado_actual = self.pila_undo.pop().unwrap(); // Sacamos el estado actual
            self.pila_redo.push(estado_actual); // Lo movemos a la pila de rehacer
            if let Some(estado_anterior) = self.pila_undo.peek() {
                self.contenido = estado_anterior.clone(); // Restauramos el estado anterior
            } else {
                println!("Error: No se pudo obtener el estado anterior de la pila de undo.");
            }
            println!("\n--- Deshaciendo ---");
            self.mostrar_contenido();
        } else {
            println!("\nNo hay nada que deshacer.");
        }
    }

    // Rehace la última acción deshecha
    fn redo(&mut self) {
        if let Some(estado_rehecho) = self.pila_redo.pop() {
            self.pila_undo.push(self.contenido.clone()); // Guardamos el estado actual antes de rehacer
            self.contenido = estado_rehecho;
            println!("\n--- Rehaciendo ---");
            self.mostrar_contenido();
        } else {
            println!("\nNo hay nada que rehacer.");
        }
    }
}

fn main() {
    let mut editor = EditorTexto::new("Hola");
    editor.mostrar_contenido();

    editor.escribir(" mundo");
    editor.escribir("!");
    editor.escribir(" Qué tal?");

    editor.undo(); // Deshace " Qué tal?"
    editor.undo(); // Deshace "!"
    editor.redo(); // Rehace "!"
    editor.escribir(" Adiós!"); // Esta nueva acción vacía la pila de redo
    editor.redo(); // Intento de rehacer, no debería pasar nada
    editor.undo();
    editor.borrar(2); // Borra "s!"
    editor.undo(); // Deshace el borrado
}