use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct DB_Events_Insert_ATG {
    pub id: String,
    pub name: String,
    pub description: String,
    pub date: String,
    pub reoccuring: bool,
    pub reoccuring_interval: String,
    pub final_occurrence: String,
}

impl DB_Events_Insert_ATG {
    pub fn new(id: String, name: String, description: String, date: String, 
               reoccuring: bool, reoccuring_interval: String, final_occurrence: String) -> Self {
        Self {
            id,
            name,
            description,
            date,
            reoccuring,
            reoccuring_interval,
            final_occurrence,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct DB_Events_Insert_ATF {
    pub id: String,
    pub name: String,
    pub description: String,
    pub date: String,
    pub reoccuring: bool,
    pub reoccuring_interval: String,
}

impl DB_Events_Insert_ATF {
    pub fn new(id: String, name: String, description: String, date: String, 
               reoccuring: bool, reoccuring_interval: String) -> Self {
        Self {
            id,
            name,
            description,
            date,
            reoccuring,
            reoccuring_interval,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct DB_Events_Insert_G {
    pub final_occurrence: String,
}

impl DB_Events_Insert_G {
    pub fn new(final_occurrence: String) -> Self {
        Self { final_occurrence }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = events)]
pub struct DB_Events_Update_G {
    pub final_occurrence: String,
}

impl DB_Events_Update_G {
    pub fn new(final_occurrence: String) -> Self {
        Self { final_occurrence }
    }
}

#[derive(Queryable)]
#[diesel(table_name = events)]
pub struct DB_Events_Select_G {
    pub final_occurrence: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = events)]
pub struct DB_Events_Update_C {
    pub description: String,
}

impl DB_Events_Update_C {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = events)]
pub struct DB_Events_Update_D {
    pub date: String,
}

impl DB_Events_Update_D {
    pub fn new(date: String) -> Self {
        Self { date }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = events)]
pub struct DB_Events_Update_EF {
    pub reoccuring: bool,
    pub reoccuring_interval: String,
}

impl DB_Events_Update_EF {
    pub fn new(reoccuring: bool, reoccuring_interval: String) -> Self {
        Self {
            reoccuring,
            reoccuring_interval,
        }
    }
} 