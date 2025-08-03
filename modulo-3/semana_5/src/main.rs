mod Node;
mod lab_linkedlist_box;

fn main() {
    println!("--- Demostración de Option<T> ---");

    let numero_valido: Option<i32> = Some(42);
    let ningun_numero: Option<i32> = None;

    // Manejando Option con 'match'
    println!("\nManejo con 'match':");
    match numero_valido {
        Some(valor) => println!("'numero_valido' contiene: {}", valor),
        None => println!("'numero_valido' no contiene nada."),
    }

    match ningun_numero {
        Some(valor) => println!("'ningun_numero' contiene: {}", valor),
        None => println!("'ningun_numero' no contiene nada."),
    }

    // Manejando Option con 'if let' (cuando solo nos interesa el caso Some)
    println!("\nManejo con 'if let':");
    if let Some(valor) = numero_valido {
        println!("Usando 'if let', el valor es: {}", valor);
    } else {
        println!("Usando 'if let', no hay valor presente.");
    }
}