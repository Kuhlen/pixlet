use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::conversion::{decode, encode, file_io, formats::OutputFormat};

/// Main converter card with drop zone, format selector, quality slider, and convert button
#[component]
pub fn ConverterCardSnippet() -> impl IntoView {
    let (selected_format, set_selected_format) = signal("png".to_string());
    let (quality, set_quality) = signal(90i32);
    let (is_converting, set_is_converting) = signal(false);
    let (progress, set_progress) = signal(0i32);
    let (conversion_stage, set_conversion_stage) = signal("idle".to_string());

    // File state - store bytes (Send+Sync) instead of web_sys::File
    let (file_bytes, set_file_bytes) = signal(Option::<Vec<u8>>::None);
    let (file_name, set_file_name) = signal(String::new());
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (is_dragging, set_is_dragging) = signal(false);
    let (preview_url, set_preview_url) = signal(Option::<String>::None);

    let file_input_ref = NodeRef::<leptos::html::Input>::new();

    let quality_applicable = Memo::new(move |_| {
        let fmt = selected_format.get();
        matches!(fmt.as_str(), "jpg")
    });

    let format_options = vec![
        ("jpg", "JPG"),
        ("png", "PNG"),
        ("webp", "WebP"),
        ("gif", "GIF"),
        ("bmp", "BMP"),
        ("tiff", "TIFF"),
    ];

    // Handle file selection: validate, create preview, read bytes async
    let handle_file = move |file: web_sys::File| {
        // Revoke old preview URL
        if let Some(old_url) = preview_url.get_untracked() {
            let _ = web_sys::Url::revoke_object_url(&old_url);
        }

        let name = file.name();
        let file_type = file.type_();

        // Validate it's an image
        if !file_type.starts_with("image/") && !name.ends_with(".bmp") && !name.ends_with(".tiff")
        {
            set_error_message.set(Some("Please select a valid image file".to_string()));
            return;
        }

        // Create preview URL from the File object
        let url = web_sys::Url::create_object_url_with_blob(&file).ok();
        set_file_name.set(name);
        set_preview_url.set(url);
        set_error_message.set(None);

        // Read file bytes asynchronously
        spawn_local(async move {
            match file_io::read_file_bytes(&file).await {
                Ok(bytes) => set_file_bytes.set(Some(bytes)),
                Err(e) => {
                    set_error_message.set(Some(format!("Failed to read file: {e}")));
                    set_file_bytes.set(None);
                }
            }
        });
    };

    let clear_file = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        if let Some(url) = preview_url.get_untracked() {
            let _ = web_sys::Url::revoke_object_url(&url);
        }
        set_file_bytes.set(None);
        set_file_name.set(String::new());
        set_preview_url.set(None);
        set_error_message.set(None);
    };

    let on_convert = move |_| {
        let bytes = match file_bytes.get_untracked() {
            Some(b) => b,
            None => {
                set_error_message.set(Some("Please select a file first".to_string()));
                return;
            }
        };
        let format_str = selected_format.get_untracked();
        let q = quality.get_untracked() as u8;
        let original_name = file_name.get_untracked();

        set_is_converting.set(true);
        set_error_message.set(None);
        set_progress.set(0);

        spawn_local(async move {
            // Step 1: Decode
            set_progress.set(30);
            set_conversion_stage.set("decoding".to_string());
            gloo_timers::future::TimeoutFuture::new(0).await;

            let img = match decode::decode_image(&bytes) {
                Ok(img) => img,
                Err(e) => {
                    set_error_message.set(Some(e));
                    set_is_converting.set(false);
                    set_conversion_stage.set("idle".to_string());
                    return;
                }
            };

            // Step 2: Encode
            set_progress.set(60);
            set_conversion_stage.set("encoding".to_string());
            gloo_timers::future::TimeoutFuture::new(0).await;

            let format = match OutputFormat::from_str(&format_str) {
                Some(f) => f,
                None => {
                    set_error_message.set(Some("Unknown format".to_string()));
                    set_is_converting.set(false);
                    set_conversion_stage.set("idle".to_string());
                    return;
                }
            };

            match encode::encode_image(&img, format, q) {
                Ok(output_bytes) => {
                    let out_name =
                        file_io::derive_output_filename(&original_name, format.extension());
                    if let Err(e) =
                        file_io::trigger_download(&output_bytes, &out_name, format.mime_type())
                    {
                        set_error_message.set(Some(e));
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(e));
                    set_is_converting.set(false);
                    set_conversion_stage.set("idle".to_string());
                    return;
                }
            }

            // Done
            set_progress.set(100);
            set_conversion_stage.set("done".to_string());

            gloo_timers::future::TimeoutFuture::new(2000).await;
            set_is_converting.set(false);
            set_progress.set(0);
            set_conversion_stage.set("idle".to_string());
        });
    };

    view! {
        <div class="card-surface rounded-3xl shadow-2xl p-8 md:p-12 max-w-4xl mx-auto">
            // Hidden file input
            <input
                node_ref=file_input_ref
                type="file"
                accept="image/jpeg,image/png,image/webp,image/gif,image/bmp,image/tiff"
                style="display: none"
                on:change=move |ev| {
                    let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
                    if let Some(files) = input.files() {
                        if let Some(file) = files.get(0) {
                            handle_file(file);
                        }
                    }
                    input.set_value("");
                }
            />

            // Upload Area
            <div
                class=move || {
                    let base = "border-2 border-dashed rounded-2xl p-12 text-center transition-all cursor-pointer";
                    if is_dragging.get() {
                        format!("{base} border-amber-500 bg-zinc-800/50")
                    } else {
                        format!("{base} border-zinc-700 hover:border-amber-500 hover:bg-zinc-800/50")
                    }
                }
                on:dragover=move |ev: web_sys::DragEvent| {
                    ev.prevent_default();
                    set_is_dragging.set(true);
                }
                on:dragleave=move |_: web_sys::DragEvent| {
                    set_is_dragging.set(false);
                }
                on:drop=move |ev: web_sys::DragEvent| {
                    ev.prevent_default();
                    set_is_dragging.set(false);
                    if let Some(dt) = ev.data_transfer() {
                        if let Some(files) = dt.files() {
                            if let Some(file) = files.get(0) {
                                handle_file(file);
                            }
                        }
                    }
                }
                on:click=move |_| {
                    if let Some(input) = file_input_ref.get() {
                        let _ = input.click();
                    }
                }
            >
                <Show
                    when=move || file_bytes.get().is_some()
                    fallback=move || {
                        view! {
                            <div class="float-animation">
                                <svg
                                    class="w-20 h-20 mx-auto text-amber-500 mb-4"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-semibold text-zinc-100 mb-2">
                                "Drop your images here"
                            </h3>
                            <p class="text-zinc-400 mb-4">"or click to browse"</p>
                            <span class="inline-block bg-amber-500 text-zinc-950 px-8 py-3 rounded-full font-semibold hover:bg-amber-400 transition transform hover:scale-105">
                                "Choose Files"
                            </span>
                            <p class="text-sm text-zinc-500 mt-4">
                                "Supports: JPG, PNG, WebP, GIF, BMP, TIFF"
                            </p>
                        }
                    }
                >
                    // Preview when file is selected
                    {move || {
                        preview_url
                            .get()
                            .map(|url| {
                                view! {
                                    <img
                                        src=url
                                        class="max-h-40 rounded-lg mx-auto mb-3 object-contain"
                                    />
                                }
                            })
                    }}
                    <p class="text-lg text-zinc-300 font-medium">{move || file_name.get()}</p>
                    <button
                        class="text-red-400 text-sm mt-2 hover:text-red-300 transition"
                        on:click=clear_file
                    >
                        "Remove"
                    </button>
                </Show>
            </div>

            // Format Selection
            <div class="mt-8">
                <label class="block text-lg font-semibold text-zinc-100 mb-4">"Convert to:"</label>
                <div class="grid grid-cols-3 md:grid-cols-6 gap-3">
                    {format_options
                        .into_iter()
                        .map(|(value, label)| {
                            let value = value.to_string();
                            let value_clone = value.clone();
                            view! {
                                <button
                                    class=move || {
                                        let base = "bg-zinc-800 border-2 rounded-xl p-4 text-center transition transform hover:scale-105 hover:border-amber-500";
                                        if selected_format.get() == value_clone {
                                            format!("{base} border-amber-500")
                                        } else {
                                            format!("{base} border-zinc-700")
                                        }
                                    }
                                    on:click=move |_| set_selected_format.set(value.clone())
                                >
                                    <div class="font-semibold text-zinc-300">{label}</div>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            // Quality Slider
            <div class=move || {
                if quality_applicable.get() {
                    "mt-8".to_string()
                } else {
                    "mt-8 opacity-40 pointer-events-none".to_string()
                }
            }>
                <label class="block text-lg font-semibold text-zinc-100 mb-3">
                    "Quality: " {move || quality.get()} "%"
                </label>
                <input
                    type="range"
                    min="1"
                    max="100"
                    prop:value=move || quality.get()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                            set_quality.set(val);
                        }
                    }
                    class="w-full h-2 bg-zinc-700 rounded-lg appearance-none cursor-pointer accent-amber-500"
                />
            </div>

            // Convert Button
            <div class="mt-8 text-center">
                <button
                    class="bg-amber-500 text-zinc-950 px-12 py-4 rounded-full text-lg font-bold hover:bg-amber-400 transition transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled=move || is_converting.get() || file_bytes.get().is_none()
                    on:click=on_convert
                >
                    <span class="flex items-center justify-center">
                        <svg
                            class="w-6 h-6 mr-2"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                            ></path>
                        </svg>
                        "Convert Image"
                    </span>
                </button>
            </div>

            // Progress Bar
            <Show when=move || is_converting.get()>
                <div class="mt-6">
                    <div class="bg-zinc-700 rounded-full h-4 overflow-hidden">
                        <div
                            class="bg-amber-500 h-full transition-all duration-300"
                            style=move || format!("width: {}%", progress.get())
                        ></div>
                    </div>
                    <p class="text-center text-zinc-400 mt-2">
                        {move || {
                            match conversion_stage.get().as_str() {
                                "decoding" => "Decoding image...".to_string(),
                                "encoding" => "Encoding to target format...".to_string(),
                                "done" => "Conversion complete! Downloading...".to_string(),
                                _ => format!("Processing... {}%", progress.get()),
                            }
                        }}
                    </p>
                </div>
            </Show>

            // Error Display
            <Show when=move || error_message.get().is_some()>
                <div class="mt-4 p-4 bg-red-900/50 border border-red-500 rounded-xl text-red-300 text-sm">
                    {move || error_message.get().unwrap_or_default()}
                </div>
            </Show>
        </div>
    }
}
