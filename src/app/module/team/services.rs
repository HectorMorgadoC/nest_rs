pub(crate) mod service {

    use crate::app::shared::common::http_error::http_error::ProblemDetails;

    use super::super::models::model::dto::Team;
    use super::super::repositories::repository::Repository;
    use crate::app::shared::common::databases_error::mongo::mongodb_error::handle_mongodb_error;
    use mongodb::bson::oid::ObjectId;

    pub(crate) struct Service {
        repository: Repository,
    }

    impl Service {
        pub(crate) fn new(repository: Repository) -> Self {
            Self { repository }
        }

        pub async fn create(&self, dto: Team) -> Result<(), ProblemDetails> {
            let team_registration = self
                .repository
                .create(dto.clone())
                .await
                .map_err(handle_mongodb_error)?;

            println!("Registration {team_registration:?}");
            Ok(())
        }

        pub async fn get(&self) -> Result<Vec<Team>, ProblemDetails> {
            let teams = self
                .repository
                .find_all()
                .await
                .map_err(handle_mongodb_error)?;

            Ok(teams)
        }

        pub async fn get_find_by_name(&self, param: String) -> Result<Vec<Team>, ProblemDetails> {
            let teams = self
                .repository
                .find_name(param)
                .await
                .map_err(handle_mongodb_error)?;

            Ok(teams)
        }

        pub async fn update(&self, dto: Team, param: String) -> Result<(), ProblemDetails> {
            let object_id = ObjectId::parse_str(param)
                .map_err(|err| ProblemDetails::unprocesable_entity(err.to_string()))?;

            let update_record = self
                .repository
                .update(dto, object_id)
                .await
                .map_err(handle_mongodb_error)?;

            println!("Updated register: {update_record:?}");

            Ok(())
        }

        pub async fn delete(&self, param: String) -> Result<(), ProblemDetails> {
            let object_id = ObjectId::parse_str(param)
                .map_err(|err| ProblemDetails::unprocesable_entity(err.to_string()))?;

            println!("{:?}", object_id);
            let delete_record = self
                .repository
                .delete(object_id)
                .await
                .map_err(handle_mongodb_error);

            println!("Register delete: {delete_record:?}");

            Ok(())
        }
    }
}
