use super::*;
use crate::hooks::*;
use std::path::PathBuf;
use yew::prelude::*;

/// The style of a file selection button.
pub type FileSelectStyle = ButtonStyle;

/// File selection properties.
#[derive(Properties, PartialEq, Clone)]
pub struct FileSelectProps {
    /// The file selection state.
    pub state: UseStateHandle<Option<PathBuf>>,
    /// The callback called when the state changes.
    #[prop_or_default]
    pub on_change: Callback<Option<PathBuf>>,
    /// The text on the file selection button.
    pub text: AttrValue,
    /// The button style.
    #[prop_or_default]
    pub style: FileSelectStyle,
    /// The path to start the file selection from.
    #[prop_or_default]
    pub start_path: Option<AttrValue>,
    /// The file selection dialog window title.
    #[prop_or_default]
    pub dialog_title: Option<AttrValue>,
    /// Whether to allow directory selection.
    #[prop_or(false)]
    pub directory: bool,
    /// Whether to allow selection of multiple files.
    #[prop_or(false)]
    pub multiple: bool,
    /// A list of acceptable file extensions. If empty, all files will be
    /// allowed.
    #[prop_or_default]
    pub extensions: Option<Vec<String>>,
    /// Whether the input is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A file selection component.
#[function_component]
pub fn FileSelect(props: &FileSelectProps) -> Html {
    let FileSelectProps {
        state,
        on_change,
        text,
        style,
        start_path,
        dialog_title,
        directory,
        multiple,
        extensions,
        disabled,
    } = props.clone();

    use_effect_with_deps(
        move |new_state| on_change.emit((**new_state).clone()),
        state.clone(),
    );

    let file_select = use_file_select(
        FileSelectConfig::new()
            .start_path(start_path.map(|s| s.to_string()))
            .dialog_title(dialog_title.map(|s| s.to_string()))
            .directory(directory)
            .multiple(multiple)
            .extensions(extensions)
            .on_select(move |path| {
                if path.is_some() && !disabled {
                    state.set(path);
                }
            }),
    );

    html! {
        <Button
            {text}
            {style}
            {disabled}
            on_click={move |_| file_select.open()}
        />
    }
}
