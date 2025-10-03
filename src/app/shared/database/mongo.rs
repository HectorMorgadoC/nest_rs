pub mod mongo_db {
    use mongodb::{Client, Database, options::ClientOptions};

    pub async fn connection(url: String, _client: &str) -> Database {
        // contiene todas las opciones de configuración para conectarse a MongoDB
        let client_options = ClientOptions::parse(url)
            .await
            .unwrap_or_else(|err| panic!("Error connection configure: {err}"));

        // El cliente de MongoDB que gestiona la conexión con el servidor_ Establece el pool de conexiones
        let client = Client::with_options(client_options)
            .unwrap_or_else(|err| panic!("Error managing mongodb client: {err}"));

        // Una referencia a una base de datos específica dentro del servidor MongoDB
        client.database(_client)
    }
}
