// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Status;

    books (id) {
        id -> Int4,
        title -> Varchar,
        auth_fst -> Varchar,
        auth_lst -> Varchar,
        curr_status -> Status,
        isbn -> Bpchar,
    }
}

diesel::table! {
    issues (id) {
        id -> Int4,
        doi -> Date,
        due -> Date,
        bk_id -> Nullable<Int4>,
        mbr_id -> Nullable<Int4>,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        fst_name -> Varchar,
        lst_name -> Varchar,
        dob -> Date,
    }
}
diesel::joinable!(issues -> books (bk_id));
diesel::joinable!(issues -> members (mbr_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    issues,
    members,
);
