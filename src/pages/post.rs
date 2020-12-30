use serde::Deserialize;
// use yewtil::NeqAssign;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use crate::{
    switch::{AppAnchor, AppRoute},
    pages::carousel
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
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct Eps
{
    name: u64,
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,

}

impl Eps {
    fn view_json(&self) -> Html {
        let mut cards: Vec<Html> = Vec::new();
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes[self.name as usize].dados[0].eps.len()
                {
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Player>
                                // <a class="card-image" style=format!("background-image: url({});", content.animes[self.name as usize].background.clone()) loading="lazy">
                                // </a>
                                <a class="card-description">
                                    <strong><h2>{content.animes[self.name as usize].dados[0].eps[i].name.clone()}</h2></strong>
                                </a>
                            </AppAnchor>
                            </li>
                        })
                }

                html! {
                    <>
                        <section class="hero is-medium is-dark is-bold has-background">
                            <img src=content.animes[self.name as usize].background.clone() class="hero-background is-transparent" style=" filter: blur(6px)"/>
                            <div class="hero-body">
                                <div class="container">
                                    <h1 class="title" style="padding-top: 40px;">
                                        {content.animes[self.name as usize].anime.clone()}
                                    </h1>
                                </div>
                            </div>
                        </section>
                        <section style="background-color: #25262F;">
                            <ul class="card-list">
                                {for cards.clone()}
                                <h1>{content.animes[self.name as usize].dados[0].eps[0].name.clone()}</h1>
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

impl Component for Eps {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self
        {
            link,
            name: props.id,
            fetch_task: None,
            json: None,
            error: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/6e15cea494cf0c0e263f7c1e91740b643e679913/tester.json")
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() }
            </>
        }
    }
}