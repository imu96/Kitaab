use diesel::prelude::*;
use chrono::{Datelike, Utc};

enum Status {
    BORROWED,
    LOST,
    AVAILABLE,
}

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub auth_fst: String,
    pub auth_lst: String,
    pub curr_status: Status,
    pub isbn: String,
}

#[derive(Queryable)]
pub struct Member {
    pub id: i32,
    pub fst_name: String,
    pub lst_name: String,
    pub dob: Date<UTC>,
}

#[derive(Queryable)]
pub struct Loan {
    pub id: i32,
    // doi stands for date of issue
    pub doi: Date<UTC>,
    pub due: Date<UTC>,
    pub bk_id: i32,
    pub mbr_id: i32,
}