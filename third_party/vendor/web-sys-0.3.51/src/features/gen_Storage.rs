#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Storage , typescript_type = "Storage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Storage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub type Storage;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Storage" , js_name = length)]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn length(this: &Storage) -> Result<u32, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Storage" , js_name = clear)]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn clear(this: &Storage) -> Result<(), JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Storage" , js_name = getItem)]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn get_item(this: &Storage, key: &str) -> Result<Option<String>, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Storage" , js_name = key)]
    #[doc = "The `key()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn key(this: &Storage, index: u32) -> Result<Option<String>, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Storage" , js_name = removeItem)]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn remove_item(this: &Storage, key: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Storage" , js_name = setItem)]
    #[doc = "The `setItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn set_item(this: &Storage, key: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class = "Storage", indexing_getter)]
    #[doc = "Indexing getter."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn get(this: &Storage, key: &str) -> Result<Option<String>, JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class = "Storage", indexing_setter)]
    #[doc = "Indexing setter."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn set(this: &Storage, key: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class = "Storage", indexing_deleter)]
    #[doc = "Indexing deleter."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
    pub fn delete(this: &Storage, key: &str) -> Result<(), JsValue>;
}
