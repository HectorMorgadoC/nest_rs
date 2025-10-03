pub mod mongodb_error {
    use super::super::super::http_error::http_error::ProblemDetails;
    use mongodb::error::{Error as MongoError, ErrorKind, WriteFailure};
    pub fn handle_mongodb_error(err: MongoError) -> ProblemDetails {
        println!("MongoDB error kind: {:?}", err.kind);

        match err.kind.as_ref() {
            // Errores de escritura (duplicados, validación, etc.)
            ErrorKind::Write(write_failure) => match write_failure {
                WriteFailure::WriteError(write_err) => {
                    let code = write_err.code;
                    let message = write_err.message.clone();
                    println!("MongoDB write error [{code}]: {message}");

                    match code {
                        // Duplicate key error
                        11000 | 11001 => ProblemDetails::conflict(message),

                        // Document validation failed
                        121 => ProblemDetails::bad_request(message),

                        // Immutable field
                        66 => ProblemDetails::bad_request(message),

                        // Cannot apply operation
                        16837 => ProblemDetails::bad_request(message),

                        // Invalid field name
                        52 | 56 => ProblemDetails::bad_request(message),

                        _ => ProblemDetails::server_error(),
                    }
                }
                WriteFailure::WriteConcernError(wc_err) => {
                    println!(
                        "MongoDB write concern error [{}]: {}",
                        wc_err.code, wc_err.message
                    );
                    ProblemDetails::service_unavailable()
                }
                _ => ProblemDetails::server_error(),
            },

            // Errores de comandos
            ErrorKind::Command(cmd_err) => {
                let code = cmd_err.code;
                let message = cmd_err.message.clone();
                println!("MongoDB command error [{code}]: {message}");

                match code {
                    // Namespace errors
                    26 => ProblemDetails::server_error(), // NamespaceNotFound
                    48 => ProblemDetails::server_error(), // NamespaceExists

                    // Authentication errors
                    11 | 18 | 33 => ProblemDetails::service_unavailable(),

                    // Authorization errors
                    13 => ProblemDetails::service_unavailable(),

                    // Index errors
                    85 | 86 | 68 => ProblemDetails::conflict(message),

                    // Invalid argument
                    2 | 9 | 14 | 28 => ProblemDetails::bad_request(message),

                    // Timeout
                    50 => ProblemDetails::service_unavailable(),

                    // Cursor errors
                    43 => ProblemDetails::bad_request("Cursor not found".to_string()),

                    // Transaction errors
                    225 | 251 | 256 | 257 | 263 => ProblemDetails::service_unavailable(),

                    _ => ProblemDetails::server_error(),
                }
            }

            // Errores de bulk write
            ErrorKind::BulkWrite(bulk_err) => {
                if !bulk_err.write_errors.is_empty() {
                    // Obtener el primer error del HashMap
                    if let Some((_, first_err)) = bulk_err.write_errors.iter().next() {
                        println!(
                            "MongoDB bulk write error [{}]: {}",
                            first_err.code, first_err.message
                        );

                        match first_err.code {
                            11000 | 11001 => ProblemDetails::conflict(first_err.message.clone()),
                            121 => ProblemDetails::bad_request(first_err.message.clone()),
                            _ => ProblemDetails::server_error(),
                        }
                    } else {
                        ProblemDetails::server_error()
                    }
                } else {
                    ProblemDetails::server_error()
                }
            }

            // Errores de autenticación
            ErrorKind::Authentication { .. } => {
                println!("MongoDB authentication error: {err}");
                ProblemDetails::service_unavailable()
            }

            // Errores de conexión
            ErrorKind::Io(_) => {
                println!("MongoDB I/O error: {err}");
                ProblemDetails::service_unavailable()
            }

            // Errores de timeout de conexión
            ErrorKind::ConnectionPoolCleared { .. } => {
                println!("MongoDB connection pool cleared: {err}");
                ProblemDetails::service_unavailable()
            }

            // Errores de selección de servidor
            ErrorKind::ServerSelection { .. } => {
                println!("MongoDB server selection error: {err}");
                ProblemDetails::service_unavailable()
            }

            // Operación no soportada en transacción
            ErrorKind::Transaction { .. } => {
                println!("MongoDB transaction error: {err}");
                ProblemDetails::bad_request("Transaction error".to_string())
            }

            // Errores de deserialización (datos corruptos o inválidos)
            ErrorKind::InvalidResponse { .. } => {
                println!("MongoDB invalid response: {err}");
                ProblemDetails::server_error()
            }

            // Errores BSON
            ErrorKind::BsonDeserialization(_) | ErrorKind::BsonSerialization(_) => {
                println!("MongoDB BSON error: {err}");
                ProblemDetails::bad_request("Invalid data format".to_string())
            }

            // Errores internos
            ErrorKind::Internal { .. } => {
                println!("MongoDB internal error: {err}");
                ProblemDetails::server_error()
            }

            // Default para cualquier otro error
            _ => {
                println!("Unexpected MongoDB error: {err}");
                ProblemDetails::server_error()
            }
        }
    }
}
