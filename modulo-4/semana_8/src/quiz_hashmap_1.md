# Semana 8 — Quiz: HashMap I (Rust)

**Instrucciones**: Mezcla de opción múltiple, verdadero/falso y completar código. Marca tus respuestas y explica brevemente el porqué cuando corresponda.

---

## 1) (Opción múltiple)
`HashMap<K, V>` requiere que el tipo `K` implemente:
A) `Clone` y `Copy`  
B) `Hash` y `Eq`  
C) `Ord` y `PartialOrd`  
D) `Default` y `FromStr`

## 2) (Verdadero/Falso)
La complejidad promedio de `insert`, `get` y `remove` en `HashMap` es O(n).  
**R/**

## 3) (Opción múltiple)
¿Cuál es la firma más precisa de `get`?
A) `fn get(self, k: K) -> Option<V>`  
B) `fn get(&self, k: &K) -> Option<&V>`  
C) `fn get(&mut self, k: &K) -> Option<&mut V>`  
D) `fn get(self, k: &K) -> V`

## 4) (Completar código)
Completa para incrementar en 1 el contador de la clave `"rust"` sin hacer `get` previo:
```rust
let mut m: HashMap<String, usize> = HashMap::new();
___________________________________________;
```

## 5) (Verdadero/Falso)
`insert(k, v)` devuelve `bool` indicando si se insertó una nueva pareja y no se actualizó.  
**R/**

## 6) (Opción múltiple)
¿Cuál de las siguientes afirmaciones sobre colisiones es correcta?
A) Nunca ocurren si la función hash es perfecta.  
B) Pueden ocurrir; el manejo de colisiones mantiene la complejidad amortizada O(1).  
C) Siempre degradan a O(n^2).  
D) Se evitan ordenando las claves.

## 7) (Completar código)
Elimina la clave `"alice"` de `m` y guarda el valor eliminado en `v` (si existe):
```rust
let v = ____________________________;
```

## 8) (Identificación de error)
¿Qué está mal en este código?
```rust
let mut m: HashMap<String, i32> = HashMap::new();
let key = "k";
m.insert(key, 1);
```
Explica y corrige.

## 9) (Opción múltiple)
Para reducir rehashes cuando conocemos el tamaño aproximado, podemos:
A) Usar `with_capacity(n)` o `reserve(n)`  
B) Usar `shrink_to_fit()`  
C) Usar `Vec::with_capacity(n)`  
D) No hay forma

## 10) (Completar código breve)
Ordena `Vec<(String, usize)> pares` por frecuencia descendente y luego palabra ascendente:
```rust
pares.sort_by(|a, b| {
    __________________________________________
});
```

## 11) (Verdadero/Falso)
En un peor caso extremo (muchas colisiones adversarias), las operaciones de `HashMap` pueden degradar a O(n).  
**R/**

## 12) (Pregunta breve)
Menciona dos usos de `HashMap` en una aplicación tipo Discord.
