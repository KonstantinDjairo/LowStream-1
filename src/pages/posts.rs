// use futures::{prelude::*};
// use std::future::Future;
use serde::Deserialize;
// use yewtil::NeqAssign;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
// use yew_router::agent::{RouteAgentDispatcher, RouteRequest};

use crate::{
    switch::{AppAnchor, AppRoute},
    components::{carousel, view_content, view_ecchi, view_romance, view_shounen},
};

// const CAROUSEL_DELAY_MS: u64 = 15000;

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
    // dados: Vec<Content>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    animes: Vec<Anime>
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ReceiveResponse(Result<Struture, anyhow::Error>),
    Payload(String),
}

#[derive(Debug)]
pub struct LoadPosts {
    payload: String,
    debugged_payload: String,
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl LoadPosts {
    fn view_json(&self) -> Html {
        fn search(card_name: String, writing: String) -> bool
        {
                // let words_in_names: Vec<&str> = card_name.split_whitespace().collect::<Vec<&str>>();
                let chars_name: Vec<char> = card_name.chars().collect();
                let writing_chars: Vec<char> = writing.chars().collect();
                for (j, c) in writing_chars.iter().enumerate()
                {
                    if c != &chars_name[j]
                    {
                        return false;
                    }
                }
                true
        }
        let mut cards: Vec<Html> = Vec::new();
        let mut background: Vec<String> = Vec::new();
        let mut count = 0;
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes.len()
                {
                    if search(content.animes[i].anime.clone().to_lowercase(), self.debugged_payload.clone().to_lowercase()) && count < 9
                    {
                        count += 1;
                        background.push(content.animes[i].background.clone());
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Eps(i as u64)>
                                <a class="card-image" style=format!("background-image: url({});", content.animes[i].background.clone()) loading="lazy">
                                    <a class="card-description">
                                        <strong><h2>{content.animes[i].anime.clone()}</h2></strong>
                                        <p>{"Assistir agora"}</p>
                                    </a>
                                </a>
                            </AppAnchor>
                            </li>
                        })
                    }
                }

                html! {
                    <>
                        <carousel::Model background=self.export_background()/>
                        <section style="background-color: #25262F;">
                        <div class="mx-auto" style="width: 250px;">
                                <div class="field has-addons" style="padding-top: 80px;">
                                    <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Encontre seu anime"/>
                                </div>
                            </div>
                            // <h3>{"Resultados:"}</h3>
                            <ul class="card-list con-cards" >
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
                <carousel::Model background=vec!["https://jaqorbelize.com/wp-content/uploads/blur-1.png".to_string()]/>
                        <section style="background-color: #25262F;">
                        <div class="mx-auto" style="width: 250px;">
                            <div class="control is-loading field has-addons">
                                <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Carregando"/>
                            </div>
                            // <div class="field has-addons" style="padding-top: 80px;">
                            //     <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Encontre seu anime"/>
                            // </div>
                        </div>
                            <ul class="card-list" >
                                // {for cards.clone()}
                            </ul>
                            <view_content::Content/>
                            <view_ecchi::Ecchi/>
                            <view_shounen::Shounen/>
                        </section>
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

    fn export_background(&self) -> Vec<String>
    {
        let mut background: Vec<String> = Vec::new();
        match self.json
        {
            Some(ref content) => 
            {
                for i in 0..content.animes.len()
                {
                    background.push(content.animes[i].background.clone());
                }
            },
            None => {
                background.push("none".to_string())
            }
        }
        background
    }
}
impl Component for LoadPosts {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self {
            payload: String::default(),
            debugged_payload: format!("{}", "none"),
            fetch_task: None,
            json: None,
            link,
            error: None,
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
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/d8912a0733e758fbc89324b16b9cea44/raw/1a75d13cec7cbc1a13975af9f863223389b27674/cards.json")
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
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() }   
                <view_content::Content/>
                <view_ecchi::Ecchi    />
                <view_shounen::Shounen/> 
                <view_romance::Romance/>
                <view_shounen::Shounen/>
            </>
        }
    }
}