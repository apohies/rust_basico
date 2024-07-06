use std::process::Command;
use std::fs;

fn main() {
    // Define la ruta al directorio donde se encuentra el script de Python
    let path_al_script = "/Users/apohies/Documents/projectrust/rust_basico/src/bin";

    // Ejecutar el script de Python en el directorio especificado y esperar a que termine
    let output = Command::new("python3")
        .arg("generate.py") // Asegúrate de que este sea el nombre correcto del script
        .current_dir(path_al_script) // Establece el directorio de trabajo
        .output()
        .expect("Fallo al ejecutar el script de Python");

    // Verificar si el script se ejecutó exitosamente
    if output.status.success() {
        // Construye la ruta al archivo JSON generado por el script de Python
        let json_path = format!("{}/persona.json", path_al_script);

        // Leer el archivo JSON
        let json = fs::read_to_string(&json_path)
            .expect("Error al leer el archivo JSON");

        // Aquí puedes procesar el JSON como desees
        println!("Archivo JSON: {}", json);
    } else {
        // Manejar el caso en que el script de Python falla
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Error al ejecutar el script de Python: {}", error_message);
    }
}
