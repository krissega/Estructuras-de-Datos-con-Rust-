// Filename: src/models.rs
//
// Este archivo define todas las estructuras de datos (`structs` y `enums`)
// del proyecto. Los marcamos como `pub` para que sean accesibles desde
// `main.rs` y `system.rs`.
use std::collections::HashMap;

/// Representa los tipos de materiales reciclables.
/// Usamos un `Enum` para garantizar que solo se acepten tipos válidos.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Material {
    Plastico,
    Vidrio,
    Papel,
    Metal,
    Organico,
}

impl Material {
    /// Convierte una cadena de texto en una variante de `Material`.
    ///
    /// # Argumentos
    /// * `s`: La cadena de texto a convertir.
    ///
    /// # Retorno
    /// Un `Option<Material>` que será `Some` si la conversión es exitosa,
    /// o `None` si el material no es reconocido.
    pub fn desde_cadena(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "plastico" => Some(Material::Plastico),
            "vidrio" => Some(Material::Vidrio),
            "papel" => Some(Material::Papel),
            "metal" => Some(Material::Metal),
            "organico" => Some(Material::Organico),
            _ => None,
        }
    }

    /// Retorna una representación del material como `String`.
    pub fn como_cadena(&self) -> String {
        match self {
            Material::Plastico => "Plástico".to_string(),
            Material::Vidrio => "Vidrio".to_string(),
            Material::Papel => "Papel".to_string(),
            Material::Metal => "Metal".to_string(),
            Material::Organico => "Orgánico".to_string(),
        }
    }
}

/// Representa una entrada de recolección de residuos.
/// Contiene el ID del punto de recolección, el material y el peso.
pub struct EntradaReciclaje {
    pub id_punto: u32,
    pub material: Material,
    pub peso_kg: f64,
}

/// Representa un punto de recolección de residuos, con el total de material recolectado.
/// Se utiliza para el reporte final.
pub struct PuntoReciclaje {
    pub id: u32,
    pub peso_total_reciclado: HashMap<Material, f64>,
}

impl PuntoReciclaje {
    /// Crea un nuevo punto de recolección vacío.
    ///
    /// # Argumentos
    /// * `id`: El ID único del punto de recolección.
    pub fn nuevo(id: u32) -> Self {
        PuntoReciclaje {
            id,
            peso_total_reciclado: HashMap::new(),
        }
    }

    /// Añade un peso de material a la cuenta total del punto.
    ///
    /// # Argumentos
    /// * `material`: El tipo de material.
    /// * `peso`: El peso del material en kg.
    pub fn agregar_peso(&mut self, material: Material, peso: f64) {
        let total = self.peso_total_reciclado.entry(material).or_insert(0.0);
        *total += peso;
    }

    /// Retorna el peso total de todos los materiales reciclados en este punto.
    pub fn obtener_peso_total(&self) -> f64 {
        self.peso_total_reciclado.values().sum()
    }
}