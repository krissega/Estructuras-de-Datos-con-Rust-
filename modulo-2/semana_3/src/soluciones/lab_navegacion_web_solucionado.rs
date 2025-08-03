// lab_navegacion_web.rs

// --- Reutilización de la estructura Pila<T> ---
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
    pila_adelante: Pila<String>, // Páginas a las que se puede avanzar después de 'Atrás'
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
        println!("  (Páginas Atrás: {}, Páginas Adelante: {})", self.pila_atras.size(), self.pila_adelante.size());
    }

    // Navega a una nueva página
    fn ir_a(&mut self, nueva_pagina: &str) {
        // Solo guarda la página actual en 'atras' si es diferente
        if self.pagina_actual != nueva_pagina.to_string() {
            self.pila_atras.push(self.pagina_actual.clone());
        }
        self.pagina_actual = nueva_pagina.to_string();
        // Tarea 1: Al ir a una nueva página, se vacía la pila de adelante
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
            // Tarea 2: Mensaje de borde mejorado
            println!("\nYa estás en el inicio del historial de navegación. No hay páginas para ir 'Atrás'.");
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
            // Tarea 2: Mensaje de borde mejorado
            println!("\nNo hay páginas para avanzar en el historial.");
        }
    }
}

fn main() {
    println!("--- Simulador de Navegador Web ---");

    let mut navegador = NavegadorWeb::new("inicio.com");
    navegador.mostrar_pagina();

    navegador.ir_a("blog.com/post1");
    navegador.ir_a("noticias.com/articulo");
    navegador.ir_a("tienda.com/productoX");

    navegador.atras(); // De productoX a noticias.com
    navegador.atras(); // De noticias.com a blog.com
    navegador.adelante(); // De blog.com a noticias.com
    navegador.ir_a("galeria.com/fotos"); // Esta nueva navegación vacía la pila de adelante
    navegador.adelante(); // Intento de ir adelante, no debería pasar nada (pila_adelante vacía)
    navegador.atras(); // De galeria.com a tienda.com (la página visitada antes de galeria.com)
    navegador.ir_a("final.com"); // Otra nueva navegación

    println!("\n--- Probando límites del historial ---");
    navegador.atras(); // A tienda.com
    navegador.atras(); // A noticias.com
    navegador.atras(); // A blog.com
    navegador.atras(); // A inicio.com
    navegador.atras(); // Tarea 2: Debería mostrar mensaje de "inicio del historial"

    navegador.ir_a("nueva_pagina.com"); // Resetear el historial de adelante
    navegador.adelante(); // Tarea 2: Debería mostrar mensaje de "no hay páginas para avanzar"

    //toDo adicionalmente agrego un ejemplo por si se quiere dejar una tarea adicional para puntos extras para los estudiantes

    // Tarea 3 (Opcional): Ejemplo de bucle interactivo
    /*
    use std::io::{self, Write};
    let mut navegador_interactivo = NavegadorWeb::new("bienvenida.com");
    navegador_interactivo.mostrar_pagina();

    println!("\n--- Modo Interactivo ---");
    println!("Comandos: ir <url>, atras, adelante, salir");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Asegura que el prompt se muestre antes de leer la entrada

        let mut comando = String::new();
        io::stdin().read_line(&mut comando).expect("Fallo al leer la línea");
        let comando = comando.trim();

        if comando.starts_with("ir ") {
            let url = comando.trim_start_matches("ir ");
            navegador_interactivo.ir_a(url);
        } else if comando == "atras" {
            navegador_interactivo.atras();
        } else if comando == "adelante" {
            navegador_interactivo.adelante();
        } else if comando == "salir" {
            println!("Saliendo del navegador interactivo.");
            break;
        } else {
            println!("Comando no reconocido. Intente: ir <url>, atras, adelante, salir");
        }
    }
    */
}