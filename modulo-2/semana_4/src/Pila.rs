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