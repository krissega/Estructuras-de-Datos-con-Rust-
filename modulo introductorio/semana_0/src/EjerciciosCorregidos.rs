/// Ejercicio 1: Tipos incorrectos y operación aritmética.
/// En este ejemplo corregimos un error de tipo al declarar una variable como `i32`,
/// que es el entero más común en Rust (equivalente a `int` en Java).
/// Se intenta sumar 8 a una cadena mal tipada originalmente.
/// Solución: usamos un literal entero directamente para evitar la conversión.
fn ejercicio_1() {
    let numero: i32 = 42;
    let resultado = numero + 8;
    println!("Ejercicio 1 - Resultado: {}", resultado);
}

/// Ejercicio 2: Ownership y Clonación.
/// En Rust, cuando asignamos una `String` a otra variable, se transfiere la propiedad.
/// El intento de usar la variable original después de moverla genera un error.
/// Solución: usamos `.clone()` para copiar el valor y conservar la propiedad original.
fn ejercicio_2() {
    let mensaje = String::from("Hola, Rust!");
    let copia = mensaje.clone(); // se debe clonar para evitar mover el valor
    println!("Ejercicio 2 - Mensaje original: {}", mensaje);
    println!("Ejercicio 2 - Mensaje copia: {}", copia);
}

/// Ejercicio 3: Sombra de variables (Shadowing).
/// Rust permite declarar una nueva variable con el mismo nombre, "sombreando" la anterior.
/// Esto permite modificar su valor sin hacerla mutable.
/// Solución: declaramos de nuevo `contador` y le sumamos 1, reemplazando su valor original.
fn ejercicio_3() {
    let contador = 0;
    let contador = contador + 1; // sombreado permitido
    println!("Ejercicio 3 - Contador: {}", contador);
}

fn main() {
    ejercicio_1();
    ejercicio_2();
    ejercicio_3();
}