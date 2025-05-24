use diesel::{Insertable, Queryable, AsChangeset};
use crate::schema::Users;




//template for structs

//struct TemplateUsers {
//    id: String,
//    name: String,
//    email: String,
//    phone: String,
//}
//DB MODELS


//insert models


#[derive(Insertable)]
#[diesel(table_name = Users)]
pub struct NewUser {
    id: String,
    name: String,
    email: String,
    password: String,
}

impl NewUser {
    pub fn new(id: String, name: String, email: String, password: String) -> Self {
        Self { id, name, email, password }
    }
}

//update models

#[derive(AsChangeset)]
#[diesel(table_name = Users)]
pub struct UpdateName {
    name: String,
}

impl UpdateName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Users)]
pub struct UpdateEmail {
    email: String,
}

impl UpdateEmail {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Users)]
pub struct UpdatePhone {
    phone: String,
}

impl UpdatePhone {
    pub fn new(phone: String) -> Self {
        Self { phone }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Users)]
pub struct UpdatePassword {
    password: String,
}

impl UpdatePassword {
    pub fn new(password: String) -> Self {
        Self { password }
    }
}




//get by id
#[derive(Queryable)]
#[diesel(table_name = Users)]
pub struct GetName {
    pub name: String,
}

impl GetName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Queryable)]
#[diesel(table_name = Users)]
pub struct GetEmail {
    pub email: String,
}

impl GetEmail {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

#[derive(Queryable)]
#[diesel(table_name = Users)]
pub struct GetPhone {
    pub phone: String,
}   

impl GetPhone {
    pub fn new(phone: String) -> Self {
        Self { phone }
    }
}

#[derive(Queryable)]
#[diesel(table_name = Users)]
pub struct GetId {
    pub id: String,
}

impl GetId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}







