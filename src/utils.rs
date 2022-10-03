use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, HtmlTextAreaElement, UrlSearchParams, Window};

pub fn get_window() -> crate::Result<Window> {
    web_sys::window().ok_or_else(|| crate::Error::MissingElement("window node".to_owned()))
}

pub fn get_query_params() -> crate::Result<UrlSearchParams> {
    let search = get_window()?.location().search().unwrap();
    UrlSearchParams::new_with_str(&search)
        .map_err(|err| crate::Error::FailedToCreateUrlSearchParams(format!("{:?}", err)))
}

fn get_element(id: &str) -> crate::Result<Element> {
    get_window()?
        .document()
        .ok_or_else(|| crate::Error::MissingElement("document node".to_owned()))?
        .get_element_by_id(id)
        .ok_or_else(|| crate::Error::MissingElement(format!("element with id '{}'", id)))
}

pub fn get_text_area(id: &str) -> crate::Result<HtmlTextAreaElement> {
    get_element(id)?
        .dyn_into::<HtmlTextAreaElement>()
        .map_err(|err| {
            crate::Error::UnexpectedElement(format!("element is not an textarea: {:?}", err))
        })
}

pub fn get_input(id: &str) -> crate::Result<HtmlInputElement> {
    get_element(id)?
        .dyn_into::<HtmlInputElement>()
        .map_err(|err| {
            crate::Error::UnexpectedElement(format!("element is not an input: {:?}", err))
        })
}
