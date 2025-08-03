# Semana 4: Laboratorio Guiado - Simulador de Fila (Queues) en Rust

¡Bienvenidos al laboratorio de la Semana 4! Aquí pondremos en práctica los conceptos de Colas (Queues) usando Rust y la estructura de datos `VecDeque`.

## Contenido de esta Carpeta

* `lab_queue_simulator.rs`: Este es el archivo principal del laboratorio que **deberás completar**. Contiene la estructura básica del programa, algunos mensajes de ayuda y comentarios que te guiarán.
* `README.md`: Este archivo que estás leyendo, con todas las instrucciones detalladas.

## Instrucciones de Configuración y Ejecución del Proyecto

Sigue estos pasos para configurar tu entorno y empezar a trabajar en el laboratorio:

### Paso 1: Crear la Estructura de tu Proyecto Local

1.  Abre tu **terminal** (o Símbolo del Sistema / PowerShell en Windows).
2.  **Crea una nueva carpeta** para tu proyecto de laboratorio en el lugar que prefieras en tu computadora. Puedes usar un comando como el siguiente (sustituye `nombre_de_tu_proyecto_cola` por un nombre que te guste, por ejemplo, `simulador_fila_semana4`):
    ```bash
    mkdir nombre_de_tu_proyecto_cola
    ```
3.  **Navega dentro de esa nueva carpeta:**
    ```bash
    cd nombre_de_tu_proyecto_cola
    ```
4.  **Inicializa un nuevo proyecto de Rust** con `cargo`. Para este laboratorio, crearemos un proyecto con un archivo de inicio con el nombre de nuestro laboratorio:
    ```bash
    cargo new --bin lab_queue_simulator
    ```
    * Este comando creará automáticamente la estructura básica de un proyecto Rust, incluyendo una carpeta `src/` y dentro de ella un archivo `src/lab_queue_simulator.rs`.

### Paso 2: Descargar el Archivo del Laboratorio Pre-escrito

1.  Ahora, necesitas descargar el contenido base del laboratorio que hemos preparado para ti. Dirígete a la siguiente URL en tu navegador:
    [https://github.com/krissega/Estructuras-de-Datos-con-Rust-/blob/main/Modulo2/Semana4/lab_queue_simulator.rs](https://github.com/krissega/Estructuras-de-Datos-con-Rust-/blob/main/Modulo2/Semana4/lab_queue_simulator.rs)
    *(**Importante:** Verifica que esta sea la ruta final del archivo `lab_queue_simulator.rs` una vez subido al repositorio. Si el profesor lo coloca en otro lugar, la URL aquí deberá actualizarse.)*
2.  Una vez en esa página, busca el botón **"Raw"** (generalmente arriba a la derecha del código) o **"Download"** para descargar el archivo `lab_queue_simulator.rs`.

### Paso 3: Pegar el Contenido Descargado en tu Proyecto Local

1.  Abre la carpeta de tu proyecto (`nombre_de_tu_proyecto_cola`) en tu **editor de código** preferido (como Visual Studio Code).
2.  Navega a la carpeta `src/` dentro de tu proyecto. Encontrarás el archivo `lab_queue_simulator.rs` que `cargo new` creó en el Paso 1.
3.  **Copia todo el contenido** del archivo `lab_queue_simulator.rs` que descargaste en el Paso 2.
4.  **Pega este contenido** *sobreescribiendo completamente* el contenido del archivo `src/lab_queue_simulator.rs` que ya está en tu proyecto local.
    * **¡Asegúrate:** de que el archivo final en tu carpeta `src/` se llame exactamente **`lab_queue_simulator.rs`!**

### Paso 4: ¡Completar el Laboratorio y Ejecutar!

1.  Ahora, abre el archivo `src/lab_queue_simulator.rs` en tu editor. Dentro de este archivo, encontrarás comentarios y marcadores como `// TU CÓDIGO AQUÍ:` donde deberás escribir tu solución.
2.  **Sigue las "Tareas" que se describen a continuación en este `README.md`** y escribe el código necesario en el archivo `lab_queue_simulator.rs`.
3.  Para **ejecutar tu código** y ver los resultados (después de guardar tus cambios), abre la terminal *en la carpeta raíz de tu proyecto* (donde está el archivo `Cargo.toml`) y ejecuta el siguiente comando:
    ```bash
    cargo run --bin lab_queue_simulator
    ```

---

## Tareas del Laboratorio: Simulador de Fila

A continuación, se detallan las tareas que debes completar en el archivo `src/lab_queue_simulator.rs`.

#### **Parte 1: La Fila Vacía - ¡Creando nuestra Cola!**

**Tarea 1.1: Inicializar la Cola**
* **En el archivo `lab_queue_simulator.rs` (dentro de `fn main() {`):**
    * Crea una nueva `VecDeque<String>` llamada `fila_clientes`.
    * Asegúrate de que la variable `fila_clientes` sea **mutable** (`mut`), ya que necesitarás añadir y quitar elementos de ella.
    * La línea `println!("\nLa fila al inicio está vacía: {:?}", fila_clientes);` ya está en el archivo y te ayudará a verificar tu progreso.

---

#### **Parte 2: ¡Llegan los Clientes! - Encolando Elementos**

**Tarea 2.1: Agregar clientes a la fila**
* **En el archivo `lab_queue_simulator.rs`:**
    * Simula la llegada de clientes. Añade los siguientes nombres a tu `fila_clientes`, **EXACTAMENTE EN ESTE ORDEN**:
        1.  `"Pedro"`
        2.  `"María"`
        3.  `"Juan"`
    * **Usa la operación `push_back()`** para añadir elementos al final de la cola.
    * **Recuerda:** Los textos como `"Pedro"` son de tipo `&str`. Para añadirlos a una `VecDeque<String>`, necesitarás convertirlos a `String` (por ejemplo, `String::from("Pedro")`).
    * La línea `println!("Después de que llegan los clientes: {:?}", fila_clientes);` te ayudará a verificar tu progreso.

**Prueba tu código aquí:**
* Guarda los cambios en `lab_queue_simulator.rs`.
* En tu terminal (desde la carpeta raíz del proyecto), ejecuta: `cargo run --bin lab_queue_simulator`
* Verifica la salida: ¿Aparecen los clientes en el orden en que los agregaste?

---

#### **Parte 3: ¿Quién es el Siguiente? - Viendo el Frente de la Cola**

**Tarea 3.1: Ver al primer cliente**
* **En el archivo `lab_queue_simulator.rs`:**
    * Queremos saber quién es el siguiente cliente a ser atendido, pero **sin quitarlo de la fila**.
    * **Usa la operación `front()`** en tu `fila_clientes`.
    * **Manejo de `Option`:** Recuerda que `front()` devuelve un `Option<T>` (puede haber un valor `Some(cliente)` o `None` si la cola está vacía). Usa una estructura `if let Some(siguiente_cliente) = ... { ... } else { ... }` para manejar ambos casos.
        * Si hay un cliente, imprime un mensaje claro como: `"El siguiente cliente a ser atendido es: [Nombre del Cliente]"`.
        * Si la fila está vacía, imprime: `"No hay nadie en la fila para ver."`.
    * Después de esta operación, la cola no debe haber cambiado. La línea `println!("La fila después de solo mirar al frente (no cambia): {:?}", fila_clientes);` te ayudará a verificar esto.

---

#### **Parte 4: ¡Atendiendo Clientes! - Desencolando Elementos**

**Tarea 4.1: Atender al primer cliente**
* **En el archivo `lab_queue_simulator.rs`:**
    * Es hora de que la panadera atienda al primer cliente.
    * **Usa la operación `pop_front()`** para sacar el elemento que está al frente de la cola.
    * **Manejo de `Option`:** `pop_front()` también devuelve un `Option<T>`. Usa `if let Some(cliente_atendido) = ... { ... } else { ... }`.
        * Si se atendió a un cliente, imprime un mensaje como: `"Se atendió a: [Nombre del Cliente]"`.
        * Si no había clientes, imprime: `"No había clientes para atender."`.
    * Imprime el estado actual de la `fila_clientes` después de esta operación.

**Tarea Opcional 4.2: Atender a otro cliente**
* **En el archivo `lab_queue_simulator.rs`:**
    * Esta tarea es para practicar más. Repite la lógica de la Tarea 4.1 para atender a un **segundo cliente** (si lo hay en la fila).
    * Imprime el mensaje correspondiente y el estado de la fila después de esta segunda atención.

---

#### **Parte 5: ¡La Fila Termina! - Vaciando la Cola**

**Tarea 5.1: Atender a todos los clientes**
* **En el archivo `lab_queue_simulator.rs`:**
    * Queremos que la panadera atienda a *todos* los clientes que queden en la fila hasta que esta esté completamente vacía.
    * **Usa un bucle `while let Some(cliente_atendido_final) = ... { ... }`** para seguir sacando clientes mientras la cola no esté vacía.
    * Dentro del bucle, por cada cliente que se atienda, imprime un mensaje como: `"Se atendió al último cliente: [Nombre del Cliente]"`.
    * Al terminar el bucle, la cola debería estar vacía. La línea `println!("\n¿La fila está completamente vacía ahora? {}", fila_clientes.is_empty());` te ayudará a verificar esto.

---

## Preguntas para Reflexionar

Estas preguntas son para que pienses un poco más allá del código y refuerces tu comprensión. No necesitas escribir las respuestas en el archivo `lab_queue_simulator.rs`, pero puedes discutirlas con tus compañeros o con tu profesor.

1.  **Mutabilidad:**