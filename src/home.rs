use wasm_peers::get_random_session_id;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::document::Query;
use crate::utils::get_input;
use crate::Route;

pub enum Msg {
    UpdateInput,
}

pub struct Home {
    input: String,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::UpdateInput => match get_input("join-input") {
                Ok(input) => {
                    self.input = input.value();
                    true
                }
                Err(err) => {
                    eprintln!("failed to get input: {err}");
                    false
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        let start_as_host = {
            let history = history.clone();
            Callback::once(move |_| {
                history
                    .push_with_query(
                        Route::Document,
                        Query::new(get_random_session_id().into_inner()),
                    )
                    .unwrap();
            })
        };
        let update_input = ctx.link().callback(|_| Msg::UpdateInput);
        let join_existing = {
            let session_id = self.input.clone();
            Callback::once(move |_| {
                if !session_id.is_empty() {
                    history
                        .push_with_query(Route::Document, Query::new(session_id))
                        .unwrap();
                }
            })
        };
        html! {
                <div class="cover-container d-flex w-100 h-100 p-3 mx-auto flex-column">
                    <header class="mb-auto">
                        <div>
                            <h3 class="float-md-start mb-0">{ "Live Document" }</h3>
                        </div>
                    </header>

                    <main class="px-3">
                        <h1>{ "Live Document" }</h1>
                        <p class="lead">{ "A shared document application akin to Google Docs." }</p>
                        <p class="lead">{ "Create new document, or join existing one." }</p>
                        <p class="lead">{ "Document lives as long as somebody is in session." }</p>
                        <hr />
                        <p class="lead">
                            <button onclick={ start_as_host } class="btn btn-lg btn-secondary fw-bold border-white bg-white">{ "Start as host" }</button>
                        </p>
                        <p class="lead">{ "or join existing document" }</p>
                        <p class="lead">
                        <input id="join-input"
                            placeholder={ "Session id from a friend" }
                            oninput={ update_input }
                        />
                        </p>
                        <p class="lead">
                            <button onclick={ join_existing } class="btn btn-lg btn-secondary fw-bold border-white bg-white">{ "Join existing" }</button>
                        </p>
                    </main>

                    <footer class="mt-auto text-white-50">
                        <p>{ "Style based on Cover Bootstrap example." }</p>
                    </footer>
                </div>
        }
    }
}
