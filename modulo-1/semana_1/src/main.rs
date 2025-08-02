//  Laboratorio Semana 1: Vectores Básicos
// Objetivo: Practicar la creación, modificación y lectura de vectores en Rust.

fn main() {
    // 1. Crear un vector vacío de tipo i32
    let mut numeros: Vec<i32> = Vec::new();

    // 2. Agregar tres elementos al vector usando push()
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);

    // 3. Imprimir todos los elementos con println!
    println!("Vector actual: {:?}", numeros);

    // 4. Eliminar el último elemento con pop() y mostrarlo
    if let Some(valor) = numeros.pop() {
        println!("Elemento eliminado: {}", valor);
    }

    // 5. Mostrar la longitud actual del vector
    println!("Tamaño del vector: {}", numeros.len());

    // 6. Usar un for para imprimir cada elemento individualmente
    for numero in &numeros {
        println!("Elemento: {}", numero);
    }

    // 7. Crear un slice del vector original
    let segmento = &numeros[0..1];
    println!("Slice: {:?}", segmento);
}
