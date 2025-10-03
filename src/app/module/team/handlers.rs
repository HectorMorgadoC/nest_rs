pub mod handler {

    use super::super::{models::model::dto::Team, services::service::Service as Services};

    use crate::app::shared::{
        api::api_request::api_request::DataRequest, common::http_error::http_error::HttpResult,
        validation::validation::validation_request::ValidatedRequest,
    };

    use actix_web::{
        HttpResponse,
        web::{Data, Path},
    };

    pub async fn get_all_team(service: Data<Services>) -> HttpResult<HttpResponse> {
        let teams = service.get().await?;

        Ok(HttpResponse::Ok().json(teams))
    }

    pub async fn get_find_by_name_team(
        service: Data<Services>,
        id: Path<String>,
    ) -> HttpResult<HttpResponse> {
        let teams = service.get_find_by_name(id.into_inner()).await?;

        Ok(HttpResponse::Ok().json(teams))
    }

    pub async fn create_team(
        service: Data<Services>,
        ValidatedRequest(dto): ValidatedRequest<DataRequest<Team>>,
    ) -> HttpResult<HttpResponse> {
        service.create(dto.into_inner()).await?;

        Ok(HttpResponse::Ok().finish())
    }

    pub async fn update_team(
        service: Data<Services>,
        ValidatedRequest(dto): ValidatedRequest<DataRequest<Team>>,
        id: Path<String>,
    ) -> HttpResult<HttpResponse> {
        service.update(dto.into_inner(), id.into_inner()).await?;

        Ok(HttpResponse::Ok().finish())
    }

    pub async fn delete_team(
        service: Data<Services>,
        id: Path<String>,
    ) -> HttpResult<HttpResponse> {
        service.delete(id.into_inner()).await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
