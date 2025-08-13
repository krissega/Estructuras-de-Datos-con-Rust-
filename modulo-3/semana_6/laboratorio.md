#  Laboratorio: Módulo de Deshacer/Rehacer para Facturación

##  Objetivo

Implementar un sistema de **"deshacer"** y **"rehacer"** para transacciones de facturación, utilizando **listas enlazadas simples** como la estructura de datos subyacente. El enfoque principal es aplicar correctamente los conceptos de `ownership`, `move` y `borrowing` de Rust para garantizar la integridad de los datos.



##  Descripción del Problema

Usted ha sido seleccionado para resolver un problema crítico en el módulo de facturación de la empresa **"Los Patitos"**.  
La funcionalidad de **"deshacer"** y **"rehacer"** transacciones está defectuosa, causando inconsistencias en los registros.

El análisis del código heredado reveló que el problema principal radica en la **gestión incorrecta del `ownership`**.  
La manipulación de las acciones del usuario entre el estado actual y las pilas de `undo` y `redo` está rompiendo las reglas de propiedad y préstamo de Rust, lo que lleva a **pérdidas de datos y un estado impredecible**.

Su misión es **refactorizar y re-implementar esta funcionalidad desde cero**.  
Deberá aplicar de forma correcta los conceptos de `ownership`, `move` y `borrowing` para asegurar que las acciones se transfieren de forma segura entre las pilas sin romper las reglas del compilador.



## Tareas a Completar

1. Implementar el método `perform_action()` para registrar una nueva acción.
2. Implementar el método `undo()` para revertir la última acción.
3. Implementar el método `redo()` para re-aplicar una acción deshecha.