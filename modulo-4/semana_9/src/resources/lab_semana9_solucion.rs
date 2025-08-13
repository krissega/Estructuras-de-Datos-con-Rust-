// ===============================================
// Semana 9 — HashMap II
// Laboratorio: Diccionario práctico con HashMap y demostración de colisiones
// ===============================================
//
// Objetivo:
//  - Implementar un CRUD (agregar, buscar, eliminar, listar) para un diccionario.
//  - Probar el uso de `std::collections::HashMap`.
//  - Comprender y experimentar con colisiones usando Separate Chaining y un Hasher forzado.
// ===============================================

// ----------------------
// Importaciones
// ----------------------

// HashMap de la librería estándar, usado para implementar el diccionario de definiciones.
use std::collections::HashMap;

// Módulos para entrada/salida estándar.
//  - io: lectura de texto desde el usuario (STDIN).
//  - Write: para forzar el vaciado del buffer de salida (flush) cuando mostramos el prompt.
use std::io::{self, Write};

// Traits necesarios para crear un "Hasher" personalizado y un constructor de hashers.
// Esto lo usamos para simular colisiones forzadas.
use std::hash::{BuildHasher, Hasher};

// ----------------------
// (A) Hasher que siempre devuelve 0
// ----------------------
// Esto sirve para forzar que todas las claves caigan en el mismo bucket.
// Es útil para mostrar cómo las colisiones afectan el rendimiento.
struct AlwaysZeroHasher;

impl Hasher for AlwaysZeroHasher {
    fn write(&mut self, _bytes: &[u8]) {}
    fn finish(&self) -> u64 { 0 }
}

// Constructor de nuestro hasher que siempre devuelve 0.
#[derive(Clone, Default)]
struct BuildZeroHasher;
impl BuildHasher for BuildZeroHasher {
    type Hasher = AlwaysZeroHasher;
    fn build_hasher(&self) -> AlwaysZeroHasher { AlwaysZeroHasher }
}

// ----------------------
// (B) Implementación didáctica de Separate Chaining
// ----------------------
// Esta estructura almacena pares (clave, valor) en "buckets" separados.
// Si dos claves caen en el mismo índice, se guardan en el mismo vector.
#[derive(Default)]
struct BucketMap {
    buckets: Vec<Vec<(String, String)>>,
}

impl BucketMap {
    fn with_capacity(cap: usize) -> Self {
        let mut buckets = Vec::with_capacity(cap);
        for _ in 0..cap {
            buckets.push(Vec::new());
        }
        Self { buckets }
    }

    // Función hash simple: suma de bytes de la clave mod número de buckets.
    fn index_for(&self, key: &str) -> usize {
        let sum: usize = key.as_bytes().iter().map(|b| *b as usize).sum();
        sum % self.buckets.len()
    }

    // Inserta o actualiza un valor
    fn insert(&mut self, key: String, value: String) {
        let idx = self.index_for(&key);
        let bucket = &mut self.buckets[idx];
        if let Some((_, v)) = bucket.iter_mut().find(|(k, _)| *k == key) {
            *v = value;
        } else {
            bucket.push((key, value));
        }
    }

    // Obtiene un valor
    fn get(&self, key: &str) -> Option<&str> {
        let idx = self.index_for(key);
        self.buckets[idx].iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    // Elimina un valor
    fn remove(&mut self, key: &str) -> bool {
        let idx = self.index_for(key);
        let bucket = &mut self.buckets[idx];
        if let Some(pos) = bucket.iter().position(|(k, _)| k == key) {
            bucket.remove(pos);
            true
        } else {
            false
        }
    }

    // Lista todas las claves ordenadas
    fn list(&self) -> Vec<(String, String)> {
        let mut all: Vec<(String, String)> = self.buckets
            .iter()
            .flat_map(|b| b.iter().cloned())
            .collect();
        all.sort_by(|a, b| a.0.cmp(&b.0));
        all
    }
}

// ----------------------
// (C) Backend seleccionable
// ----------------------
enum Backend {
    Std,
    Bucket,
}

// ----------------------
// (D) Función principal
// ----------------------
fn main() {
    // Mensaje inicial
    println!("=========================================");
    println!(" Semana 9 — HashMap II: Diccionario y Colisiones");
    println!("=========================================");
    println!("Objetivo:");
    println!(" - CRUD de definiciones (add, get, remove, list)");
    println!(" - Comparar HashMap estándar y BucketMap");
    println!(" - Experimentar con colisiones\n");

    println!("Comandos:");
    println!(" add <palabra> = <definición>");
    println!(" get <palabra>");
    println!(" remove <palabra>");
    println!(" list");
    println!(" backend std|bucket");
    println!(" demo_collisions");
    println!(" help");
    println!(" exit\n");

    // Estado inicial: usamos HashMap estándar.
    let mut backend = Backend::Std;
    let mut dict_std: HashMap<String, String> = HashMap::new();
    let mut dict_bucket = BucketMap::with_capacity(16);

    loop {
        // Mostramos prompt
        print!("> ");
        io::stdout().flush().unwrap();
        let line = read_line();

        // Separar comando y argumento
        let mut parts = line.trim().splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("").trim();

        match cmd {
            "add" => {
                if let Some((k, v)) = parse_kv(rest) {
                    match backend {
                        Backend::Std => {
                            dict_std.insert(k.to_string(), v.to_string());
                            println!("✔ Agregado/actualizado en HashMap estándar.");
                        }
                        Backend::Bucket => {
                            dict_bucket.insert(k.to_string(), v.to_string());
                            println!("✔ Agregado/actualizado en BucketMap.");
                        }
                    }
                } else {
                    eprintln!("Uso: add <palabra> = <definición>");
                }
            }
            "get" => {
                let key = rest;
                if key.is_empty() { eprintln!("Uso: get <palabra>"); continue; }
                match backend {
                    Backend::Std => match dict_std.get(key) {
                        Some(v) => println!("{}: {}", key, v),
                        None => println!("(no existe)"),
                    },
                    Backend::Bucket => match dict_bucket.get(key) {
                        Some(v) => println!("{}: {}", key, v),
                        None => println!("(no existe)"),
                    }
                }
            }
            "remove" => {
                let key = rest;
                if key.is_empty() { eprintln!("Uso: remove <palabra>"); continue; }
                let removed = match backend {
                    Backend::Std    => dict_std.remove(key).is_some(),
                    Backend::Bucket => dict_bucket.remove(key),
                };
                if removed { println!("🗑 Eliminado."); } else { println!("(no existía)"); }
            }
            "list" => {
                let items = match backend {
                    Backend::Std => {
                        let mut v: Vec<_> = dict_std.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                        v.sort_by(|a, b| a.0.cmp(&b.0));
                        v
                    }
                    Backend::Bucket => dict_bucket.list(),
                };
                if items.is_empty() {
                    println!("(diccionario vacío)");
                } else {
                    for (k, v) in items {
                        println!("- {}: {}", k, v);
                    }
                }
            }
            "backend" => {
                match rest {
                    "std" => { backend = Backend::Std; println!("Backend: HashMap estándar"); }
                    "bucket" => { backend = Backend::Bucket; println!("Backend: BucketMap"); }
                    _ => println!("Uso: backend std | bucket"),
                }
            }
            "demo_collisions" => {
                let mut forced: std::collections::HashMap<String, String, BuildZeroHasher> =
                    std::collections::HashMap::with_hasher(BuildZeroHasher);
                let keys = vec!["uno","dos","tres","cuatro","cinco","seis","siete","ocho","nueve"];
                for k in &keys { forced.insert(k.to_string(), format!("def-{}", k)); }
                println!("🔬 Insertadas {} claves con hasher que devuelve 0.", keys.len());
                println!("Todas colisionan en el mismo bucket interno.");
            }
            "help" => print_help(),
            "exit" | "quit" => { println!("¡Hasta luego!"); break; }
            "" => {}
            _ => eprintln!("Comando no reconocido. Usa `help`."),
        }
    }
}

// ----------------------
// (E) Funciones auxiliares
// ----------------------

// Lee una línea desde STDIN
fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}

// Separa un texto con formato "clave = valor" en dos partes
fn parse_kv(input: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = input.splitn(2, '=').collect();
    if parts.len() != 2 { return None; }
    let k = parts[0].trim();
    let v = parts[1].trim();
    if k.is_empty() || v.is_empty() { None } else { Some((k, v)) }
}

// Muestra la lista de comandos
fn print_help() {
    println!("\nComandos disponibles:
  add <palabra> = <definición>
  get <palabra>
  remove <palabra>
  list
  backend std|bucket
  demo_collisions
  help
  exit\n");
}