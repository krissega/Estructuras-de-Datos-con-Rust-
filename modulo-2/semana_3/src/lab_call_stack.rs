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
}

// Creamos una pila global (o la pasamos por referencia en funciones) para simular el call stack
// NOTA: En un entorno real, la pila de llamadas es gestionada por el SO, esto es una simulación.
static mut MI_CALL_STACK: Option<Pila<&'static str>> = None;

// Función de inicialización para la pila de llamadas
fn inicializar_call_stack() {
    unsafe {
        MI_CALL_STACK = Some(Pila::new());
    }
}

// Función para "llamar" una función (simulado)
fn llamar_funcion(nombre_funcion: &'static str) {
    unsafe {
        if let Some(stack) = MI_CALL_STACK.as_mut() {
            println!("-> Llamando a '{}'", nombre_funcion);
            stack.push(nombre_funcion);
            // Simula alguna lógica dentro de la función
            println!("   Dentro de '{}'", nombre_funcion);
        }
    }
}

// Función para "retornar" de una función (simulado)
fn retornar_funcion() {
    unsafe {
        if let Some(stack) = MI_CALL_STACK.as_mut() {
            if let Some(funcion_actual) = stack.pop() {
                println!("<- Retornando de '{}'", funcion_actual);
                if let Some(siguiente_funcion) = stack.peek() { //peek para ver la funcion que sigue
                    println!("   Reanudando en '{}'", siguiente_funcion);
                } else {
                    println!("   Pila de llamadas vacía. Programa terminado.");
                }
            } else {
                println!("Error: No hay funciones en la pila para retornar.");
            }
        }
    }
}


// --- FUNCIONES DE EJEMPLO ---

fn funcion_a() {
    llamar_funcion("funcion_a");
    // Llama a otra función
    funcion_b();
    retornar_funcion();
}

fn funcion_b() {
    llamar_funcion("funcion_b");
    // Llama a otra función
    funcion_c();
    retornar_funcion();
}

fn funcion_c() {
    llamar_funcion("funcion_c");
    // Esta función no llama a ninguna otra
    retornar_funcion();
}


fn main() {
    inicializar_call_stack();
    println!("--- INICIO DEL PROGRAMA ---");
    funcion_a();
    println!("--- FIN DEL PROGRAMA ---");
}