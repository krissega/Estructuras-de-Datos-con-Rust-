fn main() {
    println!("=========================================");
    println!(" Semana 9 — HashMap II: Diccionario y Colisiones");
    println!("=========================================\n");

    println!("Objetivo:");
    println!(" - Implementar un CRUD de definiciones (add, get, remove, list)");
    println!(" - Comparar HashMap estándar y BucketMap");
    println!(" - Experimentar con colisiones usando un hasher personalizado");
    println!(" - Analizar rendimiento en diferentes condiciones\n");

    println!("Comandos disponibles:");
    println!(" add <palabra> = <definición>   -> Agrega o actualiza una definición");
    println!(" get <palabra>                  -> Busca y muestra una definición");
    println!(" remove <palabra>               -> Elimina una definición existente");
    println!(" list                           -> Lista todas las definiciones ordenadas");
    println!(" backend std|bucket             -> Cambia el backend de almacenamiento");
    println!(" demo_collisions                -> Ejecuta la demo de colisiones forzadas");
    println!(" help                           -> Muestra esta ayuda");
    println!(" exit                           -> Sale del programa\n");

    println!("=== Instrucciones para el laboratorio ===");
    println!(" 1) Estudia el código base proporcionado.");
    println!(" 2) Completa las funciones y lógica marcadas con TODO en la plantilla.");
    println!(" 3) Prueba cada comando y analiza su funcionamiento.");
    println!(" 4) Cambia entre 'std' y 'bucket' para comparar resultados.");
    println!(" 5) Ejecuta 'demo_collisions' para observar el efecto de las colisiones.");
    println!(" 6) Documenta tus conclusiones.\n");

    println!("=== Fin de las instrucciones, feli aprendizaje  ===");
}