// main.rs

// --- Definición e Implementación de la Pila ---
// Definimos nuestra estructura Pila, que internamente usará un Vec<T>
pub struct Pila<T> {
    elementos: Vec<T>,
}

// Implementamos los métodos para nuestra Pila
impl<T> Pila<T> {
    // Constructor: Crea una nueva pila vacía
    pub fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }

    // Método push: Añade un elemento a la parte superior de la pila
    pub fn push(&mut self, item: T) {
        self.elementos.push(item);
    }

    // Método pop: Saca y devuelve el elemento superior de la pila.
    // Devuelve un `Option<T>` porque la pila podría estar vacía.
    pub fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    // Método peek: Permite ver el elemento superior sin eliminarlo.
    // También devuelve un `Option<&T>`
    pub fn peek(&self) -> Option<&T> {
        self.elementos.last()
    }

    // Método is_empty: Comprueba si la pila está vacía.
    pub fn is_empty(&self) -> bool {
        self.elementos.is_empty()
    }

    // Método size: Devuelve el número de elementos en la pila.
    pub fn size(&self) -> usize {
        self.elementos.len()
    }
}

// --- Validador de Expresiones Balanceadas ---
pub fn es_expresion_balanceada(expresion: &str) -> bool {
    let mut pila = Pila::new(); // Creamos una pila para caracteres

    for caracter in expresion.chars() {
        match caracter {
            // Si es un carácter de apertura, lo empujamos a la pila
            '(' | '[' | '{' => pila.push(caracter),
            // Si es un carácter de cierre
            ')' => {
                // Si la pila está vacía o el tope no es '(', no está balanceada
                if pila.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                // Si la pila está vacía o el tope no es '[', no está balanceada
                if pila.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                // Si la pila está vacía o el tope no es '{', no está balanceada
                if pila.pop() != Some('{') {
                    return false;
                }
            }
            // Ignoramos otros caracteres como números u operadores
            _ => {}
        }
    }

    // Si al final la pila está vacía, significa que todos los paréntesis/corchetes están balanceados
    pila.is_empty()
}

// --- Función Principal de Ejemplo ---
fn main() {
    println!("--- Demostración de Pila (Stack) en Rust ---");

    // Ejemplo de uso de la Pila genérica
    let mut mi_pila = Pila::new();

    mi_pila.push(10);
    mi_pila.push(20);
    mi_pila.push(30);

    println!("\nEstado inicial de la pila:");
    println!("  La pila tiene {} elementos.", mi_pila.size()); // Salida: La pila tiene 3 elementos.
    if let Some(&peeked_item) = mi_pila.peek() {
        println!("  El elemento superior es (sin sacar): {}", peeked_item); // Salida: El elemento superior es (sin sacar): 30
    }

    if let Some(top_item) = mi_pila.pop() {
        println!("  Se sacó de la pila: {}", top_item); // Salida: Se sacó de la pila: 30
    }

    println!("  La pila ahora tiene {} elementos.", mi_pila.size()); // Salida: La pila ahora tiene 2 elementos.

    while let Some(item) = mi_pila.pop() {
        println!("  Sacando: {}", item); // Salida: Sacando: 20, Sacando: 10
    }

    println!("  ¿Pila vacía al final? {}", mi_pila.is_empty()); // Salida: ¿Pila vacía al final? true

    println!("\n--- Demostración de Validador de Expresiones ---");

    // Ejemplos de uso del validador de expresiones
    let expr1 = "({[]})";
    println!("'{}' está balanceada: {}", expr1, es_expresion_balanceada(expr1));

    let expr2 = "([)]";
    println!("'{}' está balanceada: {}", expr2, es_expresion_balanceada(expr2));

    let expr3 = "{[}";
    println!("'{}' está balanceada: {}", expr3, es_expresion_balanceada(expr3));

    let expr4 = "hola(mundo)";
    println!("'{}' está balanceada: {}", expr4, es_expresion_balanceada(expr4));

    let expr5 = "((a+b)*[c-d])/e";
    println!("'{}' está balanceada: {}", expr5, es_expresion_balanceada(expr5));

    let expr6 = "({[}])"; // Mal anidada
    println!("'{}' está balanceada: {}", expr6, es_expresion_balanceada(expr6));
}