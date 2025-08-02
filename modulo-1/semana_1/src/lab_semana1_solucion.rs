/// Laboratorio 1 - Solución: Vectores básicos en Rust
/// Objetivo: Practicar declaración, mutación y operaciones comunes con vectores.

fn main() {
    //  Declaración de un vector vacío de enteros
    let mut numeros: Vec<i32> = Vec::new();

    //  También se puede declarar con macro vec!
    // let mut numeros = vec![1, 2, 3];

    //  Agregar elementos con push
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);

    //  Mostrar contenido actual del vector
    println!("Contenido del vector: {:?}", numeros); // [10, 20, 30]

    //  Acceder por índice
    println!("Primer número: {}", numeros[0]); // 10

    //  Obtener la longitud del vector
    println!("Cantidad de elementos: {}", numeros.len()); // 3

    //  Recorrer el vector con un for tradicional
    println!("Recorriendo el vector:");
    for i in 0..numeros.len() {
        println!("Elemento {}: {}", i, numeros[i]);
    }

    //  Recorrer con iterador (for-each)
    println!("Usando iter():");
    for numero in numeros.iter() {
        println!("Número: {}", numero);
    }

    //  Quitar el último elemento con pop
    if let Some(ultimo) = numeros.pop() {
        println!("Elemento eliminado con pop(): {}", ultimo); // 30
    }

    println!("Vector después de pop(): {:?}", numeros); // [10, 20]

    //  Usar slices para obtener una porción del vector
    let subvector = &numeros[0..1]; // primer elemento
    println!("Slice del vector: {:?}", subvector); // [10]

    //  Modificar un valor directamente
    numeros[0] = 99;
    println!("Vector modificado: {:?}", numeros); // [99, 20]
}