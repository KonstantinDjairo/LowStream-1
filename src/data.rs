use serde::Deserialize;
use serde_json::{Value, Map};

use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Ep {
    name: String,
    // thumbnailLink: String,
    // modifiedTime: String,
    // size: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    // padrao: bool,
    // fonte: String,
    eps: Vec<Ep>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Anime {
    anime: String,
    dados: Vec<Content>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    animes: Vec<Anime>
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<Struture, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    fetch_task: Option<FetchTask>,
    iss: Option<Struture>,
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
                        <p>{ format!("Nome: {:?}", content.animes) }</p>
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
                // 1. build the request
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/10167657cd80761b14ab629916745b56205843d0/tester.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Struture, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
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
                // we want to redraw so that the page displays the location of the Struture instead of
                // 'fetching...'
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