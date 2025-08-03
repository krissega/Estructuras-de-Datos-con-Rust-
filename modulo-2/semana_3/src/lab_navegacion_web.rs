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

// --- SIMULADOR DE NAVEGADOR WEB ---

struct NavegadorWeb {
    pagina_actual: String,
    pila_atras: Pila<String>,    // Páginas visitadas a las que se puede regresar
    pila_adelante: Pila<String>, // Páginas a las que se puede ir después de 'Atrás'
}

impl NavegadorWeb {
    fn new(pagina_inicial: &str) -> Self {
        NavegadorWeb {
            pagina_actual: pagina_inicial.to_string(),
            pila_atras: Pila::new(),
            pila_adelante: Pila::new(),
        }
    }

    fn mostrar_pagina(&self) {
        println!("\nActualmente en: \"{}\"", self.pagina_actual);
        println!("  (Atrás: {}, Adelante: {})", self.pila_atras.size(), self.pila_adelante.size());
    }

    // Navega a una nueva página
    fn ir_a(&mut self, nueva_pagina: &str) {
        // La página actual se guarda en la pila de atrás
        self.pila_atras.push(self.pagina_actual.clone());
        self.pagina_actual = nueva_pagina.to_string();
        // Al ir a una nueva página, se vacía la pila de adelante
        self.pila_adelante = Pila::new();
        println!("--- Navegando a: \"{}\" ---", nueva_pagina);
        self.mostrar_pagina();
    }

    // Retrocede en el historial
    fn atras(&mut self) {
        if let Some(pagina_anterior) = self.pila_atras.pop() {
            // La página actual se guarda para poder ir "adelante"
            self.pila_adelante.push(self.pagina_actual.clone());
            self.pagina_actual = pagina_anterior; // Se restaura la página anterior
            println!("--- Atrás ---");
            self.mostrar_pagina();
        } else {
            println!("\nNo hay historial para ir 'Atrás'.");
        }
    }

    // Avanza en el historial (después de haber usado 'Atrás')
    fn adelante(&mut self) {
        if let Some(pagina_siguiente) = self.pila_adelante.pop() {
            // La página actual se guarda para poder ir "atrás"
            self.pila_atras.push(self.pagina_actual.clone());
            self.pagina_actual = pagina_siguiente; // Se restaura la página siguiente
            println!("--- Adelante ---");
            self.mostrar_pagina();
        } else {
            println!("\nNo hay historial para ir 'Adelante'.");
        }
    }
}

fn main() {
    let mut navegador = NavegadorWeb::new("inicio.com");
    navegador.mostrar_pagina();

    navegador.ir_a("blog.com/post1");
    navegador.ir_a("noticias.com/articulo");
    navegador.ir_a("tienda.com/productoX");

    navegador.atras(); // De productoX a noticias.com
    navegador.atras(); // De noticias.com a blog.com
    navegador.adelante(); // De blog.com a noticias.com
    navegador.ir_a("galeria.com/fotos"); // Esta nueva navegación vacía la pila de adelante
    navegador.adelante(); // Intento de ir adelante, no debería pasar nada
    navegador.atras(); // De galeria.com a tienda.com
    navegador.ir_a("final.com"); // Otra nueva navegación
}