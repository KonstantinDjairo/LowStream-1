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
    TogglePlay(String, String, String),
    Close
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
    play: Html,
    close: bool
}

impl Eps {
    fn notification(&self) -> Html
    {
        if self.close
        {
            return html!{
                <>
                
                <div class="notification is-danger is-light">
                    // <button class="delete" onclick=self.link.callback(|_| Msg::Close)></button>
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
                        let video = content.animes[self.name as usize].dados[self.number].eps[i].name.clone();
                        let player = content.animes[self.name as usize].dados[self.number].eps[i].player.clone();
                        let type_video = content.animes[self.name as usize].dados[self.number].eps[i].type_video.clone();
                        cards.push(html!{
                            <li class="card" style="background: black; min-width: auto">
                            // <AppAnchor route=AppRoute::Player(content.animes[self.name as usize].dados[self.number].eps[i].player.clone(), 
                            //                                   content.animes[self.name as usize].background.clone(), 
                            //                                   content.animes[self.name as usize].dados[self.number].eps[i].name.clone(), 
                            //                                   content.animes[self.name as usize].dados[self.number].eps[i].type_video.clone(),
                            //                                   )>
                            <a onclick=self.link.callback(move |_| Msg::TogglePlay(video.clone(), player.clone(), type_video.clone()))>
                                // <a class="card-image" style=format!("background-image: url({});", content.animes[self.name as usize].background.clone()) loading="lazy">
                                // </a>
                                <a class="list">
                                    <strong><h2>{content.animes[self.name as usize].dados[self.number].eps[i].name.clone().replace(".mkv", " ").replace(".mp4", " ").replace(".avi", " ")}</h2></strong>
                                </a>
                                </a>
                            // </AppAnchor>

                            </li>
                        });
                }
                // }


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
                                    <h2 class="title" style="padding-top: 80px;">
                                        {content.animes[self.name as usize].anime.clone()}
                                    </h2>
                                    {self.notification()}
                                    <nav style="z-index: 1000">
                                        <div class="navbar-item has-dropdown is-hoverable" style="background-color: rgba(0, 0, 0, 0%); backdrop-filter: blur(10px); border-radius: 8px;">
                                            <a class="navbar-link" style="background-color: #36363626; backdrop-filter: blur(10px); border-radius: 18px; color: white;">
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
            current_video: String::new(),
            link_video: String::new(),
            type_video: String::new(),
            play: html!{},
            close: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            Close => {
                self.close = false;
                self.play = html!{};
                true
            }
            TogglePlay(name, link_video, type_video) => {
                self.close = true;
                self.current_video = name;
                self.link_video = link_video;
                self.type_video = type_video;
                self.play = html!{
                    <div class="context has-text-centered" style="padding-top: 40px; padding-bottom: 140px;">
                        <video controls=true autoplay="" width="100%" height="550" style="border-radius: 18px; box-shadow: 0px 0px 18px rgba(0, 0, 0, 70%)">
                            <source src=self.link_video.clone() type=self.type_video.clone()/>
                        </video>
                        <div class="notransition" style="background-color: rgba(0, 0, 0, 50%); color: white; display: inline-block;
                                    width: 90%;
                                    font-size: 1rem;
                                    text-decoration: none;
                                    overflow: hidden;
                                    box-shadow: 0 0 4rem -1rem rgba(0, 0, 0, 1);
                                    border-radius: 18px;">
                                        <header class="card-header">
                                            <p class="card-header-title" style="color: white"><span class="icon"><i aria-hidden="true" class="fa fa-play-circle"></i></span>{" Play in android "}<span class="icon"></span></p>
                                        </header>
                                        <div class="card-content">
                                            <div >
                                                <div class="columns is-mobile is-multiline has-text-centered">
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("iina://weblink?url={}",self.link_video.clone())>
                                                                <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/iina.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"IINA"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("potplayer://{}",self.link_video.clone())>
                                                                <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/potplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"PotPlayer"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("vlc://{}",self.link_video.clone())>
                                                                <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/vlc.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"VLC"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("thunder://{}",self.link_video.clone())>
                                                                <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/thunder.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"Thunder"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="javascript:alert(&quot;暂未实现&quot;)">
                                                                <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/aria2.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"Aria2"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("nplayer-{}",self.link_video.clone())>
                                                                <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/nplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"nPlayer"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.ad;S.title=undefined;end",self.link_video.clone())>
                                                                <figure class="image" style=" max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"MXPlayer(Free)"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.pro;S.title=undefined;end",self.link_video.clone())>
                                                                <figure class="image" style=" max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon" /></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"MXPlayer(Pro)"}</p>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                    </div>
                };
                true
            }
            GetOption(value) => {
                self.number = value;
                true
            }
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/1f829fb9436bfe24268411b97afa5f96/raw/23a1999bed331316c4f19709f523fe589f9baae0/tester.json")
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