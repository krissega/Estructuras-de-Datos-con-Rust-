#  Tipos de Datos Básicos en Rust

Rust es un lenguaje fuertemente tipado con control explícito sobre el tamaño y signo de los datos.
Esta guía resume los tipos primitivos más comunes y sus equivalentes en otros lenguajes como Java o C++.

## 🔢 Tipos Numéricos

| Concepto          | Rust     | Descripción                                 | Equivalente (Java / C / C++)     |
|-------------------|----------|---------------------------------------------|-----------------------------------|
| Entero con signo  | `i8`     | Entero de 8 bits con signo                  | `byte` / `char` (C)               |
|                   | `i16`    | Entero de 16 bits con signo                 | `short`                           |
|                   | `i32`    | Entero de 32 bits con signo (por defecto)   | `int`                             |
|                   | `i64`    | Entero de 64 bits con signo                 | `long` / `long long`             |
|                   | `i128`   | Entero de 128 bits con signo                | No tiene equivalente directo      |
| Entero sin signo  | `u8`     | Entero de 8 bits sin signo                  | `unsigned char`                   |
|                   | `u16`    | Entero de 16 bits sin signo                 | `unsigned short`                  |
|                   | `u32`    | Entero de 32 bits sin signo                 | `unsigned int`                    |
|                   | `u64`    | Entero de 64 bits sin signo                 | `unsigned long`                   |
|                   | `u128`   | Entero de 128 bits sin signo                | No tiene equivalente directo      |
| Tamaño de puntero | `isize`  | Entero con signo del tamaño del sistema     | `intptr_t`                        |
|                   | `usize`  | Entero sin signo del tamaño del sistema     | `size_t`                          |

##  Números de Punto Flotante

| Concepto              | Rust   | Descripción                           | Equivalente (Java / C) |
|-----------------------|--------|---------------------------------------|--------------------------|
| Decimal de 32 bits    | `f32`  | Precisión simple                      | `float`                  |
| Decimal de 64 bits    | `f64`  | Precisión doble (por defecto en Rust) | `double`                 |

## 🅰 Otros Tipos Primitivos

| Concepto     | Rust       | Descripción                                                    | Equivalente (Java / C / C++) |
|--------------|------------|----------------------------------------------------------------|-------------------------------|
| Booleano     | `bool`     | Valor lógico: `true` o `false`                                 | `boolean` / `_Bool`          |
| Carácter     | `char`     | Carácter Unicode de 4 bytes (`'a'`, `'ñ'`, `'😊'`)             | `char` (Java), `wchar_t`     |
| Texto        | `&str`     | Cadena de texto inmutable (slice)                             | `const char*`, `String` (Java) |
|              | `String`   | Cadena de texto heap-allocated (mutable)                      | `std::string`, `String`      |
| Arreglo fijo | `[T; N]`   | Arreglo con tamaño fijo                                       | `int[5]`, `String[]`         |
| Vector       | `Vec<T>`   | Arreglo dinámico en heap                                      | `ArrayList<T>` (Java)        |

##  Notas
- No hay **coerción implícita** entre tipos (ej: de `i32` a `f64`). Se debe hacer conversión explícita:
  ```rust
  let x: i32 = 5;
  let y: f64 = x as f64;
