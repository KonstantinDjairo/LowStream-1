use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub ep: String,
    pub background: String,
    pub name: String,
    pub type_video: String
}

pub enum Msg
{
    ActionBottom
}

pub struct Player
{
    ep: String,
    background: String,
    name: String,
    type_video: String,
    link: ComponentLink<Self>,
    notification: bool
}

impl Component for Player {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
        {
            ep: props.ep,
            background: props.background,
            name: props.name,
            type_video: props.type_video,
            link,
            notification: true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg
        {
            Msg::ActionBottom => 
            {
                self.notification = !self.notification;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="hero is-medium is-dark has-background" style="padding-top: 30px;">
                    <img src=self.background.clone() class="hero-background is-transparent" style="filter: blur(8px)"/>
                        <div class="hero-body">
                            <div class="container">
                                <h1 class="title">
                                    {self.name.clone()
                                        .replace("%20", " ")
                                        .replace("%5B", " ")
                                        .replace("%5D", " ")
                                        .replace("%21", "!")
                                        .replace("%E2%96%B3", "△")
                                        .replace("%C3%BA", "ú")
                                        .replace("%E2%99%AA", "♪")
                                        .replace("%C3%81", "á")
                                        .replace("%C3%97", "×")
                                        .replace("%E2%98%86", "☆")
                                        .replace(".mkv", " ")
                                        .replace(".mp4", " ")
                                        .replace(".avi", " ")
                                    }
                                </h1>
                                {self.notification()}
                                // <h2 class="subtitle">{"Final Season"} <a href="/router/authors/1443460666139885527" class="has-text-weight-semibold">
                                //     {": Ep. 1"}
                                //     </a>
                                // </h2>
                                // <span class="tag is-danger">
                                //     {"Ação"}
                                // </span>
                            <div class="context has-text-centered" style="padding-top: 40px; padding-bottom: 40px;">
                                <video controls=true autoplay="" width="800" height="550" style="border-radius: 18px; box-shadow: 0px 0px 18px rgba(0, 0, 0, 70%)">
                                    <source src=self.ep.clone() type=self.type_video.clone()/>
                                </video>
                                <div class="notransition" style="background-color: rgba(0, 0, 0, 50%); backdrop-filter: blur(10px); color: white; display: inline-block;
                                    width: 90%;
                                    max-width: 20rem;
                                    margin: 1rem;
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
                                                            <a href=format!("iina://weblink?url={}",self.ep.clone())>
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/iina.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"IINA"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("potplayer://{}",self.ep.clone())>
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/potplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"PotPlayer"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("vlc://{}",self.ep.clone())>
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/vlc.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"VLC"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("thunder://{}",self.ep.clone())>
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/thunder.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"Thunder"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="javascript:alert(&quot;暂未实现&quot;)">
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/aria2.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"Aria2"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("nplayer-{}",self.ep.clone())>
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/nplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"nPlayer"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.ad;S.title=undefined;end",self.ep.clone())>
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"MXPlayer(Free)"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.pro;S.title=undefined;end",self.ep.clone())>
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon" /></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"MXPlayer(Pro)"}</p>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                            </div>
                        </div>
                    </div>
                </section>

            </>
        }
    }
}

impl Player
{
    fn notification(&self) -> Html
        {
            if self.notification
            {
                return html!{
                    <div class="notification is-danger is-dark">
                        <button class="delete" onclick=self.link.callback(|_| Msg::ActionBottom)></button>
                            <strong>{"Atenção! "}</strong>
                            {"A equipe ainda está trabalhando no player,ainda está muito instável, 
                            por conta disso recomendamos que use players externos. 
                            Obrigado por sua visita. uwu"}
                        </div>
                }
            }
            html!{}
        }
}