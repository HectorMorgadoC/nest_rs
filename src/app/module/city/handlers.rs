pub mod handler {

    use super::super::services::service::Service as Services;
    use crate::app::{
        module::city::models::model::City,
        shared::{
            api::api_request::api_request::DataRequest,
            common::{
                http_error::http_error::HttpResult, pagination_dto::pagination_dto::Pagination,
            },
            validation::validation::validation_request::ValidatedRequest,
        },
    };
    use actix_web::{
        HttpResponse,
        web::{Data, Path, Query},
    };

    pub async fn get_all_city(service: Data<Services>) -> HttpResult<HttpResponse> {
        let cities = service.get().await?;

        Ok(HttpResponse::Ok().json(cities))
    }

    pub async fn get_pagination_city(
        service: Data<Services>,
        query: Query<Pagination>,
    ) -> HttpResult<HttpResponse> {
        let cities = service.get_pagination(query.into_inner()).await?;

        Ok(HttpResponse::Ok().json(cities))
    }

    pub async fn get_by_id_city(
        service: Data<Services>,
        id: Path<i32>,
    ) -> HttpResult<HttpResponse> {
        let city = service.get_find_by_id(id.into_inner()).await?;

        Ok(HttpResponse::Ok().json(city))
    }

    pub async fn create_city(
        service: Data<Services>,
        ValidatedRequest(dto): ValidatedRequest<DataRequest<City>>,
    ) -> HttpResult<HttpResponse> {
        let city = service.create(dto.into_inner()).await?;

        Ok(HttpResponse::Ok().json(city))
    }

    pub async fn update_city(
        service: Data<Services>,
        ValidatedRequest(dto): ValidatedRequest<DataRequest<City>>,
        id: Path<i32>,
    ) -> HttpResult<HttpResponse> {
        let city = service.update(dto.into_inner(), id.into_inner()).await?;

        Ok(HttpResponse::Ok().json(city))
    }

    pub async fn delete_city(service: Data<Services>, id: Path<i32>) -> HttpResult<HttpResponse> {
        service.delete(id.into_inner()).await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
