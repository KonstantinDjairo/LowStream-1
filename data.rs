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
        let mut eps: Vec<String> = Vec::new();
        let mut cards: Vec<Html> = Vec::new();
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes.len()
                {
                    // for j in 0..content.animes[i].anime.len()
                    {
                        names.push(String::from(format!("{}", content.animes[i].anime)));
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Player>
                                <a class="card-image" style="background-image: url(https://scontent.fimp1-1.fna.fbcdn.net/v/t31.0-1/c190.0.720.720a/p720x720/10530506_597753610340284_4786237311158633188_o.png?_nc_cat=100&ccb=2&_nc_sid=dbb9e7&_nc_ohc=7oPqryuXIHQAX9WwyHC&_nc_ht=scontent.fimp1-1.fna&_nc_tp=30&oh=424afc064c3a98d52699f2b5b8c60665&oe=6005074F);">
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
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/10167657cd80761b14ab629916745b56205843d0/tester.json")
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