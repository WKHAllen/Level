use crate::backend::BackendHandle;
use crate::hooks::{use_backend, use_result};
use common::*;
use std::future::Future;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// The state of a command operation.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum UseCommandState<T> {
    /// Initialized but not running.
    #[default]
    Init,
    /// The operation is running.
    Loading,
    /// The operation ran and has completed.
    Resolved(T),
}

#[allow(dead_code)]
impl<T> UseCommandState<T> {
    /// Gets the inner resolved value, if it exists.
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Resolved(val) => Some(val),
            _ => None,
        }
    }
}

/// A handle to a command operation.
#[derive(Clone)]
pub struct UseCommandHandle<T> {
    /// The inner command operation state.
    inner: UseStateHandle<UseCommandState<T>>,
    /// A callable that will run the command operation.
    run: Rc<dyn Fn()>,
}

impl<T> UseCommandHandle<T> {
    /// Runs the command operation.
    pub fn run(&self) {
        (self.run)();
    }
}

impl<T> Deref for UseCommandHandle<T> {
    type Target = UseCommandState<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Type alias for a callback to execute when a `use_command*` state changes.
type UseCommandUpdateCallback<T> =
    Option<Box<dyn FnOnce(&UseCommandState<Result<T, ExpectedCommandError>>)>>;

/// Type alias for a callback to execute when a fallible `use_command*` state
/// changes.
type UseCommandUpdateInfallibleCallback<T> = Option<Box<dyn FnOnce(&UseCommandState<T>)>>;

/// Configuration options for the `use_command` hook.
pub struct UseCommand<C, F, T>
where
    C: FnOnce(Rc<BackendHandle>) -> F,
    F: Future<Output = CommandResult<T>> + 'static,
    T: Clone + PartialEq + 'static,
{
    /// The `use_command` callable.
    f: C,
    /// Whether to run the future immediately.
    run_on_init: bool,
    /// An optional callback to execute when the state changes.
    on_update: UseCommandUpdateCallback<T>,
}

impl<C, F, T> UseCommand<C, F, T>
where
    C: FnOnce(Rc<BackendHandle>) -> F,
    F: Future<Output = CommandResult<T>> + 'static,
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `use_command` configuration with the given callable.
    pub fn new(f: C) -> Self {
        Self {
            f,
            run_on_init: true,
            on_update: None,
        }
    }

    /// Sets whether to run the callable immediately.
    pub fn run_on_init(mut self, run_on_init: bool) -> Self {
        self.run_on_init = run_on_init;
        self
    }

    /// Sets the state change callback.
    pub fn on_update<U>(mut self, callback: U) -> Self
    where
        U: FnOnce(&UseCommandState<Result<T, ExpectedCommandError>>) + 'static,
    {
        self.on_update = Some(Box::new(callback));
        self
    }
}

/// Configuration options for the `use_command_infallible` hook.
pub struct UseCommandInfallible<C, F, T>
where
    C: FnOnce(Rc<BackendHandle>) -> F,
    F: Future<Output = T> + 'static,
    T: Clone + PartialEq + 'static,
{
    /// The `use_command_infallible` callable.
    f: C,
    /// Whether to run the callable immediately.
    run_on_init: bool,
    /// An optional callback to execute when the state changes.
    on_update: UseCommandUpdateInfallibleCallback<T>,
}

#[allow(dead_code)]
impl<C, F, T> UseCommandInfallible<C, F, T>
where
    C: FnOnce(Rc<BackendHandle>) -> F,
    F: Future<Output = T> + 'static,
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `use_command_infallible` configuration with the given
    /// callable.
    pub fn new(f: C) -> Self {
        Self {
            f,
            run_on_init: true,
            on_update: None,
        }
    }

    /// Sets whether to run the callable immediately.
    pub fn run_on_init(mut self, run_on_init: bool) -> Self {
        self.run_on_init = run_on_init;
        self
    }

    /// Sets the state change callback.
    pub fn on_update<U>(mut self, callback: U) -> Self
    where
        U: FnOnce(&UseCommandState<T>) + 'static,
    {
        self.on_update = Some(Box::new(callback));
        self
    }
}

/// Configuration options for the `use_command_sync` hook.
pub struct UseCommandSync<F, T>
where
    F: FnOnce(Rc<BackendHandle>) -> CommandResult<T> + 'static,
    T: Clone + PartialEq + 'static,
{
    /// The `use_command_sync` callable.
    f: F,
    /// Whether to run the callable immediately.
    run_on_init: bool,
    /// An optional callback to execute when the state changes.
    on_update: UseCommandUpdateCallback<T>,
}

#[allow(dead_code)]
impl<F, T> UseCommandSync<F, T>
where
    F: FnOnce(Rc<BackendHandle>) -> CommandResult<T> + 'static,
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `use_command_sync` configuration with the given
    /// callable.
    pub fn new(f: F) -> Self {
        Self {
            f,
            run_on_init: true,
            on_update: None,
        }
    }

    /// Sets whether to run the callable immediately.
    pub fn run_on_init(mut self, run_on_init: bool) -> Self {
        self.run_on_init = run_on_init;
        self
    }

    /// Sets the state change callback.
    pub fn on_update<U>(mut self, callback: U) -> Self
    where
        U: FnOnce(&UseCommandState<Result<T, ExpectedCommandError>>) + 'static,
    {
        self.on_update = Some(Box::new(callback));
        self
    }
}

/// Configuration options for the `use_command_sync_infallible` hook.
pub struct UseCommandSyncInfallible<F, T>
where
    F: FnOnce(Rc<BackendHandle>) -> T + 'static,
    T: Clone + PartialEq + 'static,
{
    /// The `use_command_sync_infallible` callable.
    f: F,
    /// Whether to run the callable immediately.
    run_on_init: bool,
    /// An optional callback to execute when the state changes.
    on_update: UseCommandUpdateInfallibleCallback<T>,
}

#[allow(dead_code)]
impl<F, T> UseCommandSyncInfallible<F, T>
where
    F: FnOnce(Rc<BackendHandle>) -> T + 'static,
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `use_command_sync_infallible` configuration with the
    /// given callable.
    pub fn new(f: F) -> Self {
        Self {
            f,
            run_on_init: true,
            on_update: None,
        }
    }

    /// Sets whether to run the callable immediately.
    pub fn run_on_init(mut self, run_on_init: bool) -> Self {
        self.run_on_init = run_on_init;
        self
    }

    /// Sets the state change callback.
    pub fn on_update<U>(mut self, callback: U) -> Self
    where
        U: FnOnce(&UseCommandState<T>) + 'static,
    {
        self.on_update = Some(Box::new(callback));
        self
    }
}

/// Gives access to the backend command system within an async environment.
/// Errors are handled automatically.
///
/// For an infallible version of this, use [`use_command_infallible`].
///
/// For a synchronous version of this, use [`use_command_sync`].
///
/// For a synchronous, infallible version of this, use [`use_command_sync_infallible`].
#[hook]
pub fn use_command<C, F, T>(
    config: UseCommand<C, F, T>,
) -> UseCommandHandle<Result<T, ExpectedCommandError>>
where
    C: FnOnce(Rc<BackendHandle>) -> F + 'static,
    F: Future<Output = CommandResult<T>> + 'static,
    T: Clone + PartialEq + 'static,
{
    let UseCommand {
        f,
        run_on_init,
        on_update,
    } = config;

    let (backend, _) = use_backend();
    let res = use_result();

    let inner = use_state(|| UseCommandState::Init);
    let f_ref = use_mut_latest(Some(f));

    use_effect_with(inner.clone(), move |value| {
        if let Some(callback) = on_update {
            (callback)(value);
        }
    });

    let run = {
        let inner = inner.clone();

        Rc::new(move || {
            let backend = backend.clone();
            let res = res.clone();
            let inner = inner.clone();
            let f_ref = f_ref.clone();

            spawn_local(async move {
                let f_ref = f_ref.current();
                let f = (*f_ref.borrow_mut()).take();

                if let Some(f) = f {
                    inner.set(UseCommandState::Loading);
                    let value_result = f(backend).await;
                    let value = res.handle(value_result);
                    inner.set(UseCommandState::Resolved(value));
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

    UseCommandHandle { inner, run }
}

/// Gives access to the backend command system within an async environment.
/// Errors are handled automatically.
///
/// For a synchronous version of this, use [`use_command_sync`].
#[hook]
pub fn use_command_infallible<C, F, T>(config: UseCommandInfallible<C, F, T>) -> UseCommandHandle<T>
where
    C: FnOnce(Rc<BackendHandle>) -> F + 'static,
    F: Future<Output = T> + 'static,
    T: Clone + PartialEq + 'static,
{
    let UseCommandInfallible {
        f,
        run_on_init,
        on_update,
    } = config;

    let (backend, _) = use_backend();

    let inner = use_state(|| UseCommandState::Init);
    let f_ref = use_mut_latest(Some(f));

    use_effect_with(inner.clone(), move |value| {
        if let Some(callback) = on_update {
            (callback)(value);
        }
    });

    let run = {
        let inner = inner.clone();

        Rc::new(move || {
            let backend = backend.clone();
            let inner = inner.clone();
            let f_ref = f_ref.clone();

            spawn_local(async move {
                let f_ref = f_ref.current();
                let f = (*f_ref.borrow_mut()).take();

                if let Some(f) = f {
                    inner.set(UseCommandState::Loading);
                    let value = f(backend).await;
                    inner.set(UseCommandState::Resolved(value));
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

    UseCommandHandle { inner, run }
}

/// See [`use_command`].
#[hook]
pub fn use_command_sync<C, T>(
    config: UseCommandSync<C, T>,
) -> UseCommandHandle<Result<T, ExpectedCommandError>>
where
    C: FnOnce(Rc<BackendHandle>) -> CommandResult<T> + 'static,
    T: Clone + PartialEq + 'static,
{
    let UseCommandSync {
        f,
        run_on_init,
        on_update,
    } = config;

    let (backend, _) = use_backend();
    let res = use_result();

    let inner = use_state(|| UseCommandState::Init);
    let f_ref = use_mut_latest(Some(f));

    use_effect_with(inner.clone(), move |value| {
        if let Some(callback) = on_update {
            (callback)(value);
        }
    });

    let run = {
        let inner = inner.clone();

        Rc::new(move || {
            let backend = backend.clone();
            let inner = inner.clone();
            let f_ref = f_ref.clone();

            let f_ref = f_ref.current();
            let f = (*f_ref.borrow_mut()).take();

            if let Some(f) = f {
                inner.set(UseCommandState::Loading);
                let value_result = f(backend);
                let value = res.handle(value_result);
                inner.set(UseCommandState::Resolved(value));
            }
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

    UseCommandHandle { inner, run }
}

/// See [`use_command`].
#[hook]
pub fn use_command_sync_infallible<C, T>(
    config: UseCommandSyncInfallible<C, T>,
) -> UseCommandHandle<T>
where
    C: FnOnce(Rc<BackendHandle>) -> T + 'static,
    T: Clone + PartialEq + 'static,
{
    let UseCommandSyncInfallible {
        f,
        run_on_init,
        on_update,
    } = config;

    let (backend, _) = use_backend();

    let inner = use_state(|| UseCommandState::Init);
    let f_ref = use_mut_latest(Some(f));

    use_effect_with(inner.clone(), move |value| {
        if let Some(callback) = on_update {
            (callback)(value);
        }
    });

    let run = {
        let inner = inner.clone();

        Rc::new(move || {
            let backend = backend.clone();
            let inner = inner.clone();
            let f_ref = f_ref.clone();

            let f_ref = f_ref.current();
            let f = (*f_ref.borrow_mut()).take();

            if let Some(f) = f {
                inner.set(UseCommandState::Loading);
                let value = f(backend);
                inner.set(UseCommandState::Resolved(value));
            }
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

    UseCommandHandle { inner, run }
}
