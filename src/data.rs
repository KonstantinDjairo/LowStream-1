use serde::Deserialize;

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

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<Struture, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl FetchServiceExample {
    fn view_json(&self) -> Html {
        let mut names: Vec<String> = Vec::new();
        let mut background: Vec<String> = Vec::new();
        let mut eps: Vec<String> = Vec::new();
        let mut cards: Vec<Html> = Vec::new();
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes.len()
                {
                    // for j in 0..content.animes[i].anime.len()
                    {
                        names.push(String::from(format!("{}", content.animes[i].anime)));
                        background.push(String::from(format!("{}", content.animes[i].background)));
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Post(names[i].clone())>
                                <a class="card-image" style=format!("background-image: url({});", background[i].clone()) loading="lazy">
                                </a>
                                <a class="card-description">
                                    <strong><h2>{names[i].clone()}</h2></strong>
                                    <p>{"Assistir agora"}</p>
                                </a>
                            </AppAnchor>
                            </li>
                        });

                        // eps.push(String::from(format!("{}", content.animes[i].dados[0].eps[j].name)));
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
                            <button class="button is-dark is-rounded" onclick=self.link.callback(|_| Msg::GetLocation)>
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
            html! { <><div class="d-flex justify-content-center">
				<div class="spinner-border is-white" role="status">
					<span class="visually-hidden">{"Carregando os dados..."}</span>
				</div>
			</div></> }
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
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/d650d792c4d37d8f7f32f6c7be4c175d4075b42c/tester.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Struture, anyhow::Error>>>| {
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
                        self.json = Some(location);
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