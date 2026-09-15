use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{DragEvent, File, HtmlInputElement};

use crate::api::file;
use crate::components::ui::{Button, Icon, icon};
use crate::core::convert;
use crate::core::format::OutputFormat;
use crate::core::source::SourceImage;
use crate::core::stage::ConversionStage;
use crate::error::AppError;

const ACCEPT: &str = "image/jpeg,image/png,image/webp,image/gif,image/bmp,image/tiff";

/// Decode and encode run synchronously on the main thread. `TimeoutFuture(0)`
/// lets the browser paint the progress bar before the heavy work.
async fn run(
    src: SourceImage,
    format: OutputFormat,
    quality: u8,
    stage: RwSignal<ConversionStage>,
) -> Result<(), AppError> {
    stage.set(ConversionStage::Decoding);
    TimeoutFuture::new(0).await;
    let img = convert::decode(&src.bytes)?;

    stage.set(ConversionStage::Encoding);
    TimeoutFuture::new(0).await;
    let out = convert::encode(&img, format, quality)?;
    file::download(&out, &src.output_name(format), format.mime_type())?;

    stage.set(ConversionStage::Done);
    TimeoutFuture::new(2_000).await;
    Ok(())
}

/// Drop zone, format picker, quality slider, convert button, progress, error.
#[component]
pub fn ConverterCard() -> impl IntoView {
    let format = RwSignal::new(OutputFormat::Png);
    let quality = RwSignal::new(90u8);
    let stage = RwSignal::new(ConversionStage::Idle);
    let source = RwSignal::new(Option::<SourceImage>::None);
    let preview_url = RwSignal::new(Option::<String>::None);
    let error = RwSignal::new(Option::<AppError>::None);
    let is_dragging = RwSignal::new(false);
    let file_input = NodeRef::<leptos::html::Input>::new();

    let is_converting = move || stage.get().is_running();
    let has_source = move || source.with(|s| s.is_some());
    let quality_applicable = move || format.get().supports_quality();

    let clear_source = move || {
        if let Some(url) = preview_url.get_untracked() {
            file::revoke_url(&url);
        }
        source.set(None);
        preview_url.set(None);
        error.set(None);
    };

    let select_file = move |f: File| {
        clear_source();
        if let Err(e) = SourceImage::check_supported(&f.name(), &f.type_()) {
            error.set(Some(e.into()));
            return;
        }
        preview_url.set(file::preview_url(&f));
        spawn_local(async move {
            match file::read_bytes(&f).await {
                Ok(bytes) => source.set(Some(SourceImage {
                    name: f.name(),
                    bytes,
                })),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    let on_input_change = move |ev: leptos::ev::Event| {
        let input = event_target::<HtmlInputElement>(&ev);
        if let Some(f) = input.files().and_then(|l| l.get(0)) {
            select_file(f);
        }
        // reset so picking the same file fires change again
        input.set_value("");
    };

    let on_remove = move |ev: leptos::ev::MouseEvent| {
        // parent click opens the file picker, stop it here
        ev.stop_propagation();
        clear_source();
    };

    let on_convert = move || {
        let Some(src) = source.get_untracked() else {
            return;
        };
        error.set(None);
        spawn_local(async move {
            if let Err(e) = run(src, format.get_untracked(), quality.get_untracked(), stage).await {
                error.set(Some(e));
            }
            stage.set(ConversionStage::Idle);
        });
    };

    view! {
        <div class="card-surface rounded-3xl shadow-2xl p-8 md:p-12 max-w-4xl mx-auto">
            <input
                node_ref=file_input
                type="file"
                accept=ACCEPT
                class="hidden"
                on:change=on_input_change
            />

            <div
                class="border-2 border-dashed rounded-2xl p-6 md:p-12 text-center transition-all cursor-pointer hover:border-amber-500 hover:bg-zinc-800/50"
                class:border-amber-500=is_dragging
                class=("bg-zinc-800/50", is_dragging)
                class:border-zinc-700=move || !is_dragging.get()
                on:dragover=move |ev: DragEvent| {
                    ev.prevent_default();
                    is_dragging.set(true);
                }
                on:dragleave=move |_| is_dragging.set(false)
                on:drop=move |ev: DragEvent| {
                    ev.prevent_default();
                    is_dragging.set(false);
                    let dropped = ev
                        .data_transfer()
                        .and_then(|dt| dt.files())
                        .and_then(|l| l.get(0));
                    if let Some(f) = dropped {
                        select_file(f);
                    }
                }
                on:click=move |_| {
                    if let Some(input) = file_input.get() {
                        input.click();
                    }
                }
            >
                <Show
                    when=has_source
                    fallback=|| {
                        view! {
                            <div class="float-animation">
                                <Icon
                                    d=icon::UPLOAD
                                    class="w-14 h-14 md:w-20 md:h-20 mx-auto text-amber-500 mb-4"
                                />
                            </div>
                            <h3 class="text-xl md:text-2xl font-semibold text-zinc-100 mb-2">
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
                    {move || {
                        preview_url
                            .get()
                            .map(|url| {
                                view! {
                                    <img
                                        src=url
                                        alt="Preview"
                                        class="max-h-40 rounded-lg mx-auto mb-3 object-contain"
                                    />
                                }
                            })
                    }}
                    <p class="text-lg text-zinc-300 font-medium">
                        {move || source.with(|s| s.as_ref().map(|s| s.name.clone()))}
                    </p>
                    <button
                        class="text-red-400 text-sm mt-2 hover:text-red-300 transition"
                        on:click=on_remove
                    >
                        "Remove"
                    </button>
                </Show>
            </div>

            <div class="mt-8">
                <p class="block text-lg font-semibold text-zinc-100 mb-4">"Convert to:"</p>
                <div class="grid grid-cols-3 md:grid-cols-6 gap-3">
                    {OutputFormat::ALL
                        .map(|f| {
                            view! {
                                <button
                                    class="bg-zinc-800 border-2 rounded-xl p-4 text-center font-semibold text-zinc-300 transition transform hover:scale-105 hover:border-amber-500"
                                    class:border-amber-500=move || format.get() == f
                                    class:border-zinc-700=move || format.get() != f
                                    aria-pressed=move || (format.get() == f).to_string()
                                    on:click=move |_| format.set(f)
                                >
                                    {f.label()}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            <div
                class="mt-8"
                class:opacity-40=move || !quality_applicable()
                class:pointer-events-none=move || !quality_applicable()
            >
                <label class="block text-lg font-semibold text-zinc-100 mb-3" for="quality-slider">
                    "Quality: "
                    {move || quality.get()}
                    "%"
                </label>
                <input
                    id="quality-slider"
                    type="range"
                    min="1"
                    max="100"
                    prop:value=move || quality.get()
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<u8>() {
                            quality.set(v);
                        }
                    }
                    class="w-full h-2 bg-zinc-700 rounded-lg appearance-none cursor-pointer accent-amber-500"
                />
            </div>

            <div class="mt-8 text-center">
                <Button
                    large=true
                    disabled=Signal::derive(move || is_converting() || !has_source())
                    on_click=move |_| on_convert()
                >
                    <span class="flex items-center justify-center">
                        <Icon d=icon::REFRESH class="w-6 h-6 mr-2" />
                        "Convert Image"
                    </span>
                </Button>
            </div>

            <Show when=is_converting>
                <div class="mt-6">
                    <div class="bg-zinc-700 rounded-full h-4 overflow-hidden">
                        <div
                            class="bg-amber-500 h-full transition-all duration-300"
                            style:width=move || format!("{}%", stage.get().progress())
                        ></div>
                    </div>
                    <p class="text-center text-zinc-400 mt-2">{move || stage.get().label()}</p>
                </div>
            </Show>

            <Show when=move || error.with(|e| e.is_some())>
                <div
                    role="alert"
                    class="mt-4 p-4 bg-red-900/50 border border-red-500 rounded-xl text-red-300 text-sm"
                >
                    {move || error.with(|e| e.as_ref().map(ToString::to_string))}
                </div>
            </Show>
        </div>
    }
}
