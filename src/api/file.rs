use js_sys::{Array, Uint8Array};
use leptos::prelude::document;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, File, HtmlAnchorElement, Url};

use crate::error::AppError;

fn js_message(e: JsValue) -> String {
    e.as_string().unwrap_or_else(|| format!("{e:?}"))
}

pub async fn read_bytes(file: &File) -> Result<Vec<u8>, AppError> {
    let buf = JsFuture::from(file.array_buffer())
        .await
        .map_err(|e| AppError::FileRead(js_message(e)))?;
    Ok(Uint8Array::new(&buf).to_vec())
}

/// Object URL for `<img src>`. Caller must `revoke_url` when done.
pub fn preview_url(file: &File) -> Option<String> {
    Url::create_object_url_with_blob(file).ok()
}

pub fn revoke_url(url: &str) {
    let _ = Url::revoke_object_url(url);
}

/// Download via a synthetic `<a download>`. Never attached to the DOM, clicked directly.
pub fn download(bytes: &[u8], filename: &str, mime_type: &str) -> Result<(), AppError> {
    let parts = Array::of1(&Uint8Array::from(bytes));
    let opts = BlobPropertyBag::new();
    opts.set_type(mime_type);
    let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &opts)
        .map_err(|e| AppError::Download(js_message(e)))?;
    let url =
        Url::create_object_url_with_blob(&blob).map_err(|e| AppError::Download(js_message(e)))?;

    let anchor: HtmlAnchorElement = document()
        .create_element("a")
        .map_err(|e| AppError::Download(js_message(e)))?
        .unchecked_into();
    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();

    revoke_url(&url);
    Ok(())
}
