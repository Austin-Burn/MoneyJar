// @generated automatically by Diesel CLI.

diesel::table! {
    Events (id) {
        id -> Text,
        owner_id -> Text,
        name -> Text,
        description -> Text,
        date -> Text,
        reoccuring -> Bool,
        reoccuring_interval -> Text,
    }
}

diesel::table! {
    Transactions (id) {
        id -> Nullable<Integer>,
        from_user_id -> Text,
        to_user_id -> Text,
        event_id -> Text,
        amount -> Integer,
        date -> Text,
    }
}

diesel::table! {
    UserPaymentMethods (user_id, payment_method) {
        user_id -> Text,
        payment_method -> Text,
    }
}

diesel::table! {
    Users (id) {
        id -> Nullable<Text>,
        name -> Text,
        email -> Text,
        phone -> Nullable<Text>,
    }
}

diesel::table! {
    WhoInWhat (user_id, event_id) {
        user_id -> Text,
        event_id -> Text,
    }
}

diesel::joinable!(Events -> Users (owner_id));
diesel::joinable!(Transactions -> Events (event_id));
diesel::joinable!(UserPaymentMethods -> Users (user_id));
diesel::joinable!(WhoInWhat -> Events (event_id));
diesel::joinable!(WhoInWhat -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    Events,
    Transactions,
    UserPaymentMethods,
    Users,
    WhoInWhat,
);
