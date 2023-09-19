use js_sys::{Function, Promise, Reflect};
use serde::Serialize;
use std::path::PathBuf;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

/// Extension filters for the Tauri dialog API.
#[derive(Debug, Clone, Serialize)]
pub struct FileSelectFilters {
    /// The filter name.
    name: String,
    /// The extensions to filter.
    extensions: Vec<String>,
}

/// Arguments passed to the Tauri dialog API.
#[derive(Debug, Clone, Serialize)]
pub struct FileSelectArgs {
    /// The path to start the file selection from.
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
    /// The file selection dialog window title.
    title: Option<String>,
    /// Whether to allow directory selection.
    directory: bool,
    /// Whether to allow selection of multiple files.
    multiple: bool,
    /// A list of dialog filters.
    filters: Option<Vec<FileSelectFilters>>,
}

/// File selection configuration.
#[derive(Default)]
pub struct FileSelectConfig {
    /// The path to start the file selection from.
    start_path: Option<String>,
    /// The file selection dialog window title.
    dialog_title: Option<String>,
    /// Whether to allow directory selection.
    directory: bool,
    /// Whether to allow selection of multiple files.
    multiple: bool,
    /// A list of acceptable file extensions. If empty, all files will be
    /// allowed.
    extensions: Option<Vec<String>>,
    /// The callback called when the user selects a file.
    on_select: Option<Rc<dyn Fn(Option<PathBuf>)>>,
}

impl FileSelectConfig {
    /// Create a new default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the path to start the file selection from.
    pub fn start_path(mut self, path: Option<String>) -> Self {
        self.start_path = path;
        self
    }

    /// Sets the file selection dialog window title.
    pub fn dialog_title(mut self, title: Option<String>) -> Self {
        self.dialog_title = title;
        self
    }

    /// Enables or disables directory selection.
    pub fn directory(mut self, allow: bool) -> Self {
        self.directory = allow;
        self
    }

    /// Enables or disables multiple file selection.
    pub fn multiple(mut self, allow: bool) -> Self {
        self.multiple = allow;
        self
    }

    /// Sets the allowed file extensions. If not specified, all file
    /// extensions will be allowed.
    pub fn extensions(mut self, exts: Option<Vec<String>>) -> Self {
        self.extensions = exts;
        self
    }

    /// Sets the callback function for when the user selects a file.
    pub fn on_select<F>(mut self, f: F) -> Self
    where
        F: Fn(Option<PathBuf>) + 'static,
    {
        self.on_select = Some(Rc::new(f));
        self
    }

    /// Convert the configuration into the arguments for the the file open
    /// API.
    pub fn to_args(&self) -> FileSelectArgs {
        FileSelectArgs {
            default_path: self.start_path.clone(),
            title: self.dialog_title.clone(),
            directory: self.directory,
            multiple: self.multiple,
            filters: self.extensions.as_ref().map(|exts| {
                vec![FileSelectFilters {
                    name: "File types".to_owned(),
                    extensions: exts.clone(),
                }]
            }),
        }
    }
}

/// A file selection handle.
pub struct UseFileSelect {
    /// The provided configuration.
    config: FileSelectConfig,
}

impl UseFileSelect {
    /// Opens the file selection dialog.
    pub fn open(&self) {
        let tauri = web_sys::window().unwrap().get("__TAURI__").unwrap();
        let dialog = Reflect::get(&tauri.into(), &"dialog".into()).unwrap();
        let open = Reflect::get(&dialog, &"open".into()).unwrap();
        let open_function = open.dyn_ref::<Function>().unwrap();

        let args = self.config.to_args();
        let js_args = serde_wasm_bindgen::to_value(&args).unwrap();

        let response = open_function.call1(open_function, &js_args).unwrap();
        let response_promise = response.dyn_into::<Promise>().unwrap();
        let response_future = JsFuture::from(response_promise);

        let callback = self.config.on_select.as_ref().map(Rc::clone);

        spawn_local(async move {
            let response_value = response_future.await.unwrap();

            let response_str = if let Some(response_str) = response_value.as_string() {
                Some(response_str)
            } else if response_value.is_null() {
                None
            } else {
                Result::<(), _>::Err(format!(
                    "expected file select response to be `string` or `null`, instead got: `{:?}`",
                    response_value
                ))
                .unwrap();
                unreachable!();
            };

            let response_path = response_str.map(PathBuf::from);

            if let Some(callback) = callback {
                (*callback)(response_path);
            }
        });
    }
}

/// Prompts the user to select a file from the file system.
#[hook]
pub fn use_file_select(config: FileSelectConfig) -> UseFileSelect {
    UseFileSelect { config }
}
