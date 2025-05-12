use diesel::{Insertable, Queryable, AsChangeset};
use crate::schema::Users;




//template for structs

struct Users {
    id: String,
    name: String,
    email: String,
    phone: String,
}
//DB MODELS


//insert models


#[derive(Insertable)]
#[diesel(table_name = Users)]
pub struct NewUser {
    id: String,
    name: String,
    email: String,
}

impl NewUser {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

//update models

#[derive(AsChangeset)]
#[diesel(table_name = Users)]
pub struct UpdateName {
    email: String,
}

impl UpdateName {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

pub struct UpdateEmail {
    name: String,
}

impl UpdateEmail {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct UpdatePhone {
    name: String,
}

impl UpdatePhone {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

//query models 

//get by id
#[derive(Queryable)]
#[diesel(table_name = Users)]
pub struct GetName {
    pub id: String,
    pub name: String,
}

impl GetName {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Queryable)]
#[diesel(table_name = Users)]
pub struct GetEmail {
    pub id: String,
    pub email: String,
}

impl GetEmail {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Queryable)]
#[diesel(table_name = Users)]
pub struct GetPhone {
    pub id: String,
    pub phone: String,
}   

impl GetPhone {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}





