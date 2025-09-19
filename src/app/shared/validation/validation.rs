pub mod validation_request {
    use crate::app::shared::common::http_error::http_error::ProblemDetails;
    use actix_web::{FromRequest, HttpRequest, web::Json};
    use serde::de::DeserializeOwned;
    use std::future::Future;
    use std::pin::Pin;
    use validator::Validate;

    /// Generic wrapper for validated HTTP requests
    #[derive(Debug)]
    pub struct ValidatedRequest<T>(pub T);

    impl<T> ValidatedRequest<T> {
        // Extract the inner validated value
        //pub fn into_inner(self) -> T {
        //    self.0
        //}
    }

    impl<T> std::ops::Deref for ValidatedRequest<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> FromRequest for ValidatedRequest<T>
    where
        T: DeserializeOwned + Validate + 'static,
    {
        type Error = ProblemDetails;
        type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

        fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
            println!("{:?}", req.clone());
            let json_future = Json::<T>::from_request(req, payload);
            //let uri = req.uri().to_string();
            Box::pin(async move {
                // Extract JSON payload
                let json_result = json_future.await;

                let data = match json_result {
                    Ok(json) => json.into_inner(),
                    Err(e) => {
                        return Err(ProblemDetails::bad_request(format!(
                            "JSON parsing error: {}",
                            e
                        )));
                    }
                };

                // Validate the data
                match data.validate() {
                    Ok(()) => Ok(ValidatedRequest(data)),
                    Err(validation_errors) => Err(ProblemDetails::unprocesable_entity(
                        validation_errors.to_string(),
                    )),
                }
            })
        }
    }

    // Type aliases for common use cases
    //pub type ValidatedProduct = ValidatedRequest<Product>;
    //pub type ValidatedUser = ValidatedRequest<User>;
    // Add more aliases as needed
}
