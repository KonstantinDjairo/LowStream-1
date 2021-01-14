use serde::Deserialize;
// use yewtil::NeqAssign;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use crate::{
    switch::{AppAnchor, AppRoute},
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
    background: String,
    dados: Vec<Content>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    animes: Vec<Anime>
}

// #[derive(Clone, Debug, PartialEq, Properties)]
// struct Props
// {
//     load: Callback(Result<Struture, anyhow::Error>)
// }

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ReceiveResponse(Result<Struture, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    // props: Props
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl FetchServiceExample {
    fn view_json(&self) -> Html {
        let mut names: Vec<String> = Vec::new();
        let mut background: Vec<String> = Vec::new();
        let mut cards: Vec<Html> = Vec::new();
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes.len()
                {
                    {
                        names.push(String::from(format!("{}", content.animes[i].anime)));
                        background.push(String::from(format!("{}", content.animes[i].background)));
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Eps(i as u64)>
                                <a class="card-image" style=format!("background-image: url({});", background[i].clone()) loading="lazy">
                                </a>
                                <a class="card-description">
                                    <strong><h2>{names[i].clone()}</h2></strong>
                                    <p>{"Assistir agora"}</p>
                                </a>
                            </AppAnchor>
                            </li>
                        });
                    }
                }

                html! {
                    <>
                        <section style="background-color: #25262F;">
                            <ul class="card-list">
                                {for cards.clone()}
                            </ul>
                        </section>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        <div class="has-text-centered" style="padding-top: 10px">
                            <button class="button is-dark is-rounded" onclick=self.link.callback(|_| Msg::GetInfo)>
                                { "Procurar dados *_*" }
                            </button>
                        </div>
                    </>
                }
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { 
                <>
				<div class="spinner-border is-white position-absolute top-50 start-50" role="status">
					<span class="visually-hidden">{"Carregando os dados..."}</span>
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
impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            json: None,
            link,
            error: None
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/8c326da70bccea06f488663e460a590ed47d1568/tester.json")
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