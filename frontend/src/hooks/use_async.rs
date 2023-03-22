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
        match *self {
            Self::Loading(_) => true,
            _ => false,
        }
    }

    /// Check if the future succeeded.
    pub fn succeeded(&self) -> bool {
        match *self {
            Self::Success(_) => true,
            _ => false,
        }
    }

    /// Check if the future failed.
    pub fn failed(&self) -> bool {
        match *self {
            Self::Failure(_) => true,
            _ => false,
        }
    }
}

/// State handle for the [`use_async`] hook.
pub struct UseAsyncHandle<T, E> {
    inner: UseStateHandle<UseAsyncState<T, E>>,
    run: Rc<dyn Fn()>,
}

impl<T, E> UseAsyncHandle<T, E> {
    /// Start to resolve the async future to a final value.
    pub fn run(&self) {
        (self.run)();
    }

    /// Update `data` directly.
    #[allow(unused)]
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

/// This hook returns state and a `run` callback for an async future.
#[hook]
pub fn use_async<F, T, E>(future: F, run_on_init: bool) -> UseAsyncHandle<T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Clone + 'static,
    E: Clone + 'static,
{
    let inner = use_state(|| UseAsyncState::<T, E>::Init);
    let future_ref = use_mut_latest(Some(future));

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
