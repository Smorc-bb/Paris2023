// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    base_types::{ObjectID, ObjectRef, SequenceNumber},
    error::SuiResult,
    event::Event,
    object::Object,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum DeleteKind {
    /// An object is provided in the call input, and gets deleted.
    Normal,
    /// An object is not provided in the call input, but gets unwrapped
    /// from another object, and then gets deleted.
    UnwrapThenDelete,
    /// An object is provided in the call input, and gets wrapped into another object.
    Wrap,
}

pub enum ObjectChange {
    Write(Object),
    Delete(SequenceNumber, DeleteKind),
}

/// An abstraction of the (possibly distributed) store for objects, and (soon) events and transactions
pub trait Storage {
    fn reset(&mut self);

    fn read_object(&self, id: &ObjectID) -> Option<&Object>;

    // Specify the list of object IDs created during the transaction.
    // This is needed to determine unwrapped objects at the end.
    fn set_create_object_ids(&mut self, ids: BTreeSet<ObjectID>);

    /// Record an event that happened during execution
    fn log_event(&mut self, event: Event);

    fn apply_object_changes(&mut self, changes: BTreeMap<ObjectID, ObjectChange>);
}

pub trait BackingPackageStore {
    fn get_package(&self, package_id: &ObjectID) -> SuiResult<Option<Object>>;
}

impl<S: BackingPackageStore> BackingPackageStore for std::sync::Arc<S> {
    fn get_package(&self, package_id: &ObjectID) -> SuiResult<Option<Object>> {
        BackingPackageStore::get_package(self.as_ref(), package_id)
    }
}

impl<S: BackingPackageStore> BackingPackageStore for &S {
    fn get_package(&self, package_id: &ObjectID) -> SuiResult<Option<Object>> {
        BackingPackageStore::get_package(*self, package_id)
    }
}

impl<S: BackingPackageStore> BackingPackageStore for &mut S {
    fn get_package(&self, package_id: &ObjectID) -> SuiResult<Option<Object>> {
        BackingPackageStore::get_package(*self, package_id)
    }
}

pub trait ParentSync {
    fn get_latest_parent_entry_ref(&self, object_id: ObjectID) -> SuiResult<Option<ObjectRef>>;
}

impl<S: ParentSync> ParentSync for std::sync::Arc<S> {
    fn get_latest_parent_entry_ref(&self, object_id: ObjectID) -> SuiResult<Option<ObjectRef>> {
        ParentSync::get_latest_parent_entry_ref(self.as_ref(), object_id)
    }
}

impl<S: ParentSync> ParentSync for &S {
    fn get_latest_parent_entry_ref(&self, object_id: ObjectID) -> SuiResult<Option<ObjectRef>> {
        ParentSync::get_latest_parent_entry_ref(*self, object_id)
    }
}

impl<S: ParentSync> ParentSync for &mut S {
    fn get_latest_parent_entry_ref(&self, object_id: ObjectID) -> SuiResult<Option<ObjectRef>> {
        ParentSync::get_latest_parent_entry_ref(*self, object_id)
    }
}
