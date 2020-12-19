use serde::Deserialize;
use serde_json::{Value, Map};

use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    name: Map<String, Value>,
    // link: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Anime {
    animes:Content,
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<Anime, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    fetch_task: Option<FetchTask>,
    iss: Option<Anime>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl FetchServiceExample {
    fn view_iss_location(&self) -> Html {
        match self.iss {
            Some(ref content) => {
                html! {
                    <>
                        <p>{ "Animes:" }</p>
                        <p>{ format!("Nome: {:?}", content.animes.name) }</p>
                        // <p>{ format!("Link do video: {}", content.animes.link) }</p>
                    </>
                }
            }
            None => {
                html! {
                     <button onclick=self.link.callback(|_| Msg::GetLocation)>
                         { "Procurar dados *_*" }
                     </button>
                }
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Buscando dados..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}
impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            iss: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetLocation => {
                
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/40acb1ab78cd5d2fd1c594a137635c37/raw/d697f57bd6f4cc581dbcb5b69d517ba33277509c/fetching_links.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Anime, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.iss = Some(location);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                <div style="padding-top: 80px"></div>
                { self.view_fetching() }
                { self.view_iss_location() }
                { self.view_error() }
            </>
        }
    }
}
