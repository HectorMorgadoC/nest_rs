pub(crate) mod repository {
    use super::super::models::model::dto::Team;
    use futures_util::TryStreamExt;
    use mongodb::{
        Database,
        bson::{doc, oid::ObjectId},
        error::Error,
        results::{DeleteResult, InsertOneResult, UpdateResult},
    };

    #[derive(Clone)]
    pub(crate) struct Repository {
        connection: Database,
    }

    impl Repository {
        pub fn new(connection: Database) -> Self {
            Self { connection }
        }

        pub async fn create(&self, dto: Team) -> Result<InsertOneResult, Error> {
            let collection = self.connection.collection::<Team>("team");

            collection.insert_one(dto).await
        }

        pub async fn find_all(&self) -> Result<Vec<Team>, Error> {
            let collection = self.connection.collection::<Team>("team");
            let mut teams: Vec<Team> = vec![];

            let mut cursor = collection.find(doc! {}).await?;

            while let Some(doc) = cursor.try_next().await? {
                teams.push(doc);
            }

            Ok(teams)
        }

        pub async fn find_name(&self, param: String) -> Result<Vec<Team>, Error> {
            let collection = self.connection.collection::<Team>("team");
            let mut teams: Vec<Team> = vec![];

            let mut cursor = collection.find(doc! {"name": param}).await?;

            while let Some(team) = cursor.try_next().await? {
                teams.push(team);
            }

            Ok(teams)
        }

        pub async fn update(&self, dto: Team, object_id: ObjectId) -> Result<UpdateResult, Error> {
            let collection = self.connection.collection::<Team>("team");
            let filter = doc! { "_id":object_id };
            let price: u32 = dto.price as u32;
            let update_record = doc! {"$set": doc! {"price": price, "name": dto.name } };

            collection.update_many(filter, update_record).await
        }

        pub async fn delete(&self, object_id: ObjectId) -> Result<DeleteResult, Error> {
            let collection = self.connection.collection::<Team>("team");
            let filter = doc! {"$and": [ doc! {"_id": object_id } ]};

            collection.delete_one(filter).await
        }
    }
}
