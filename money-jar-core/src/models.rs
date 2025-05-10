pub struct User {
    id: String,
    name: String,
    email: String,
    phone: String,
    events: Vec<u16>

}

pub struct Event {
    id: String,
    name: String,
    description: String,
    date: String,
    reoccuring: bool,
    reoccuring_interval: String,
    final_occurrence: String,
    users: Vec<User>

}


pub struct Transaction {
    id: String,
    amount: f64,
    date: String,
    event: Event,
    from: User,
    to: User,
}

// above are the main structs for the database, below are the structs for the database intereactions, you should cast the main structs to these structs when interacting with the database

#[derive(Debug, Serialize, Deserialize)]
struct DB_User_Insert {
    name: String,
    email: String,
    phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DB_Event_Insert {
    id: String,
    name: String,
    description: String,
    date: String,
    reoccuring: bool,
    reoccuring_interval: String,
    final_occurrence: String,
    users: Vec<User>
}

#[derive(Debug, Serialize, Deserialize)]
struct DB_Transaction_Insert {
    amount: f64,
    date: String,
}


pub fn update_event(event: Event) -> Result<Event, Error> {
    let db_event = cast(event as DB_Event_Insert);

    let db_event = db_event.insert_into("events").execute(&mut conn);

    let event = db_event.map_err(|e| Error::new(ErrorKind::Other, e))?;

    Ok(event)
}