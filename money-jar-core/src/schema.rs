// @generated automatically by Diesel CLI.

diesel::table! {
    Auth (user_id) {
        user_id -> Text,
        token -> Text,
        expiry -> Text,
    }
}

diesel::table! {
    Events (id) {
        id -> Text,
        owner_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        event_date -> Nullable<Text>,
        reoccuring -> Bool,
        reoccuring_interval -> Nullable<Text>,
        final_date -> Nullable<Text>,
    }
}

diesel::table! {
    Friends (user_id, friend_id) {
        user_id -> Text,
        friend_id -> Text,
    }
}

diesel::table! {
    Transactions (id) {
        id -> Integer,
        from_user_id -> Text,
        to_user_id -> Text,
        event_id -> Text,
        amount -> Integer,
        date -> Text,
        payment_method -> Text,
        comment -> Nullable<Text>,
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
        id -> Text,
        name -> Text,
        email -> Text,
        phone -> Nullable<Text>,
        password -> Text,
    }
}

diesel::table! {
    WhoInWhat (user_id, event_id) {
        user_id -> Text,
        event_id -> Text,
    }
}

diesel::joinable!(Auth -> Users (user_id));
diesel::joinable!(Events -> Users (owner_id));
diesel::joinable!(Transactions -> Events (event_id));
diesel::joinable!(UserPaymentMethods -> Users (user_id));
diesel::joinable!(WhoInWhat -> Events (event_id));
diesel::joinable!(WhoInWhat -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    Auth,
    Events,
    Friends,
    Transactions,
    UserPaymentMethods,
    Users,
    WhoInWhat,
);
