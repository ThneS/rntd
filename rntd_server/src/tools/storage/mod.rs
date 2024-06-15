//! pgsql sqlx storage
// pgsql with sqlx storage
// wiki from conf/sql/wiki.sql
// event from conf/sql/event.sql

use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    prelude::FromRow,
};
use std::{ops::Deref, time::Duration};
pub struct Storage;

#[derive(Clone)]
pub struct StoragePool {
    inner: PgPool,
}
impl Deref for StoragePool {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Wiki {
    alarm_id: i64,
    wiki: String,
    advice: String,
    level: String,
    module: String,
    sub_module: String,
    create_time: NaiveDateTime,
    update_time: Option<NaiveDateTime>,
    enable: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Event {
    id: i64,
    task_id: i64,
    alarm_id: i64,
    create_time: NaiveDateTime,
    timestamp: i64,
    advice: String,
    level: String,
    module: String,
    sub_module: String,
}

#[allow(unused)]
impl Storage {
    pub async fn get_wiki(&mut self, _pool: &PgPool, _alarm_id: i64) -> Result<Wiki, sqlx::Error> {
        todo!("get wiki by alarm_id")
    }
    pub async fn get_event(&mut self, _pool: &PgPool, _id: i64) -> Result<Event, sqlx::Error> {
        todo!("get event by id")
    }
    pub async fn insert_wiki(&mut self, _pool: &PgPool, _wiki: Wiki) -> Result<(), sqlx::Error> {
        todo!("insert wiki")
    }
    pub async fn insert_event(&mut self, _pool: &PgPool, _event: Event) -> Result<(), sqlx::Error> {
        todo!("insert event")
    }
}

pub async fn try_new_pool() -> Result<StoragePool, sqlx::Error> {
    let inner = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("ntd:ntd@localhost:5432/ntd")
        .await
        .expect("can't connect to database");
    Ok(StoragePool { inner })
}
