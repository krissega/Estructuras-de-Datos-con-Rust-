use std::collections::HashMap;
use std::io::{self, Write};

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

    // TODO: implementar insert, get, remove, list
}

fn main() {
    println!("=========================================");
    println!(" Laboratorio Semana 9 — HashMap II");
    println!(" Diccionario práctico y colisiones");
    println!("=========================================\n");

    println!("Objetivo:");
    println!(" 1) Implementar CRUD de definiciones con HashMap.");
    println!(" 2) Probar colisiones usando BucketMap.");
    println!(" 3) Comparar rendimiento de backends.\n");

    println!("Pasos a seguir:");
    println!(" - Completar las funciones marcadas con TODO.");
    println!(" - Implementar parseo de 'clave = valor' para add.");
    println!(" - Implementar menú de comandos en el bucle principal.");
    println!(" - Probar con backend estándar y BucketMap.\n");

    println!("Comandos esperados:");
    println!("   add <palabra> = <definición>");
    println!("   get <palabra>");
    println!("   remove <palabra>");
    println!("   list");
    println!("   backend std|bucket");
    println!("   demo_collisions");
    println!("   help");
    println!("   exit\n");

    println!("=== ¡Ahora empieza a implementar! ===\n");

    let mut backend = "std";
    let mut dict_std: HashMap<String, String> = HashMap::new();
    let mut dict_bucket = BucketMap::with_capacity(16);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let cmd = line.trim();
        if cmd == "exit" {
            println!("Saliendo...");
            break;
        }

        // TODO: procesar comandos
    }
}