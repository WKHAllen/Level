#![allow(dead_code)]

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::*;

/// An identifier for a value in the runtime.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct ValueId(usize);

/// A value in the runtime.
#[derive(Debug)]
struct Value<T> {
    /// The internal state of the value.
    state: UseStateHandle<Rc<T>>,
}

impl<T> Value<T> {
    /// Creates a new value.
    pub fn new(state: UseStateHandle<Rc<T>>) -> Self {
        Self { state }
    }

    /// Returns an `Rc` containing the inner value.
    pub fn get(&self) -> Rc<T> {
        Rc::clone(&*self.state)
    }

    /// Sets the inner value.
    pub fn set(&self, value: T) {
        self.state.set(Rc::new(value));
    }
}

/// A dynamic representation of a value in the runtime.
#[derive(Debug)]
struct ValueDyn(Box<dyn Any>);

impl ValueDyn {
    /// Creates a new dynamic value in the runtime.
    pub fn new<T>(state: UseStateHandle<Rc<T>>) -> Self
    where
        T: 'static,
    {
        Self(Box::new(Value::new(state)))
    }

    /// Casts the dynamic value into a ref of the inner value. Panics if the
    /// value is not of the expected type.
    pub fn downcast_ref<T>(&self) -> &Value<T>
    where
        T: 'static,
    {
        self.0
            .downcast_ref()
            .expect("received wrong type in downcast_ref")
    }

    /// Casts the dynamic value into a mut ref of the inner value. Panics if
    /// the value is not of the expected type.
    pub fn downcast_mut<T>(&mut self) -> &mut Value<T>
    where
        T: 'static,
    {
        self.0
            .downcast_mut()
            .expect("received wrong type in downcast_mut")
    }
}

/// A runtime containing all dynamic values.
#[derive(Debug, Default)]
struct Runtime {
    /// A map of the values themselves.
    values: RefCell<HashMap<ValueId, ValueDyn>>,
    /// The ID of the next value to use.
    next_value_id: Cell<ValueId>,
}

impl Runtime {
    /// Creates a new instance of a runtime.
    pub fn new() -> Self {
        Self {
            values: RefCell::new(HashMap::new()),
            next_value_id: Cell::new(ValueId(0)),
        }
    }

    /// Gets the next value ID and increments the internal count.
    pub fn new_value_id(&self) -> ValueId {
        let id = self.next_value_id.get();
        self.next_value_id.set(ValueId(id.0 + 1));
        id
    }

    /// Creates or updates a value within the runtime, depending on whether it
    /// already exists.
    pub fn create_value<T>(&self, id: ValueId, state: UseStateHandle<Rc<T>>)
    where
        T: 'static,
    {
        self.values
            .borrow_mut()
            .entry(id)
            .and_modify(|value| value.downcast_mut().state = state.clone())
            .or_insert_with(|| ValueDyn::new(state));
    }

    /// Gets an `Rc` containing a value from within the runtime. Panics if the
    /// value could not be found.
    pub fn get_value<T>(&self, id: ValueId) -> Rc<T>
    where
        T: 'static,
    {
        self.values
            .borrow()
            .get(&id)
            .expect("value with the given id does not exist")
            .downcast_ref::<T>()
            .get()
    }

    /// Sets the value with a given ID. Panics if the value could not be
    /// found.
    pub fn set_value<T>(&self, id: ValueId, value: T)
    where
        T: 'static,
    {
        self.values
            .borrow()
            .get(&id)
            .expect("value with the given id does not exist")
            .downcast_ref::<T>()
            .set(value);
    }

    /// Removes a value from the runtime. This should only happen when the
    /// value's creation context is destroyed, else future attempts to access
    /// the value will cause a panic.
    pub fn remove_value(&self, id: ValueId) {
        self.values.borrow_mut().remove(&id);
    }
}

thread_local! {
    /// The global runtime instance.
    static RT: Runtime = Runtime::new();
}

/// A handle to a value. It contains only an identifier with which it can
/// query the underlying runtime for the actual value. Because of this, it
/// implements `Copy`, and can be implicitly moved into any closure.
#[derive(Debug)]
pub struct UseValueHandle<T>
where
    T: 'static,
{
    /// The value's identifier.
    id: ValueId,
    /// A marker to make `T` relevant.
    marker: PhantomData<T>,
}

impl<T> UseValueHandle<T>
where
    T: 'static,
{
    /// Gets an `Rc` containing the inner value.
    pub fn get(&self) -> Rc<T> {
        RT.with(|rt| rt.get_value(self.id))
    }

    /// Sets the inner value, triggering a re-render.
    pub fn set(&self, value: T) {
        RT.with(|rt| rt.set_value(self.id, value));
    }

    /// Performs an immutable operation on the inner value.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&*self.get())
    }
}

impl<T> UseValueHandle<T>
where
    T: Copy + 'static,
{
    /// Gets a copy of the inner value.
    pub fn get_copy(&self) -> T {
        *self.get()
    }
}

impl<T> UseValueHandle<T>
where
    T: Clone + 'static,
{
    /// Gets a clone of the inner value.
    pub fn get_clone(&self) -> T {
        (*self.get()).clone()
    }
}

impl<T> Copy for UseValueHandle<T> {}

impl<T> Clone for UseValueHandle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            marker: self.marker,
        }
    }
}

/// Manage a stateful value in a function component. This is distinct from
/// `use_state` in that the returned handle is `Copy`, and can be implicitly
/// copied into any closure.
#[hook]
pub fn use_value<T, F>(init_fn: F) -> UseValueHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let id_state = use_state(|| RT.with(|rt| rt.new_value_id()));
    let id = *id_state;
    let state = use_state(|| Rc::new(init_fn()));
    RT.with(|rt| rt.create_value(id, state));

    use_unmount(move || RT.with(|rt| rt.remove_value(id)));

    UseValueHandle {
        id,
        marker: PhantomData,
    }
}
