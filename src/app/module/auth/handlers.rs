pub(crate) mod handler {
    use actix_web::{HttpResponse, web};

    use crate::app::{
        module::auth::models::model::dto::User,
        shared::{
            api::api_request::api_request::DataRequest,
            validation::validation::validation_request::ValidatedRequest,
        },
    };

    use super::super::services::service::Service as Services;
    use crate::app::shared::common::http_error::http_error::HttpResult;
    pub async fn create_user(
        service: web::Data<Services>,
        ValidatedRequest(dto): ValidatedRequest<DataRequest<User>>,
    ) -> HttpResult<HttpResponse> {
        let user_auth = service.create(&dto.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user_auth))
    }
}
