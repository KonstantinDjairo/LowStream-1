use serde::{Deserialize};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};


use crate::{
    // switch::{AppAnchor, AppRoute},
    components::{video, box_players},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Ep {
    name: String,
    player: String,
    type_video: String
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
    GetOption(usize),
    TogglePlay(String, String, String, String),
    Close,
    ViewElements(usize)
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
    number: usize,
    current_video: String,
    link_video: String,
    type_video: String,
    poster_video: String,
    play: Html,
    close: bool,
    change: usize
}

impl Eps {
    fn notification(&self) -> Html
    {
        if self.close
        {
            return html!{
                <>
                    <div class="notification is-danger is-light">
                            <strong>{"O player ainda é um test, por conta disso a sua lógica ainda está em processo de desenvolvimento. Para reproduzir o próximo episódio, feche o player abaixo primeiramente, e por conseguinte clique no card desejado. Obg por sua visita uwu"}</strong>
                    </div>
                    <div class="notification  is-dark">
                        <button class="delete" onclick=self.link.callback(|_| Msg::Close)></button>
                        <strong>{self.current_video.clone()}</strong>
                    </div>
                    {self.play.clone()}
                </>
            }
        }
        html!{}
    }
    fn view_json(&self) -> Html {
        let mut cards: Vec<Html> = Vec::new();
        let mut options: Vec<Html> = Vec::new();
        let mut buttons: Vec<Html> = Vec::new();
        let mut count: u64 = 0;
        match self.json {
            Some(ref content) => {
                let quantidade_de_eps = content.animes[self.name as usize].dados[self.number].eps.len();
                for j in 0..content.animes[self.name as usize].dados.len()
                {
                    options.push(html!{
                        <a class="navbar-item" onclick=self.link.callback(move |_| Msg::GetOption(j)) style="color: white">
                            {format!("nº {}", j + 1)}
                        </a>
                    });
                }

                if quantidade_de_eps.rem_euclid(25) == 0
                {
                    for j in 0..quantidade_de_eps.div_euclid(25)
                    {
                        buttons.push(html!{
                            <button class="button is-black" onclick=self.link.callback(move |_| Msg::ViewElements(j))>
                                {format!("{}", j + 1)}
                            </button>
                        });
                    }
                }
                else
                {
                    for j in 0..quantidade_de_eps.div_euclid(25) + 1
                    {
                        buttons.push(html!{
                            <button class="button is-black" onclick=self.link.callback(move |_| Msg::ViewElements(j))>
                                {format!("{}", j + 1)}
                            </button>
                        });
                    }
                    if buttons.len() == 1
                    {
                        buttons = vec![html!{}];
                    }
                }

                for i in self.change * 25..quantidade_de_eps
                {
                        let video = content.animes[self.name as usize].dados[self.number].eps[i].name.clone();
                        let player = content.animes[self.name as usize].dados[self.number].eps[i].player.clone();
                        let type_video = content.animes[self.name as usize].dados[self.number].eps[i].type_video.clone();
                        let poster_video = content.animes[self.name as usize].background.clone();
                        cards.push(html!{
                            <li style="background: black; min-width: auto">
                            <a onclick=self.link.callback(move |_| Msg::TogglePlay(video.clone(), player.clone(), type_video.clone(), poster_video.clone()))>
                                <a>
                                    <h1 class="text-in-square">{format!("{}", i + 1)}</h1>
                                    <strong><h2>{content.animes[self.name as usize].dados[self.number].eps[i].name.clone().replace(".mkv", " ").replace(".mp4", " ").replace(".avi", " ")}</h2></strong>
                                </a>
                                </a>
                            </li>
                        });
                        count += 1;
                        if count >= 25
                        {
                            break;
                        }
                }


                html! {
                    <>
                        <section class="hero is-small is-dark is-bold has-background">
                            <img src=content.animes[self.name as usize].background.clone() class="hero-background is-transparent" style=" filter: blur(6px)"/>
                            <div class="cover-image-header__animes">
                            <div class="cover-image-header__rows">
                            </div>
                        </div>
                            <div class="hero-body">
                                <div class="container">
                                    <h2 class="title" style="padding-top: 80px; text-shadow: 1px 1px #363636;">
                                        {content.animes[self.name as usize].anime.clone()}
                                    </h2>
                                    {self.notification()}
                                    <nav style="z-index: 1000">
                                        <div class="navbar-item has-dropdown is-hoverable" style="background-color: rgba(0, 0, 0, 10%); backdrop-filter: blur(10px); border-radius: 8px;">
                                            <a class="navbar-link" style="background-color: #36363600;color: white;">
                                                {"Opções"}
                                            </a>
                                            <div class="navbar-dropdown is-up is-boxed" style="background-color: rgba(0, 0, 0);">
                                            {for options.clone()}
                                            </div>
                                        </div>
                                    </nav>
                                </div>
                            </div>
                        </section>
                        <section style="background-color: #25262F; margin-top: 12pc">
                            <div class="con-cards">
                                {for buttons.clone()}
                            </div>
                            <ol class="gradient-list" style="margin-left: 30px; margin-right: 30px;">
                                {for cards.clone()}
                            </ol>
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
                <section class="hero is-small is-dark is-bold has-background">
                            <div class="cover-image-header__animes">
                            <div class="cover-image-header__rows">
                            </div>
                            </div>
                            <div class="hero-body">
                                <div class="container">
                                    <h1 class="title" style="padding-top: 15pc;">
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
            html! {}
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
            error: None,
            number: 0,
            current_video: String::new(),
            link_video: String::new(),
            type_video: String::new(),
            poster_video: String::new(),
            play: html!{},
            close: false,
            change: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            ViewElements(number) => {
                self.change = number;
                true
            }
            Close => {
                self.close = false;
                self.play = html!{};
                true
            }
            TogglePlay(name, link_video, type_video, poster_video) => {
                self.close = true;
                self.current_video = name;
                self.link_video = link_video;
                self.type_video = type_video;
                self.poster_video = poster_video;
                self.play = html!{
                    <div class="context has-text-centered" style="padding-top: 40px; padding-bottom: 140px;">
                        <video::Video video=self.link_video.clone() type_video=self.type_video.clone() poster=self.poster_video.clone()/>
                        <box_players::BoxPlayers link_video=self.link_video.clone()/>
                    </div>
                };
                
                true
            }
            GetOption(value) => {
                self.number = value;
                true
            }
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/e8505b942766876cd4e3229841532e84556b95dc/tester.json")
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