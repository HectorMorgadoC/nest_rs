pub mod api_request {

    use serde::{Deserialize, Serialize};
    use validator::{Validate, ValidationError, ValidationErrors};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DataRequest<T> {
        pub r#type: String,
        pub atributes: T,
    }

    impl<T> DataRequest<T> {
        pub fn new(r#type: String, atributes: T) -> Self {
            Self { r#type, atributes }
        }

        pub fn into_inner(self) -> T {
            self.atributes
        }

        pub fn get_attributes(&self) -> &T {
            &self.atributes
        }

        pub fn get_type(&self) -> &str {
            &self.r#type
        }
    }

    impl<T> Validate for DataRequest<T>
    where
        T: Validate,
    {
        fn validate(&self) -> Result<(), ValidationErrors> {
            //self.atributes.validate(); // Llama a la validación existente de T
            let mut errors = ValidationErrors::new();

            // Validar que type no esté vacío
            if self.r#type.trim().is_empty() {
                let mut error = ValidationError::new("required");
                error.message = Some("Type field cannot be empty".into());
                errors.add("type", error);
            }

            // Validar longitud del type (1-50 caracteres)
            if self.r#type.len() < 1 || self.r#type.len() > 50 {
                let mut error = ValidationError::new("length");
                error.message = Some("Type field must be between 1 and 50 characters".into());
                errors.add("type", error);
            }

            // Validar que type solo contenga letras
            if !self.r#type.chars().all(|c| c.is_alphabetic()) {
                let mut error = ValidationError::new("invalid_format");
                error.message = Some("Type field can only contain letters".into());
                errors.add("type", error);
            }

            // Validar los atributes
            if let Err(attr_errors) = self.atributes.validate() {
                // Convertir los errores de atributes en un error simple
                let attr_error_msg = format!("{}", attr_errors);
                let mut error = ValidationError::new("invalid");
                error.message = Some(attr_error_msg.into());
                errors.add("atributes", error);
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }
    }
}
