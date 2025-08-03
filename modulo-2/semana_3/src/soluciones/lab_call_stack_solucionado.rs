// lab_call_stack.rs

// --- Reutilización de la estructura Pila<T> (ya vista en el tutorial) ---
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

    pub fn peek(&self) -> Option<&T> {
        self.elementos.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elementos.is_empty()
    }
}

// --- Simulación de la Pila de Llamadas (Call Stack) ---
// Usamos 'unsafe' para simular una pila global, ya que en Rust las variables mutables
// globales son inherentemente inseguras por problemas de concurrencia.
// En un escenario real, la pila de llamadas es gestionada por el sistema operativo.
static mut MI_CALL_STACK: Option<Pila<&'static str>> = None;

// Función de inicialización para la pila de llamadas simulada
fn inicializar_call_stack() {
    unsafe {
        // Aseguramos que la pila se inicialice solo una vez o se restablezca.
        MI_CALL_STACK = Some(Pila::new());
    }
}

// Función que simula una "llamada" a una función.
// Añade el nombre de la función a nuestra pila de llamadas simulada.
fn llamar_funcion(nombre_funcion: &'static str) {
    unsafe {
        if let Some(stack) = MI_CALL_STACK.as_mut() {
            println!("-> Llamando a '{}'", nombre_funcion);
            stack.push(nombre_funcion);
            // Simula alguna lógica dentro de la función actual
            println!("   [Dentro de '{}']", nombre_funcion);
        }
    }
}

// Función que simula un "retorno" de una función.
// Saca el nombre de la función actual de nuestra pila de llamadas simulada.
fn retornar_funcion() {
    unsafe {
        if let Some(stack) = MI_CALL_STACK.as_mut() {
            if let Some(funcion_actual) = stack.pop() {
                println!("<- Retornando de '{}'", funcion_actual);
                // Tarea 4: Mostrar la función a la que se reanuda la ejecución
                if let Some(siguiente_funcion) = stack.peek() {
                    println!("   Reanudando en '{}'", siguiente_funcion);
                } else {
                    // Tarea 3: Mensaje específico al vaciarse la pila
                    println!("   Pila de llamadas vacía. Fin de la ejecución del programa simulado.");
                }
            } else {
                // Tarea 3: Mensaje si se intenta retornar con pila vacía
                println!("Error: No hay funciones en la pila para retornar (Pila vacía).");
            }
        }
    }
}

// --- FUNCIONES DE EJEMPLO para simular el flujo de llamadas ---

fn funcion_a() {
    llamar_funcion("funcion_a");
    // Llama a otra función
    funcion_b(); // Llamada a función_b
    retornar_funcion();
}

fn funcion_b() {
    llamar_funcion("funcion_b");
    // Tarea 2: Llamada a la nueva funcion_d
    funcion_d();
    // Llama a otra función
    funcion_c(); // Llamada a función_c
    retornar_funcion();
}

fn funcion_c() {
    llamar_funcion("funcion_c");
    // Esta función no llama a ninguna otra, solo retorna
    retornar_funcion();
}

// Tarea 2: Nueva función añadida
fn funcion_d() {
    llamar_funcion("funcion_d");
    // Esta función no llama a ninguna otra, solo retorna
    retornar_funcion();
}


fn main() {
    // Es crucial inicializar la pila antes de cualquier operación
    inicializar_call_stack();
    println!("--- INICIO DEL PROGRAMA SIMULADO ---");

    // Inicio del flujo de llamadas desde main
    funcion_a();

    println!("--- FIN DEL PROGRAMA SIMULADO ---");
    // Después de que funcion_a termina, la pila debería estar vacía.
    // Un intento adicional de retornar confirmaría el estado.
    // retornar_funcion(); // Descomenta para probar el mensaje de pila vacía al final
}