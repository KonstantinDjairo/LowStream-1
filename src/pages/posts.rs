use serde::Deserialize;
use yewtil::NeqAssign;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = std::u64::MAX / ITEMS_PER_PAGE;

use crate::{
    // components::pagination::Pagination,
    switch::{AppAnchor, AppRoute},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Ep {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    eps: Vec<Ep>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Anime {
    anime: String,
    background: String,
    dados: Vec<Content>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    animes: Vec<Anime>
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ReceiveResponse(Result<Struture, anyhow::Error>),
    ShowPage(u64),
    Payload(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
}

#[derive(Debug)]
pub struct LoadPosts {
    payload: String,
    debugged_payload: String,
    props: Props,
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
    route_dispatcher: RouteAgentDispatcher,
}

impl LoadPosts {
    fn view_json(&self) -> Html {
        fn search(card_name: String, writing: String) -> bool
        {
            let chars_name: Vec<char> = card_name.chars().collect();
            let writing_chars: Vec<char> = writing.chars().collect();
            let mut word: String = String::default();
            for (i, c) in writing_chars.iter().enumerate()
            {
                if c == &chars_name[i]
                {
                    word = chars_name.iter().collect::<String>();
                }
                else
                {
                    return false;
                }
            }
            true
        }
        let mut cards: Vec<Html> = Vec::new();
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes.len()
                {
                    if search(content.animes[i].anime.clone().to_lowercase(), self.debugged_payload.clone().to_lowercase())
                    {
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Player>
                                <a class="card-image" style=format!("background-image: url({});", content.animes[i].background.clone()) loading="lazy">
                                </a>
                                <a class="card-description">
                                    <strong><h2>{content.animes[i].anime.clone()}</h2></strong>
                                    <p>{"Assistir agora"}</p>
                                </a>
                            </AppAnchor>
                            </li>
                            // <h5>{format!("{}", content.animes[i].dados[0].eps[i].name)}</h5>
                        })
                    }
                }

                html! {
                    <>
                        <section style="background-color: #25262F;">
                            <div class="level-item" style="padding-top: 80px;">
                                <div class="field has-addons">
                                    <input class="input is-rounded is-fixed-top" type="text"  oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Encontre seu anime"/>
                                </div>
                            </div>
                            <ul class="card-list">
                                {for cards.clone()}
                            </ul>
                        </section>
                    </>
                }
            }
            None => {
                html! {}
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { 
                <>
                <div class="position-absolute top-50 start-50 translate-middle">
                    <div class="d-flex justify-content-center">
                        <div class="spinner-border is-white" role="status">
                            <span class="visually-hidden">{"Loading..."}</span>
                        </div>
                    </div>
                </div>
                </> 
            }
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
impl Component for LoadPosts {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self {
            payload: String::default(),
            debugged_payload: format!("{}", "none"),
            props,
            fetch_task: None,
            json: None,
            link,
            error: None,
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            Msg::Payload(payload) => {
                if payload != self.payload {
                    self.debugged_payload = format!("{}", payload);
                    if self.debugged_payload == ""
                    {
                        self.debugged_payload = format!("{}", "none");
                    }
                    self.payload = payload;
                    true
                } else {
                    false
                }
            }
            Msg::ShowPage(page) => {
                let route = AppRoute::PostListPage(page);
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(route.into_route()));
                false
            }
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/e0b63616f2ff394ca7b75163e23325f83f4a0425/tester.json")
                    .body(Nothing)
                    .expect("Não foi possível efetuar o request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Struture, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("Falha ao iniciar o request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(dados) => {
                        self.json = Some(dados);
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
                { self.view_json() }
                { self.view_error() }
                
            </>
        }
    }
}