table! {
    departments (id) {
        id -> Int4,
        name -> Varchar,
        admin -> Nullable<Uuid>,
    }
}

table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    use crate::models::user::ClusterRoleMapping;

    use diesel::sql_types::{Uuid, Varchar, Text, Nullable, Int4, Timestamp};
    users (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Varchar,
        password -> Text,
        role -> ClusterRoleMapping,
        belong_to -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(users -> departments (belong_to));

allow_tables_to_appear_in_same_query!(
    departments,
    invitations,
    users,
);
