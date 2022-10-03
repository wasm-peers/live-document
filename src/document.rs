use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use wasm_peers::many_to_many::NetworkManager;
use wasm_peers::{get_random_session_id, ConnectionType, SessionId};
use yew::{html, Component, Context, Html};

use crate::utils::get_window;
use crate::{utils, CONFIG};

pub enum Msg {
    UpdateValue,
}

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub session_id: String,
}

impl Query {
    pub const fn new(session_id: String) -> Self {
        Self { session_id }
    }
}

pub struct Document {
    session_id: SessionId,
    network_manager: NetworkManager,
    is_ready: Rc<RefCell<bool>>,
}

const TEXTAREA_ID: &str = "document-textarea";

impl Component for Document {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let query_params = utils::get_query_params().expect("failed to get query params, aborting");
        let session_id = query_params.get("session_id").map_or_else(
            || {
                let location = get_window().expect("failed to get a window").location();
                let generated_session_id = get_random_session_id();
                query_params.append("session_id", generated_session_id.as_str());
                let search: String = query_params.to_string().into();
                location.set_search(&search).unwrap();
                generated_session_id
            },
            SessionId::new,
        );

        let is_ready = Rc::new(RefCell::new(false));
        let connection_type = ConnectionType::StunAndTurn {
            stun_urls: CONFIG.stun_server_urls.clone(),
            turn_urls: CONFIG.turn_server_urls.clone(),
            username: CONFIG.turn_server_username.clone(),
            credential: CONFIG.turn_server_credential.clone(),
        };
        let mut network_manager = NetworkManager::new(
            &format!("{}/{}", CONFIG.signaling_server_url, "many-to-many"),
            session_id.clone(),
            connection_type,
        )
        .unwrap();
        let on_open_callback = {
            let mini_server = network_manager.clone();
            let is_ready = Rc::clone(&is_ready);
            move |user_id| {
                let text_area = match utils::get_text_area(TEXTAREA_ID) {
                    Ok(text_area) => text_area,
                    Err(err) => {
                        log::error!("failed to get textarea: {:#?}", err);
                        return;
                    }
                };
                if !*is_ready.borrow() {
                    text_area.set_disabled(false);
                    text_area.set_placeholder(
                        "This is a live document shared with other users.\nWhat you write will be \
                         visible to everyone.",
                    );
                    *is_ready.borrow_mut() = true;
                }
                let value = text_area.value();
                if !value.is_empty() {
                    mini_server
                        .send_message(user_id, &value)
                        .expect("failed to send current input to new connection");
                }
            }
        };
        let on_message_callback = {
            move |_, message: String| match utils::get_text_area(TEXTAREA_ID) {
                Ok(text_area) => {
                    text_area.set_value(&message);
                }
                Err(err) => {
                    log::error!("failed to get textarea: {:#?}", err);
                }
            }
        };
        network_manager.start(on_open_callback, on_message_callback);
        Self {
            session_id,
            network_manager,
            is_ready,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::UpdateValue => match utils::get_text_area(TEXTAREA_ID) {
                Ok(text_area) => {
                    self.network_manager.send_message_to_all(&text_area.value());
                    true
                }
                Err(err) => {
                    log::error!("failed to get textarea: {:#?}", err);
                    false
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|_| Self::Message::UpdateValue);
        let disabled = !*self.is_ready.borrow();
        let placeholder = "This is a live document shared with other users.\nYou will be allowed \
                           to write once other join, or your connection is established.";
        html! {
            <main class="px-3">
                <p class="lead"> { "Share session id: " } <span class="line">{ &self.session_id }</span> </p>
                <p class="lead"> { "or just copy the page url." } </p>
                <textarea id={ TEXTAREA_ID } class="document" cols="100" rows="30" { disabled } { placeholder } { oninput }/>
            </main>
        }
    }
}
