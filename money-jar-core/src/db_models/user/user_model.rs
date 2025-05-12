use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct DB_Users_Insert_ATD {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl DB_Users_Insert_ATD {
    pub fn new(id: String, name: String, email: String, phone: String) -> Self {
        Self {
            id,
            name,
            email,
            phone,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct DB_Users_Insert_ATC {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl DB_Users_Insert_ATC {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct DB_Users_Insert_D {
    pub phone: String,
}

impl DB_Users_Insert_D {
    pub fn new(phone: String) -> Self {
        Self { phone }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct DB_Users_Update_D {
    pub phone: String,
}

impl DB_Users_Update_D {
    pub fn new(phone: String) -> Self {
        Self { phone }
    }
}

#[derive(Queryable)]
#[diesel(table_name = users)]
pub struct DB_Users_Select_D {
    pub phone: String,
}

impl DB_Users_Select_D {
    pub fn new(phone: String) -> Self {
        Self { phone }
    }
} 