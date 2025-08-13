# Semana 9 — HashMap II  
**Laboratorio:** Diccionario práctico con HashMap y demostración de colisiones

## Requisitos cumplidos
- CRUD de definiciones (agregar, buscar, eliminar, listar) en consola.
- Uso de `std::collections::HashMap`.
- Ejemplo didáctico de colisiones (construcción de un pequeño mapa con *separate chaining*).
- Código comentado paso a paso y probado en Rust estable (sin *crates* externos).

## Cómo ejecutar
1. Guarda este archivo como `lab_diccionario.rs`.
2. Ejecuta:
   ```bash
   rustc lab_diccionario.rs && ./lab_diccionario
   ```

## Sugerencia pedagógica
Este archivo ya es funcional. Verás marcadores `TODO` con ideas para ampliar o modificar.  
La solución ultra-comentada está en `lab_semana9_solucion.rs`.

---

## Nota sobre la demostración de colisiones
Se incluye una implementación de un **Hasher** que siempre devuelve `0`, lo que provoca que **todas las claves caigan en el mismo "bucket" interno** del `HashMap`.  
Esto es útil para observar (empíricamente) la degradación del rendimiento en casos extremos.
