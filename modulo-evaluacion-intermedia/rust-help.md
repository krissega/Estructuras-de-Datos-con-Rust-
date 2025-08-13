# Guía de Convenciones de Código para Rust

## Introducción

Esta guía establece un conjunto de convenciones de estilo y buenas prácticas para el desarrollo en el lenguaje de programación Rust. El objetivo es mantener un código limpio, coherente y fácil de mantener dentro del equipo de trabajo o en proyectos individuales.

## Convenciones Generales

### Nombres

- Usa `snake_case` para nombres de variables, funciones y módulos.
    - Ejemplo: `calcular_promedio`, `numero_total`
- Usa `PascalCase` para nombres de tipos, structs, enums y traits.
    - Ejemplo: `Cliente`, `FacturaElectronica`
- Usa `SCREAMING_SNAKE_CASE` para constantes.
    - Ejemplo: `MAX_REINTENTOS`

### Comentarios

- Usa comentarios `//` para explicar secciones del código cuando sea necesario.
- Mantén los comentarios concisos y útiles.
- Evita comentar código obsoleto o innecesario.

### Formato

- Usa indentación de 4 espacios.
- Usa líneas en blanco para separar bloques lógicos de código.
- Limita el ancho de línea a 100 caracteres cuando sea posible.
- El archivo debe terminar con una línea en blanco.

## Organización del Código

- Agrupa las funciones relacionadas juntas.
- Coloca los `use` al inicio del archivo, ordenados alfabéticamente.
- Separa las declaraciones de `struct`, `impl`, `enum` y `mod` con una línea en blanco.

## Manejo de Errores

- Prefiere `Result<T, E>` en lugar de `panic!` para errores esperados.
- Usa `unwrap()` y `expect()` solo cuando estés completamente seguro de que no habrá error.
- Implementa tipos de error personalizados si el dominio del problema lo requiere.

## Uso de Option y Result

- Usa `match` o `if let` para manejar valores de `Option` y `Result`.
- Evita patrones anidados complicados que dificulten la legibilidad.

## Pruebas

- Coloca las pruebas unitarias en un módulo `mod tests` al final del archivo fuente.
- Usa `#[cfg(test)]` para marcar el módulo de pruebas.
- Cubre tanto casos de éxito como de error.

## Documentación

- Documenta las funciones públicas con `///`.
- Describe qué hace la función, sus parámetros y el valor de retorno si aplica.

```rust
/// Calcula el promedio de una lista de enteros.
/// 
/// # Parámetros
/// - `numeros`: Un vector de enteros.
///
/// # Retorna
/// - El valor promedio como `f64`.
fn calcular_promedio(numeros: Vec<i32>) -> f64 {
    // ...
}