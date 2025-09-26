pub(crate) mod handler {
    use actix_web::{HttpResponse, cookie::Cookie, http::header, web};

    use crate::app::{
        module::auth::models::model::{authorization::User as UserLogin, dto::User as UserDto},
        shared::{
            api::api_request::api_request::DataRequest,
            validation::validation::validation_request::ValidatedRequest,
        },
    };

    use super::super::services::service::Service as Services;
    use crate::app::shared::common::http_error::http_error::HttpResult;
    pub async fn create_user(
        service: web::Data<Services>,
        ValidatedRequest(dto): ValidatedRequest<DataRequest<UserDto>>,
    ) -> HttpResult<HttpResponse> {
        let user_auth = service.create(&dto.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user_auth))
    }

    pub async fn login(
        service: web::Data<Services>,
        ValidatedRequest(user_login): ValidatedRequest<DataRequest<UserLogin>>,
    ) -> HttpResult<HttpResponse> {
        let token = service.login(user_login.into_inner()).await?;

        let cookie = Cookie::build("access_token", token.clone())
            .path("/")
            .http_only(true)
            .secure(true) // usar true en producción (HTTPS)
            .same_site(actix_web::cookie::SameSite::Lax)
            .finish();

        Ok(HttpResponse::Ok().cookie(cookie).finish())
    }
}
