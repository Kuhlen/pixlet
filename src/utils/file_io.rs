use js_sys::{Array, ArrayBuffer, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, FileReader, Url};

/// Read a web_sys::File into a Vec<u8> using FileReader
pub async fn read_file_bytes(file: &web_sys::File) -> Result<Vec<u8>, String> {
    let reader = FileReader::new().map_err(|_| "Failed to create FileReader".to_string())?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let reader_clone = reader.clone();
        let onload = Closure::once(Box::new(move |_: web_sys::Event| {
            let result = reader_clone.result().unwrap();
            resolve.call1(&JsValue::NULL, &result).unwrap();
        }));
        let onerror = Closure::once(Box::new(move |_: web_sys::Event| {
            reject
                .call1(&JsValue::NULL, &JsValue::from_str("File read error"))
                .unwrap();
        }));
        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        reader.read_as_array_buffer(file).unwrap();
        onload.forget();
        onerror.forget();
    });

    let result = JsFuture::from(promise)
        .await
        .map_err(|e| format!("{e:?}"))?;
    let array_buffer: ArrayBuffer = result
        .dyn_into()
        .map_err(|_| "Not an ArrayBuffer".to_string())?;
    let uint8_array = Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

/// Create a Blob from bytes and trigger a file download
pub fn trigger_download(bytes: &[u8], filename: &str, mime_type: &str) -> Result<(), String> {
    let uint8_array = Uint8Array::from(bytes);
    let array = Array::new();
    array.push(&uint8_array.buffer());

    let opts = BlobPropertyBag::new();
    opts.set_type(mime_type);
    let blob = Blob::new_with_buffer_source_sequence_and_options(&array, &opts)
        .map_err(|_| "Failed to create Blob".to_string())?;

    let url =
        Url::create_object_url_with_blob(&blob).map_err(|_| "Failed to create URL".to_string())?;

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let anchor: web_sys::HtmlAnchorElement =
        document.create_element("a").unwrap().dyn_into().unwrap();
    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();

    let _ = Url::revoke_object_url(&url);
    Ok(())
}

/// Derive output filename by replacing extension
pub fn derive_output_filename(original: &str, new_ext: &str) -> String {
    match original.rsplit_once('.') {
        Some((stem, _)) => format!("{stem}.{new_ext}"),
        None => format!("{original}.{new_ext}"),
    }
}
