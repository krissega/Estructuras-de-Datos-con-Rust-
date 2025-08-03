#  reto_memory_game.md
## Semana 1 – Módulo 1: Vectores I
### Reto: Memory Game en Rust

---

###  Objetivo del Reto

Crear una versión sencilla de un juego tipo *Memory* en Rust donde el jugador debe adivinar una lista de palabras ocultas. Este reto te permitirá practicar el uso de vectores, mutación de datos, iteración, condiciones y entrada del usuario.

---

### 🛠 Habilidades que vas a practicar

- Declaración y mutación de vectores (`Vec<T>`)
- Uso de `push`, `len`, `iter`, `enumerate`
- Entrada desde consola (`io::stdin`)
- Comparación de cadenas (`String`)
- Control de flujo (`if`, `while`, `for`)
- Contadores y lógica condicional

---

###  Enunciado

Construye una aplicación en Rust donde:

1. Tengas una lista de palabras predefinidas como `Vec<&str>`, por ejemplo:  
   `"manzana", "pera", "uva", "melón"`

2. Uses un segundo vector `Vec<bool>` para registrar cuáles palabras ya han sido adivinadas.

3. Solicites al usuario que adivine las palabras, una por una, por consola.

4. Cada vez que el usuario adivine una palabra correctamente, debe mostrarse en pantalla reemplazando el `???`.

5. Si la palabra ya fue adivinada o es incorrecta, debe mostrarse un mensaje de advertencia.

6. El jugador tiene un máximo de **10 intentos** para completar el reto.

7. Al finalizar, muestra si ganó o perdió, y cuántos intentos utilizó.

---

###  Tips

- Puedes convertir la entrada del usuario con `.trim().to_lowercase()` para comparar correctamente.
- Usa `enumerate()` para recorrer los vectores con índice.
- Para verificar si aún quedan palabras por adivinar:  
  `adivinadas.iter().any(|&x| !x)`
- Para verificar si ya ganó:  
  `adivinadas.iter().all(|&x| x)`

---

###  Criterios de Evaluación

| Criterio                          | Puntaje |
|----------------------------------|---------|
| Uso correcto de `Vec<&str>` y `Vec<bool>` | 2 pts |
| Lógica para verificar aciertos    | 2 pts |
| Control de intentos y límite      | 2 pts |
| Mensajes de retroalimentación     | 2 pts |
| Funcionalidad completa y sin errores | 2 pts |
| **Total**                         | **10 pts** |

---

###  Nivel Extra (Opcional)

- Permite al usuario reiniciar el juego si pierde.
- Agrega colores en la consola (usando crates como `colored`).
- Usa un `Vec<String>` para permitir edición dinámica de la lista.
