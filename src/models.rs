use diesel::prelude::*;
use crate::schema::{books, members, loans};
use chrono::{Datelike, Utc};

enum Status {
    BORROWED,
    LOST,
    AVAILABLE,
 }

#[derive(Queryable)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub auth_fst: String,
    pub auth_lst: String,
    pub curr_status: Status,
    pub isbn: String,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub auth_fst:&'a str,
    pub auth_lst:&'a str,
    pub curr_status: Status,
    pub isbn:&'a str,
}

#[derive(Queryable)]
pub struct Member {
    pub id: u32,
    pub fst_name: String,
    pub lst_name: String,
    pub dob: Date<UTC>,
}

#[derive(Insertable)]
#[table_name = "members"]
pub struct NewMember<'a> {
    pub fst_name: &'a str,
    pub lst_name:&'a str,
    pub dob: Date<UTC>,
}

#[derive(Queryable)]
pub struct Issue {
    pub id: u32,
    // doi stands for date of issue
    pub doi: Date<UTC>,
    pub due: Date<UTC>,
    pub bk_id: u32,
    pub mbr_id: u32,
}

#[derive(Insertable)]
#[table_name = "issues"]
pub struct NewIssue {
    pub doi: Date<UTC>,
    pub due: Date<UTC>,
    pub bk_id: u32,
    pub mbr_id: u32,
}
