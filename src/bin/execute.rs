use std::env;
use std::process::Command;
use std::path::PathBuf;


fn main() {

    let exe_path = "/Users/apohies/Documents/projectrust/rust_basico/src/bin";

    let output = Command::new("python3")
        .arg("generate.py")
        .current_dir(exe_path)
        .output()
        .expect("Fallo al ejecutar el script de Python");


    if output.status.success() {
        print!("se genero");
    }else{

        println!("La ruta del ejecutable es: {}", &exe_path);
    }
}

