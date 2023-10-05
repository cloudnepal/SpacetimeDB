// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InsertUniqueStringArgs {
    pub s: String,
    pub data: i32,
}

impl Reducer for InsertUniqueStringArgs {
    const REDUCER_NAME: &'static str = "insert_unique_string";
}

#[allow(unused)]
pub fn insert_unique_string(s: String, data: i32) {
    InsertUniqueStringArgs { s, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_unique_string(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueStringArgs> {
    InsertUniqueStringArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueStringArgs { s, data } = __args;
        __callback(__identity, __addr, __status, s, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_unique_string(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueStringArgs> {
    InsertUniqueStringArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueStringArgs { s, data } = __args;
        __callback(__identity, __addr, __status, s, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_unique_string(id: ReducerCallbackId<InsertUniqueStringArgs>) {
    InsertUniqueStringArgs::remove_on_reducer(id);
}