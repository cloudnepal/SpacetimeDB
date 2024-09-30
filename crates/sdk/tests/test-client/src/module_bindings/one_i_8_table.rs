// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::one_i_8_type::OneI8;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `one_i8`.
///
/// Obtain a handle from the [`OneI8TableAccess::one_i_8`] method on [`super::RemoteTables`],
/// like `ctx.db.one_i_8()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_i_8().on_insert(...)`.
pub struct OneI8TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<OneI8>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_i8`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneI8TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneI8TableHandle`], which mediates access to the table `one_i8`.
    fn one_i_8(&self) -> OneI8TableHandle<'_>;
}

impl OneI8TableAccess for super::RemoteTables {
    fn one_i_8(&self) -> OneI8TableHandle<'_> {
        OneI8TableHandle {
            imp: self.imp.get_table::<OneI8>("one_i8"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneI8InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct OneI8DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for OneI8TableHandle<'ctx> {
    type Row = OneI8;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneI8> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneI8InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneI8InsertCallbackId {
        OneI8InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneI8InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneI8DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneI8DeleteCallbackId {
        OneI8DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneI8DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<OneI8>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"one_i8\"")
}