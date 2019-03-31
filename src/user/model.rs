use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use user::schema::users;
use chrono::{NaiveDate, NaiveDateTime};

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub user_id: Option<i32>,
    pub email: String,
    pub password: String,
    pub created: Option<NaiveDateTime>
}

impl User {
    pub fn create(user: User, connection: &MysqlConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)?;

        users::table.order(users::user_id.desc()).first(connection)
    }

    pub fn read(id: i32, connection: &MysqlConnection) -> QueryResult<Vec<User>> {
        if id != 0 {
            users::table.find(id).load::<User>(connection)
        } else {
            users::table.order(users::user_id).load::<User>(connection)
        }
    }

    pub fn by_email_and_password(email_: String, password_: String, connection: &MysqlConnection) -> Option<User> {
        let res = users::table
            .filter(users::email.eq(email_))
            .filter(users::password.eq(password_))
            .order(users::user_id)
            .first(connection);
        match res {
            Ok(user) => Some(user),
            Err(_) => {
                None
            }
        }
    }

    pub fn update(id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}
