use freya::events::MouseEvent;
use freya::prelude::*;

use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref TEXT: Mutex<String> = Mutex::new(String::from(""));
}

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_decorations(true)
            .with_transparency(false)
            .with_title("Editor")
            .with_width(1920.0)
            .with_height(1080.0)
            .build(),
    );
}

fn app() -> Element {
    use_init_theme(DARK_THEME);
    rsx!(Body {})
}

#[allow(non_snake_case)]
fn Body() -> Element {
    let theme = use_theme();
    let theme = theme.read();

    let mut editable = use_editable(
        || EditableConfig::new("And thus there was light ;)".to_string()),
        EditableMode::MultipleLinesSingleEditor,
    );
    let cursor_reference = editable.cursor_attr();
    let editor = editable.editor().read();

    let onclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };
    let mut text = TEXT.lock();
    *text = (*editor).to_string();

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "4",
            onkeydown,
            cursor_reference,
            direction: "horizontal",
            onglobalclick: onclick,
            background: "{theme.body.background}",
            VirtualScrollView {
                theme: theme_with!(ScrollViewTheme {
                    width: "100%".into(),
                }),
                length: editor.len_lines(),
                item_size: 20.0,
                scroll_with_arrows: false,
                cache_elements: false,
                builder: move |line_index, _: &Option<()>| {
                    let editor = editable.editor().read();
                    let line = editor.line(line_index).unwrap();

                    let is_line_selected = editor.cursor_row() == line_index;

                    // Only show the cursor in the active line
                    let character_index = if is_line_selected {
                        editor.cursor_col().to_string()
                    } else {
                        "none".to_string()
                    };

                    // Only highlight the active line
                    let line_background = if is_line_selected {
                        "rgb(37, 37, 37)"
                    } else {
                        ""
                    };

                    let onmousedown = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
                    };

                    let onmouseover = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseOver(e.data, line_index));
                    };

                    let highlights = editable.highlights_attr(line_index);

                    rsx! {
                        rect {
                            key: "{line_index}",
                            width: "100%",
                            height: "20",
                            direction: "horizontal",
                            background: "{line_background}",
                            rect {
                                width: "30",
                                height: "100%",
                                main_align: "center",
                                direction: "horizontal",
                                label {
                                    font_size: "17",
                                    color: "rgb(200, 200, 200)",
                                    "{line_index + 1} "
                                }
                            }
                            paragraph {
                                height: "100%",
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                max_lines: "1",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                onmousedown,
                                onmouseover,
                                highlights: highlights,
                                text {
                                    color: "rgb(240, 240, 240)",
                                    font_size: "18",
                                    "{line}"
                                }
                            }
                        }
                    }
                }
            }

        }
    )
}

// most of the features need to be added
// this is only a hackaround on an example code from freya repository
