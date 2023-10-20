use std::future::Future;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::{use_mount, use_mut_latest};

/// The previous state of an async future.
#[derive(Clone, PartialEq, Eq)]
pub enum PreviousUseAsyncState<T, E> {
    None,
    Success(T),
    Failure(E),
}

/// The current state of an async future.
#[derive(Clone, PartialEq, Eq)]
pub enum UseAsyncState<T, E> {
    Init,
    Loading(PreviousUseAsyncState<T, E>),
    Success(T),
    Failure(E),
}

#[allow(unused)]
impl<T, E> UseAsyncState<T, E> {
    /// Check if the future is loading.
    pub fn loading(&self) -> bool {
        matches!(*self, Self::Loading(_))
    }

    /// Check if the future succeeded.
    pub fn succeeded(&self) -> bool {
        matches!(*self, Self::Success(_))
    }

    /// Check if the future failed.
    pub fn failed(&self) -> bool {
        matches!(*self, Self::Failure(_))
    }
}

/// State handle for the [`use_async`] hook.
pub struct UseAsyncHandle<T, E> {
    inner: UseStateHandle<UseAsyncState<T, E>>,
    run: Rc<dyn Fn()>,
}

#[allow(dead_code)]
impl<T, E> UseAsyncHandle<T, E> {
    /// Start to resolve the async future to a final value.
    pub fn run(&self) {
        (self.run)();
    }

    /// Update `data` directly.
    pub fn update(&self, data: T) {
        self.inner.set(UseAsyncState::Success(data))
    }
}

impl<T, E> Deref for UseAsyncHandle<T, E> {
    type Target = UseAsyncState<T, E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, E> Clone for UseAsyncHandle<T, E> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            run: self.run.clone(),
        }
    }
}

impl<T, E> PartialEq for UseAsyncHandle<T, E>
where
    T: PartialEq,
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// Type alias for a callback to execute when a `use_async` state changes.
type UseAsyncUpdateCallback<T, E> = Option<Box<dyn FnOnce(&UseAsyncState<T, E>)>>;

/// Configuration options for the `use_async` hook.
pub struct UseAsync<F, T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Clone + PartialEq + 'static,
    E: Clone + PartialEq + 'static,
{
    /// The `use_async` future.
    future: F,
    /// Whether to run the future immediately.
    run_on_init: bool,
    /// An optional callback to execute when the state changes.
    on_update: UseAsyncUpdateCallback<T, E>,
}

#[allow(dead_code)]
impl<F, T, E> UseAsync<F, T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Clone + PartialEq + 'static,
    E: Clone + PartialEq + 'static,
{
    /// Creates a new `use_async` configuration with the given future.
    pub fn new(future: F) -> Self {
        Self {
            future,
            run_on_init: true,
            on_update: None,
        }
    }

    /// Sets whether to run the future immediately.
    pub fn run_on_init(mut self, run_on_init: bool) -> Self {
        self.run_on_init = run_on_init;
        self
    }

    /// Sets the state change callback.
    pub fn on_update<C>(mut self, callback: C) -> Self
    where
        C: FnOnce(&UseAsyncState<T, E>) + 'static,
    {
        self.on_update = Some(Box::new(callback));
        self
    }
}

/// This hook returns state and a `run` callback for an async future.
#[hook]
pub fn use_async<F, T, E>(config: UseAsync<F, T, E>) -> UseAsyncHandle<T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Clone + PartialEq + 'static,
    E: Clone + PartialEq + 'static,
{
    let UseAsync {
        future,
        run_on_init,
        on_update,
    } = config;

    let inner = use_state(|| UseAsyncState::<T, E>::Init);
    let future_ref = use_mut_latest(Some(future));

    use_effect_with(inner.clone(), move |value| {
        if let Some(callback) = on_update {
            (callback)(value);
        }
    });

    let run = {
        let inner = inner.clone();

        Rc::new(move || {
            let inner = inner.clone();
            let future_ref = future_ref.clone();

            spawn_local(async move {
                let future_ref = future_ref.current();
                let future = (*future_ref.borrow_mut()).take();

                if let Some(future) = future {
                    inner.set(UseAsyncState::Loading(match &*inner {
                        UseAsyncState::Init => PreviousUseAsyncState::None,
                        UseAsyncState::Loading(value) => value.clone(),
                        UseAsyncState::Success(data) => {
                            PreviousUseAsyncState::Success(data.clone())
                        }
                        UseAsyncState::Failure(err) => PreviousUseAsyncState::Failure(err.clone()),
                    }));

                    match future.await {
                        Ok(data) => inner.set(UseAsyncState::Success(data)),
                        Err(err) => inner.set(UseAsyncState::Failure(err)),
                    }
                }
            });
        })
    };

    {
        let run = run.clone();
        use_mount(move || {
            if run_on_init {
                run();
            }
        });
    }

    UseAsyncHandle { inner, run }
}
