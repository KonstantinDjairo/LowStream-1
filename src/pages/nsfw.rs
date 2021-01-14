use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use crate::components::carousel;

#[derive(Deserialize, Debug, Clone)]
pub struct Hentai {
    background: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    nsfw: Vec<Hentai>
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ReceiveResponse(Result<Struture, anyhow::Error>),
}

pub struct Nsfw
{
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Nsfw
{
    fn export_img(&self) -> Vec<String>
    {
        let mut images: Vec<String> = Vec::new();
        match self.json
        {
            Some(ref content) => {
                for i in 0..content.nsfw.len()
                {
                    images.push(content.nsfw[i].background.clone());
                }
                images
            }
            None => {
                vec![String::from(" ")]
            }
        }
    }

    fn view_json(&self) -> Html {
        let mut background: Vec<Html> = Vec::new();
        match self.json
        {
            Some(ref content) => {
                for i in 0..content.nsfw.len()
                {
                    background.push(html!{
                        <img src=content.nsfw[i].background.clone()/>
                    });
                }
                html!{
                    <>
                        <carousel::Model background=self.export_img()/>
                        <div class="cover-image-header__overlay">
                            <div class="cover-image-header__rows">
                                <h2>
                                    {"🚧Em desenvolvimento🚧"}
                                </h2>
                            </div>
                        </div>
                        <div style="padding-top: 5pc;">
                            <section id="photos" >
                                { for background }
                            </section>
                        </div>
                    </>
                }
            }
            None => {
                html!{

                }
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { 
                <>
                <section class="hero is-medium is-dark is-bold ">
                            <div class="hero-body">
                                <div class="container">
                                    <h1 class="title" style="padding-top: 40px;">
                                        {"Loading..."}
                                    </h1>
                                </div>
                            </div>
                        </section>
                        <section style="background-color: #25262F;">
                            <ul class="card-list">
                                <h1>{"..."}</h1>
                            </ul>
                        </section>
                <div class="position-absolute top-90 start-50 translate-middle">
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

impl Component for Nsfw {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self{
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
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/716b7a3f3315c607ca26fda8fdfd6005/raw/55c4fa6d609778c57d8c0ac2b669053c18a30a1f/nsfw.json")
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

    fn view(&self) -> Html
    {
        html!{
            <>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() } 
            </>
        }
    }
}
