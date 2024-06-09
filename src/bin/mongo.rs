use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    nombre: String,
    descripcion: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = get_client().await?;
    
    let db = client.database("cratos");

    let collection :Collection<Product> = db.collection::<Product>("mundus");

    let product = Product {
        nombre: "Producto 1".to_string(),
        descripcion: "Descripcion del producto 1".to_string(),
    };

    let result = collection.insert_one(product, None).await?;

    Ok(())

}


// crear un metodo devuelva el cliente con la conexion a la base de datos y la coleccion
async fn get_client() -> Result<Client, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    
    Ok(client)
}


