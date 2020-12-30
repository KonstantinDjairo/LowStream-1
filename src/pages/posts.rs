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
    pages::carousel,
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
        let mut background: Vec<String> = Vec::new();
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes.len()
                {
                    if search(content.animes[i].anime.clone().to_lowercase(), self.debugged_payload.clone().to_lowercase())
                    {
                        background.push(content.animes[i].background.clone());
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Eps(i as u64)>
                                <a class="card-image" style=format!("background-image: url({});", content.animes[i].background.clone()) loading="lazy">
                                </a>
                                <a class="card-description">
                                    <strong><h2>{content.animes[i].anime.clone()}</h2></strong>
                                    <p>{"Assistir agora"}</p>
                                </a>
                            </AppAnchor>
                            </li>
                        })
                    }
                }

                html! {
                    <>
                        <carousel::Model background=self.export_background()  />
                        <section style="background-color: #25262F;">
                        <div class="mx-auto" style="width: 250px;">
                                <div class="field has-addons" style="padding-top: 80px;">
                                    <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Encontre seu anime"/>
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
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() }
                <div class="content">
                <button id="prev" class="btn">
                    <i class="bx bxs-chevron-left"></i>
                </button>
                <div class="con-cards">
                    <div class="card">
                        <h3>{"4.6"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://freepngimg.com/thumb/anime/2-2-anime-png-pic.png" alt=""/>
                            <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Chuunibyou Demo Koi Ga Shitai!"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"4.1"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://lh3.googleusercontent.com/proxy/iUGCT61btTRwzqAm3MPiEVXmbwQI667aetNTEjySXHrsld33W3OmqDgcmw5XMEX8PF9vqAeB95cpgiJZztehRCq3JvH8rGlscEvGyUD0iyFQJb9JhXiB15Jt4sPGJYM" alt=""/>
                            <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Nico Nico Nii"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"4.8"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://i.pinimg.com/originals/79/67/fe/7967feedae6a76b044fc407a1a3026cf.png" alt=""/>
                            <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Boku no Hero"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"3.9"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/5.png" alt=""/>
                            <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/5.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Angry Birds"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"4.2"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/6.png" alt=""/>
                            <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/6.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Clash of Clans"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"4.9"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/7.png" alt=""/>
                            <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/7.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Dark Souls"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"3.6"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/8.png" alt=""/>
                            <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/8.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Far Cry 4"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"3.8"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/9.png" alt=""/>
                            <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/9.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Final Fantasy"}
                            </h2>
                        </div>
                    </div>
                    <div class="card">
                        <h3>{"4.7"}</h3>
                        <i class="bx bx-heart"></i>
                        <div class="con-img">
                            <img src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/10.png" alt=""/>
                            <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/10.png" alt=""/>
                        </div>
                        <div class="con-text">
                            <h2>
                                {"Gears of War"}
                            </h2>
                        </div>
                    </div>
                </div>
                <button id="next" class="btn">
                    <i class="bx bxs-chevron-right" ></i>
                </button>
            </div>
            </>
        }
    }
}