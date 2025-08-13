# Rust Olympics - Semana de Proyecto

## Bienvenida

¡Felicidades, campeones! Han llegado a la mitad del curso y es hora de poner a prueba sus habilidades en un reto que no solo validará su conocimiento, sino que también tendrá un impacto social real.

El objetivo de esta semana es claro: aplicar todo lo aprendido sobre ownership, structs, enums, vectores y funciones para resolver un problema que beneficie a la comunidad.

## El Desafío: Sistema de Gestión de Residuos Reciclables

La Municipalidad de su ciudad ha lanzado un programa piloto para mejorar la gestión de residuos. Han recolectado datos de diferentes puntos de reciclaje, pero necesitan un programa que procese y analice esta información de forma eficiente.

### Tu misión es crear un programa en Rust que pueda:

- Modelar los datos: Representar los diferentes tipos de residuos y los puntos de recolección utilizando `structs` y `enums`.
- Utilizar principios de Programación Orientada a Objetos: Modelar el comportamiento de los datos con `impl` blocks para un diseño limpio y modular.
- Procesar la información: Recibir datos de recolección de residuos y almacenarlos en un `vector` de forma segura, respetando las reglas de ownership.
- Generar un reporte: Calcular la cantidad total de cada tipo de material reciclado e identificar el punto de recolección más eficiente.

## Requerimientos Técnicos

Este proyecto debe iniciarse desde cero. No se proporciona código base.

### Datos de Entrada

El programa recibirá una serie de datos en formato `String` con la siguiente estructura:

```
punto_id,material,peso_kg
```

#### Ejemplo de entrada:

```
101,plastico,50.5
102,vidrio,25.0
101,papel,15.2
103,plastico,30.0
```

## Entregables y Criterios de Evaluación

Tu solución será evaluada con base en los siguientes aspectos:

- Funcionalidad: El programa debe compilar y procesar correctamente los datos de entrada, produciendo el reporte solicitado.
- Organización del Código: Se valorará el uso adecuado de `structs`, `enums` y `impl blocks` para estructurar el programa.
- Integridad de Datos: Tu código debe demostrar un manejo seguro de los datos, sin errores de ownership o borrowing.
- Limpieza del Código: El código debe estar formateado con `rustfmt` y contener comentarios de documentación `///` claros.

## Potencial para tu Portafolio Profesional

Este reto representa una oportunidad valiosa. Un sistema de gestión de datos como este es un problema real en muchas industrias. Al resolverlo, demuestras tu capacidad para:

- Analizar un problema real y dividirlo en partes manejables.
- Diseñar estructuras de datos y comportamientos adecuados.
- Crear código robusto, seguro y profesional que puede escalarse en el futuro.

## Conclusión

Prepárate. El reloj comienza ahora.

¡Que gane el mejor equipo!
