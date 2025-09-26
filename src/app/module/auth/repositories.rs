pub(crate) mod repository {
    use diesel::prelude::*;
    use diesel::result::Error as error;

    use crate::app::{
        module::auth::models::model::dto::User, shared::database::diesel::diesel::DBPool,
    };

    use crate::schema::user_auth::dsl::*;

    #[derive(Clone)]
    pub struct Repository {
        connection: DBPool,
    }

    impl Repository {
        pub fn new(connection: DBPool) -> Self {
            Self { connection }
        }

        pub fn create(&self, _dto: &User) -> Result<usize, error> {
            let mut connection = self.connection.get().map_err(|e| {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string()),
                )
            })?;
            diesel::insert_into(user_auth)
                .values(_dto)
                .execute(&mut connection)
        }

        pub fn get_by_email(&self, _email: String) -> Result<Vec<User>, error> {
            let mut connection = self.connection.get().map_err(|e| {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string()),
                )
            })?;
            let result = user_auth
                .filter(email.eq(_email))
                .select(User::as_select())
                .load(&mut connection);

            result
        }
    }
}
