// Filename: src/main.rs
// Filename: src/main.rs
//
// El archivo principal del programa. Se encarga de la entrada y salida de datos
// y de orquestar las llamadas a los diferentes módulos.
//
// Declaramos los módulos que usaremos. El compilador buscará los archivos
// `src/models.rs` y `src/system.rs`.
mod models;
mod system;

use crate::system::SistemaReciclaje;

fn main() {
    let data = "101,plastico,50.5\n\
                102,vidrio,25.0\n\
                101,papel,15.2\n\
                103,plastico,30.0\n\
                102,plastico,10.0\n\
                103,metal,5.0\n\
                101,vidrio,20.0\n\
                102,organico,3.5\n\
                103,papel,12.0";

    println!("Iniciando el procesamiento de datos de reciclaje...");
    let mut sistema = SistemaReciclaje::nuevo();
    sistema.procesar_datos(data);
    sistema.generar_reporte();
}

