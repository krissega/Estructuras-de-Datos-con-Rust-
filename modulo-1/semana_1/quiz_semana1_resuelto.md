# Quiz Semana 1 Resuelto: Vectores en Rust

1. **¿Qué diferencia principal existe entre un `Vec<T>` y un arreglo (`[T; N]`) en Rust?**
   - Un `Vec<T>` es dinámico y puede cambiar de tamaño en tiempo de ejecución, mientras que un arreglo tiene un tamaño fijo determinado en tiempo de compilación.

2. **Escribe una línea de código que cree un vector vacío de enteros.**
   ```rust
   let mut numeros: Vec<i32> = Vec::new();
   ```

3. **¿Qué método se utiliza para agregar un nuevo elemento al final de un vector?**
   - El método `.push()`.

4. **Dado el vector `let nombres = vec!["Ana", "Luis", "Marta"];`, escribe un bucle `for` que imprima cada nombre.**
   ```rust
   for nombre in &nombres {
       println!("{}", nombre);
   }
   ```

5. **¿Qué devuelve el método `.len()` aplicado a un vector y por qué es útil?**
   - Devuelve el número de elementos actuales en el vector. Es útil para saber cuántos elementos hay sin necesidad de contarlos manualmente.

6. **¿Qué sucede si intentas acceder a un índice que no existe en el vector? ¿Cómo puedes evitar que el programa se caiga?**
   - El programa entrará en pánico (panic). Puedes evitarlo usando el método `.get(indice)`, que devuelve un `Option<T>` y puedes manejar el caso de `None`.
