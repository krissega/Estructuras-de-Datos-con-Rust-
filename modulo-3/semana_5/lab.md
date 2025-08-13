#  Laboratorio 5: Listas Enlazadas I (Rust)

##  Objetivo del Laboratorio

El objetivo de este laboratorio es que implementes una **Lista Enlazada básica en Rust**. Aplicarás los conceptos teóricos de `Box<T>` y `Option<T>` para construir una estructura de datos dinámica que te permita insertar elementos al inicio y recorrer la lista.

---

##  Conceptos Clave a Aplicar

- **Listas Enlazadas:** Comprender la estructura de nodos y cómo se conectan entre sí.
- **Box<T>:** Usar punteros inteligentes para almacenar nodos en el heap, resolviendo la recursividad infinita en las definiciones de tipo.
- **Option<T>:** Manejar la posibilidad de que un nodo no tenga un siguiente (`None`), marcando el final de la lista.
- **Manejo de la `head`:** Entender que `head` es el punto de entrada principal para toda la lista.

---

##  Archivos del Laboratorio

- `lab_linkedlist_box.rs`: Archivo principal donde desarrollarás el laboratorio. Contiene la base del código y secciones marcadas con `// TODO:` que debes completar.
- `quiz_listas_1.md`: Cuestionario de repaso para consolidar los conceptos teóricos de la semana.

---

##  Instrucciones

1. **Abre el archivo del laboratorio:**

   En tu repositorio del curso, navega hasta `lab_linkedlist_box.rs` y ábrelo.

2. **Completa el código:**

   Ubica las secciones marcadas con `// TODO:` y completa las tareas indicadas.

3. **Ejecuta el programa:**

   Asegúrate de estar en la raíz del repositorio y ejecuta el siguiente comando en tu terminal:

   ```bash cargo run
    --bin lab_linkedlist_box

## Tareas a Completar

A continuación se listan las tareas principales que debes implementar:

- [ ] Definir la estructura `Node<T>` con los campos `data` y `next`.
- [ ] Definir la estructura `LinkedList<T>` con el campo `head`.
- [ ] Implementar el método `new()` para crear una lista vacía.
- [ ] Implementar el método `push_front()` para insertar un nodo al inicio.
- [ ] Implementar el método `print_list()` para recorrer e imprimir los nodos.

##  Ejemplo Visual (Diagrama)

```text
Antes de insertar:
head -> None

Después de insertar 10:
head -> [10] -> None

Después de insertar 20:
head -> [20] -> [10] -> None

Después de insertar 30:
head -> [30] -> [20] -> [10] -> None

##  Resultado Esperado

Una vez finalizada tu implementación y ejecutado el programa, deberías obtener una salida similar a esta:

```text
--- Laboratorio de Listas Enlazadas ---
Lista inicial:
--- Recorrido de la lista ---
None

Lista después de insertar 30, 20, 10:
--- Recorrido de la lista ---
30 -> 20 -> 10 -> None

Lista después de insertar 5:
--- Recorrido de la lista ---
5 -> 30 -> 20 -> 10 -> None

