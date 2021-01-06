use serde::Deserialize;
// use yewtil::NeqAssign;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

// mod components;
use crate::{
    switch::{AppAnchor, AppRoute},
    pages::{carousel, video}
    // components::video
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
    TogglePlay(String, String, String)
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
    type_video: String
}

impl Eps {
    fn view_json(&self) -> Html {
        let mut cards: Vec<Html> = Vec::new();
        let mut options: Vec<Html> = Vec::new();
        // let mut video = &self.current_video;
        // let mut type_video
        match self.json {
            Some(ref content) => {
                
                for j in 0..content.animes[self.name as usize].dados.len()
                {
                    options.push(html!{
                        // <hr class="navbar-divider"/>
                        <a class="navbar-item" onclick=self.link.callback(move |_| Msg::GetOption(j)) style="color: white">
                            {format!("nº {}", j + 1)}
                        </a>
                    });
                }
                
                // for i in 0..content.animes[self.name as usize].dados.len()
                // {

                // }

                // for j in 0..content.animes[self.name as usize].dados.len()
                // {
                for i in 0..content.animes[self.name as usize].dados[self.number].eps.len()
                {
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Player(content.animes[self.name as usize].dados[self.number].eps[i].player.clone(), 
                                                              content.animes[self.name as usize].background.clone(), 
                                                              content.animes[self.name as usize].dados[self.number].eps[i].name.clone(), 
                                                              content.animes[self.name as usize].dados[self.number].eps[i].type_video.clone(),
                                                              )>
                            // <a onclick=self.link.callback(move |_| Msg::TogglePlay(content.animes[self.name as usize].dados[self.number].eps[i].name.clone(),
                            //                                                        content.animes[self.name as usize].dados[self.number].eps[i].player.clone(),
                            //                                                        content.animes[self.name as usize].dados[self.number].eps[i].type_video.clone()))>
                                // <a class="card-image" style=format!("background-image: url({});", content.animes[self.name as usize].background.clone()) loading="lazy">
                                // </a>
                                <a class="list">
                                    <strong><h2>{content.animes[self.name as usize].dados[self.number].eps[i].name.clone().replace(".mkv", " ").replace(".mp4", " ").replace(".avi", " ")}</h2></strong>
                                </a>
                                // </a>
                            </AppAnchor>

                            </li>
                        });
                }
                // }


                html! {
                    <>
                        <section class="hero is-small is-dark is-bold has-background">
                            <img src=content.animes[self.name as usize].background.clone() class="hero-background is-transparent" style=" filter: blur(6px)"/>
                            <div class="hero-body">
                                <div class="container">
                                    <h2 class="title" style="padding-top: 80px;">
                                        {content.animes[self.name as usize].anime.clone()}
                                    </h2>
                                    // <video::Video video=&self.current_video, 
                                    //             type_video=&self.type_video />
                                    <nav style="z-index: 1000">
                                        <div class="navbar-item has-dropdown is-hoverable" style="background-color: rgba(0, 0, 0, 0%); backdrop-filter: blur(10px); border-radius: 8px;">
                                            <a class="navbar-link" style="background-color: rgba(0, 0, 0, 0%); backdrop-filter: blur(10px); border-radius: 18px; color: white;">
                                                {"Opções"}
                                            </a>
                                            <div class="navbar-dropdown is-up is-boxed" style="background-color: rgba(0, 0, 0);">
                                            {for options.clone()}
                                            //    <a class="navbar-item" href="https://github.com/lowstream-community/LowStream" style="color: white">
                                            //        {"GitHub"}
                                            //    </a>
                                            </div>
                                        </div>
                                    </nav>
                                </div>
                            </div>
                        </section>
                        <section style="background-color: #25262F; margin-top: 12pc">
                            <ul class="card-list">
                                {for cards.clone()}
                            </ul>
                            // <h2 style="padding-bottom: 400px">{"uwu"}</h2>
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
                <section class="hero is-medium is-dark is-bold ">
                            // <img src="" class="hero-background is-transparent" style=""/>
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
                                // {for cards.clone()}
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
            current_video: String::from("none"),
            link_video: String::from("none"),
            type_video: String::from("none")
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            TogglePlay(name, link_video, type_video) => {
                self.current_video = name;
                self.link_video = link_video;
                self.type_video = type_video;
                true
            }
            GetOption(value) => {
                self.number = value;
                true
            }
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/605110cd84788c1ac0e74af7ef94ec5a16c197ec/tester.json")
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

// pub struct Playlist
// {
//     links: Vec<String>,
//     names: Vec<String>
// }

// impl Component for Playlist
// {
//     type Message = Msg;
//     type Properties = Props;

//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self
//         {
//             links: Eps.json.clone(),
//             names: Vec::new()
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         true
//     }

//     fn view(&self) -> Html {
//         html! {
//             <>
                
//             </>
//         }
//     }
// }