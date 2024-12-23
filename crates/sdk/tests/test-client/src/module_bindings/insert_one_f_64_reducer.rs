// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertOneF64Args {
    pub f: f64,
}

impl From<InsertOneF64Args> for super::Reducer {
    fn from(args: InsertOneF64Args) -> Self {
        Self::InsertOneF64 { f: args.f }
    }
}

impl __sdk::InModule for InsertOneF64Args {
    type Module = super::RemoteModule;
}

pub struct InsertOneF64CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_one_f64`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_one_f_64 {
    /// Request that the remote module invoke the reducer `insert_one_f64` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_one_f_64`] callbacks.
    fn insert_one_f_64(&self, f: f64) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_one_f64`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOneF64CallbackId`] can be passed to [`Self::remove_on_insert_one_f_64`]
    /// to cancel the callback.
    fn on_insert_one_f_64(
        &self,
        callback: impl FnMut(&super::EventContext, &f64) + Send + 'static,
    ) -> InsertOneF64CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_one_f_64`],
    /// causing it not to run in the future.
    fn remove_on_insert_one_f_64(&self, callback: InsertOneF64CallbackId);
}

impl insert_one_f_64 for super::RemoteReducers {
    fn insert_one_f_64(&self, f: f64) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_one_f64", InsertOneF64Args { f })
    }
    fn on_insert_one_f_64(
        &self,
        mut callback: impl FnMut(&super::EventContext, &f64) + Send + 'static,
    ) -> InsertOneF64CallbackId {
        InsertOneF64CallbackId(self.imp.on_reducer(
            "insert_one_f64",
            Box::new(move |ctx: &super::EventContext| {
                let super::EventContext {
                    event:
                        __sdk::Event::Reducer(__sdk::ReducerEvent {
                            reducer: super::Reducer::InsertOneF64 { f },
                            ..
                        }),
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, f)
            }),
        ))
    }
    fn remove_on_insert_one_f_64(&self, callback: InsertOneF64CallbackId) {
        self.imp.remove_on_reducer("insert_one_f64", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_one_f64`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_one_f_64 {
    /// Set the call-reducer flags for the reducer `insert_one_f64` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_one_f_64(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_one_f_64 for super::SetReducerFlags {
    fn insert_one_f_64(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_one_f64", flags);
    }
}
