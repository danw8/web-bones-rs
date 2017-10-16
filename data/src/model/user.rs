use super::super::schema::users;

#[derive(Identifiable, Queryable, Deserialize, Serialize, Clone)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub email: String,
    pub enabled: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub enabled: bool,
}