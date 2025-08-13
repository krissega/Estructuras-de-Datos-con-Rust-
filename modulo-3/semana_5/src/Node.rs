// Definimos nuestro nodo de la lista enlazada
struct Node {
    data: i32, // El dato que este nodo guarda (podría ser cualquier tipo T)
    next: Option<Box<Node>>, // Un puntero al siguiente nodo, que puede existir (Some) o no (None)
}

// Y nuestra estructura de lista para manejar la cabeza
struct LinkedList {
    head: Option<Box<Node>>, // La cabeza de la lista, que también es opcional y apunta a un Box
}