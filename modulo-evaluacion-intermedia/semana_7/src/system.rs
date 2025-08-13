// Filename: src/system.rs
//
// Este archivo contiene la lógica principal del sistema, encapsulada en el
// `struct` `SistemaReciclaje`.
use std::collections::HashMap;
use crate::models::{Material, EntradaReciclaje, PuntoReciclaje};

/// Modela el sistema de gestión de reciclaje completo, encapsulando los datos y la lógica.
pub struct SistemaReciclaje {
    puntos_reciclaje: HashMap<u32, PuntoReciclaje>,
}

impl SistemaReciclaje {
    /// Crea una nueva instancia del sistema de reciclaje.
    pub fn nuevo() -> Self {
        SistemaReciclaje {
            puntos_reciclaje: HashMap::new(),
        }
    }

    /// Procesa una serie de entradas de reciclaje desde una cadena de texto.
    ///
    /// # Argumentos
    /// * `cadena_datos`: Una cadena de texto que contiene los datos de recolección.
    pub fn procesar_datos(&mut self, cadena_datos: &str) {
        for linea in cadena_datos.lines() {
            let partes: Vec<&str> = linea.trim().split(',').collect();
            if partes.len() != 3 {
                println!("Advertencia: Línea con formato incorrecto, saltando: {}", linea);
                continue;
            }

            let id_punto = match partes[0].parse::<u32>() {
                Ok(id) => id,
                Err(_) => {
                    println!("Error al parsear el ID, saltando: {}", linea);
                    continue;
                }
            };
            let material = match Material::desde_cadena(partes[1]) {
                Some(mat) => mat,
                None => {
                    println!("Material no reconocido, saltando: {}", linea);
                    continue;
                }
            };
            let peso_kg = match partes[2].parse::<f64>() {
                Ok(peso) => peso,
                Err(_) => {
                    println!("Error al parsear el peso, saltando: {}", linea);
                    continue;
                }
            };

            let entrada = EntradaReciclaje {
                id_punto,
                material,
                peso_kg,
            };

            let punto = self.puntos_reciclaje.entry(entrada.id_punto).or_insert_with(|| PuntoReciclaje::nuevo(entrada.id_punto));
            punto.agregar_peso(entrada.material, entrada.peso_kg);
        }
    }

    /// Genera y muestra un reporte consolidado de los datos procesados.
    pub fn generar_reporte(&self) {
        println!("--- REPORTE DE RECICLAJE ---");
        println!("Total de puntos de recolección procesados: {}", self.puntos_reciclaje.len());
        println!("---------------------------");

        let mut total_reciclado_por_material: HashMap<Material, f64> = HashMap::new();
        let mut id_punto_mas_eficiente = 0;
        let mut peso_maximo = 0.0;

        for (id_punto, punto) in &self.puntos_reciclaje {
            let peso_total_del_punto = punto.obtener_peso_total();
            println!("Punto de Recolección ID {}:", punto.id);

            for (material, peso) in &punto.peso_total_reciclado {
                println!("\t- {}: {:.2} kg", material.como_cadena(), peso);
                let total = total_reciclado_por_material.entry(*material).or_insert(0.0);
                *total += peso;
            }
            println!("\tTotal Recolectado en este punto: {:.2} kg\n", peso_total_del_punto);

            if peso_total_del_punto > peso_maximo {
                peso_maximo = peso_total_del_punto;
                id_punto_mas_eficiente = *id_punto;
            }
        }

        println!("--- RESUMEN POR MATERIAL ---");
        for (material, peso_total) in total_reciclado_por_material {
            println!("Total de {}: {:.2} kg", material.como_cadena(), peso_total);
        }
        println!("---------------------------");

        if id_punto_mas_eficiente != 0 {
            println!("\nEl Punto de Recolección más eficiente es el ID {} con {:.2} kg.", id_punto_mas_eficiente, peso_maximo);
        } else {
            println!("\nNo se pudo determinar el punto más eficiente.");
        }
    }
}