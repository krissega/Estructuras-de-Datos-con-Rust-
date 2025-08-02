fn main() {
    //No se puede modificar una variable no mutable
    // por eso agregamos mut para poder cambiar su valor
    let mut contador = 0;
    contador += 1;
    println!("Contador: {}", contador);

    let mensaje = String::from("Hola, Rust!");
    let copia = mensaje.clone(); // ✅ Clonamos la memoria del String
    println!("Mensaje original: {}", mensaje);
    println!("Copia: {}", copia);

    let contador = 0;
    contador += 1;
    println!("Contador: {}", contador);

}

