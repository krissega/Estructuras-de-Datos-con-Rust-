# Semana 9 — Quiz: HashMap II (Diccionario, Colisiones y Uso Avanzado)

**Instrucciones:** contesta sin ayudas externas. Algunas preguntas tienen más de una respuesta correcta. Incluye justificación breve cuando se solicite.

---

## Parte A — Opción múltiple (selecciona una)
1) ¿Cuál es la causa directa de una colisión en una tabla hash?
- [ ] A. Que dos claves distintas tengan el mismo valor.
- [x] B. Que dos claves diferentes produzcan el mismo valor de hash.
- [ ] C. Que el factor de carga sea menor a 0.5.
- [ ] D. Que se use un `HashMap` en lugar de `BTreeMap`.

2) ¿Cuál estrategia describe mejor **open addressing**?
- [ ] A. Almacenar una lista de colisiones por bucket.
- [x] B. Buscar posiciones alternativas dentro del mismo arreglo hasta encontrar un hueco.
- [ ] C. Reasignar todos los elementos al azar.
- [ ] D. Crear un árbol balanceado para cada bucket.

3) En Rust, `HashMap::with_hasher(BuildZeroHasher)` afectará principalmente:
- [ ] A. La ordenación de claves al iterar.
- [x] B. La función de hash usada, pudiendo forzar colisiones.
- [ ] C. La semántica de `Clone` en valores.
- [ ] D. La cantidad de memoria de la pila (stack).

---

## Parte B — Verdadero/Falso
4) **V/F**: En `HashMap`, una mayor uniformidad del hash reduce la probabilidad de colisiones.  
> Respuesta: **V**

5) **V/F**: El `HashMap` de Rust expone públicamente sus buckets internos para inspección.  
> Respuesta: **F**

6) **V/F**: Las estrategias de **separate chaining** almacenan elementos colisionados fuera del arreglo principal.  
> Respuesta: **V**

---

## Parte C — Completa el código
7) Completa la función para **actualizar** una definición si existe o insertarla si no existe usando `HashMap<String, String>`:

```rust
use std::collections::HashMap;

fn upsert(dict: &mut HashMap<String, String>, k: String, v: String) {
    // TODO: completa en una línea usando la API idiomática
    // dict.___________
}
```

**Solución esperada (una posible):**
```rust
dict.insert(k, v);
```

8) Escribe una función que **liste** las entradas del diccionario ordenadas por clave:

```rust
fn list_sorted(dict: &std::collections::HashMap<String, String>) -> Vec<(String,String)> {
    let mut v: Vec<_> = dict.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        // TODO: ordena por clave
        .collect();
    v
}
```

**Solución esperada:**
```rust
let mut v: Vec<_> = dict.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
v.sort_by(|a, b| a.0.cmp(&b.0));
v
```

---

## Parte D — Teoría de colisiones
9) ¿Cuál(es) enunciado(s) es/son correcto(s)? Marca todo lo que aplique.
- [x] A. En separate chaining, cada bucket puede almacenar múltiples parejas (k, v).
- [ ] B. En open addressing, las colisiones se guardan en una lista ligada externa.
- [x] C. En **robin hood hashing**, los elementos con sondas largas pueden "robar" posiciones a otros con sondas más cortas.
- [x] D. A mayor factor de carga, mayor probabilidad de colisiones.

10) Supón un `BucketMap` con 8 buckets y hash `sum(bytes) % 8`. Da un ejemplo de dos claves distintas que colisionen y explica por qué.  
> **Respuesta libre:** p. ej., `"ab"` (97+98=195) y `"ba"` (98+97=195) ⇒ 195 % 8 = 3 en ambos casos.

---

## Parte E — Detección de errores
11) ¿Qué error conceptual ves en el siguiente fragmento?

```rust
let mut dict = std::collections::HashMap::new();
let v = dict.get("clave").unwrap(); // ...
```

- [x] A. Puede hacer `panic!` si la clave no existe; debe manejarse `Option` con `match` o `if let`.
- [ ] B. `HashMap` no soporta `get`.
- [ ] C. `unwrap` solo se puede usar con `Result`, no con `Option`.
- [ ] D. Ninguno.

12) ¿Qué problema potencial hay en este hash didáctico?
```rust
fn index_for(key: &str, cap: usize) -> usize {
    key.as_bytes().iter().map(|b| *b as usize).sum::<usize>() % cap
}
```
- [x] A. Pobre distribución: claves anagramas colisionan con facilidad.
- [ ] B. Es criptográficamente fuerte.
- [ ] C. No compila.
- [ ] D. Consume memoria lineal.

---

¡Feliz aprendizaje rustaceos!
