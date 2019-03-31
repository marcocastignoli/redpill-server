use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::sql_query;
use post::schema::posts;
use user::schema::users;
use user::model::User;
use diesel::result::Error;
use chrono::{NaiveDate, NaiveDateTime};
use reaction::schema::reactions;
use diesel::sql_types::BigInt;
use diesel::sql_types::Varchar;
use diesel::sql_types::Datetime;
use diesel::sql_types::Nullable;
use diesel::sql_types::Integer;

#[table_name = "posts"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[belongs_to(User, foreign_key = "author_id")]
#[primary_key(post_id)]
pub struct Post {
    pub post_id: Option<i32>,
    pub content: String,
    pub author_id: Option<i32>,
    pub parent_id: i32,
    pub created: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct PostResult {
    #[sql_type="Integer"]
    pub post_id: i32,
    #[sql_type="Varchar"]
    pub content: String,
    #[sql_type="Integer"]
    pub author_id: i32,
    #[sql_type="Integer"]
    pub parent_id: i32,
    #[sql_type="Nullable<Datetime>"]
    pub created: Option<NaiveDateTime>,
    #[sql_type="Varchar"]
    pub email: String,
    #[sql_type="BigInt"]
    pub upvotes: i64,
    #[sql_type="BigInt"]
    pub downvotes: i64,
    #[sql_type="Nullable<Integer>"]
    my_vote: Option<i32>,
    #[sql_type="BigInt"]
    pub subposts: i64
}

impl Post {
    pub fn create(vettore: Post, connection: &MysqlConnection) -> bool {
        diesel::insert_into(posts::table)
            .values(&vettore)
            .execute(connection)
            .is_ok()
    }

    pub fn read(
        limit: i64,
        offset: i64,
        parent_id: i32,
        sortBy: Option<String>,
        sortDesc: Option<bool>,
        content: Option<String>,
        connection: &MysqlConnection
    ) -> Result<Vec<PostResult>, Error> {
        sql_query("SELECT 
                p.post_id,
                content,
                p.author_id,
                p.parent_id,
                p.created,
                users.email,
                count(IF(reactions.reaction_type=1, 1, NULL)) as upvotes,
                count(IF(reactions.reaction_type=2, 1, NULL)) as downvotes,
                (SELECT reaction_type FROM reactions WHERE author_id = 1 AND post_id = p.post_id) as my_vote,
                (SELECT count(*) FROM posts p1 WHERE p1.parent_id = p.post_id) as subposts
            FROM posts p
            LEFT JOIN reactions ON p.post_id = reactions.post_id
            LEFT JOIN users ON p.author_id = users.user_id
            WHERE parent_id = ?
            GROUP BY p.post_id, content, users.email, p.created, p.parent_id, p.author_id
            ORDER BY created DESC;
        ")
        .bind::<Integer, _>(parent_id)
        .load(connection)
    }

    pub fn count(parent_id: i32, content: Option<String>, connection: &MysqlConnection) -> i64 {
        let content_ = format!("%{}%", content.unwrap_or("".to_string()));
        posts::table
            .filter(posts::content.like(content_))
            .count()
            .filter(posts::parent_id.eq(parent_id))
            .get_result(connection).expect("Error counting cute kittens")
    }

    pub fn update(id: i32, author_id: i32, post: Post, connection: &MysqlConnection) -> bool {
        diesel::update(
            posts::table
                .filter(posts::post_id.eq(id))
                .filter(posts::author_id.eq(author_id))
        ).set(&post).execute(connection).is_ok()
    }

    pub fn delete(id: i32, author_id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(
            posts::table
                .filter(posts::post_id.eq(id))
                .filter(posts::author_id.eq(author_id))
        ).execute(connection).is_ok()
    }
}
