use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use reaction::schema::reactions;
use user::schema::users;
use user::model::User;
use diesel::result::Error;
use chrono::{NaiveDate, NaiveDateTime};

#[table_name = "reactions"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Reaction, foreign_key = "post_id")]
#[primary_key(post_id)]
pub struct Reaction {
    pub reaction_id: Option<i32>,
    pub reaction_type: i32,
    pub author_id: Option<i32>,
    pub post_id: i32,
    pub created: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize)]
pub struct ReactionCountTypes {
    pub type1: i64,
    pub type2: i64,
    pub type3: i64,
}

impl Reaction {
    pub fn create(vettore: Reaction, connection: &MysqlConnection) -> bool {
        diesel::insert_into(reactions::table)
            .values(&vettore)
            .execute(connection)
            .is_ok()
    }

    pub fn count_reaction_type_to_post(post_id: i32, connection: &MysqlConnection) -> ReactionCountTypes {
        
        let mut query = reactions::table
            .filter(reactions::post_id.eq(post_id))
            .count()
            .get_result(connection).expect("Error counting cute kittens");

        return ReactionCountTypes {
            type1: query,
            type2: 0,
            type3: 0,
        }
    }

    pub fn delete(post_id: i32, author_id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(
            reactions::table
                .filter(reactions::post_id.eq(post_id))
                .filter(reactions::author_id.eq(author_id))
        ).execute(connection).is_ok()
    }
}
