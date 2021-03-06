use crate::document::DocumentQuery;
use wasm_peers::get_random_session_id;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::utils::get_input;

pub(crate) enum HomeMsg {
    UpdateInput,
}

pub(crate) struct Home {
    input: String,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::UpdateInput => {
                self.input = get_input("join-input").value();
                true
            }
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
                        DocumentQuery::new(get_random_session_id().into_inner()),
                    )
                    .unwrap();
            })
        };
        let update_input = ctx
            .link()
            .callback(|_| HomeMsg::UpdateInput);
        let join_existing = {
            let session_id = self.input.clone();
            Callback::once(move |_| {
                if !session_id.is_empty() {
                    history
                        .push_with_query(Route::Document, DocumentQuery::new(session_id))
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
