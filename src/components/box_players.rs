use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub link_video: String,
}

pub struct BoxPlayers
{
    props: Props
}
impl Component for BoxPlayers {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <>
            <div class="notransition" style="background-color: rgba(0, 0, 0, 50%); color: white; display: inline-block;
                                    width: 90%;
                                    font-size: 1rem;
                                    text-decoration: none;
                                    overflow: hidden;
                                    box-shadow: 0 0 4rem -1rem rgba(0, 0, 0, 1);
                                    border-radius: 18px;">
                    <header class="card-header">
                        <h2 class="card-header-title" style="color: white"><span class="icon"><i aria-hidden="true" class="fa fa-play-circle"></i></span>{" Play in android "}<span class="icon"></span></h2>
                    </header>
                    <div class="card-content">
                        <div >
                            <div class="columns is-mobile is-multiline has-text-centered">
                                <div class="column">
                                    <p class="heading">
                                        <a href=format!("iina://weblink?url={}",self.props.link_video.clone())>
                                            <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/iina.png" class="icon"/></figure>
                                        </a>
                                    </p>
                                    <h2>{"IINA"}</h2>
                                </div>
                                <div class="column">
                                    <p class="heading">
                                        <a href=format!("potplayer://{}",self.props.link_video.clone())>
                                            <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/potplayer.png" class="icon"/></figure>
                                        </a>
                                    </p>
                                    <h2>{"PotPlayer"}</h2>
                                </div>
                                <div class="column">
                                    <p class="heading">
                                        <a href=format!("vlc://{}",self.props.link_video.clone())>
                                            <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/vlc.png" class="icon"/></figure>
                                        </a>
                                    </p>
                                    <h2>{"VLC"}</h2>
                                </div>
                                <div class="column">
                                    <p class="heading">
                                        <a href=format!("thunder://{}",self.props.link_video.clone())>
                                            <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/thunder.png" class="icon"/></figure>
                                        </a>
                                    </p>
                                    <h2>{"Thunder"}</h2>
                                </div>
                                <div class="column">
                                    <p class="heading">
                                        <a href="javascript:alert(&quot;暂未实现&quot;)">
                                            <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/aria2.png" class="icon"/></figure>
                                        </a>
                                    </p>
                                    <h2>{"Aria2"}</h2>
                                </div>
                                <div class="column">
                                    <p class="heading">
                                        <a href=format!("nplayer-{}",self.props.link_video.clone())>
                                            <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/nplayer.png" class="icon"/></figure>
                                        </a>
                                    </p>
                                    <h2>{"nPlayer"}</h2>
                                </div>
                                <div class="column">
                                    <p class="heading">
                                        <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.ad;S.title=undefined;end",self.props.link_video.clone())>
                                            <figure class="image" style=" max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon"/></figure>
                                        </a>
                                    </p>
                                    <h2>{"MXPlayer(Free)"}</h2>
                                </div>
                                <div class="column">
                                    <p class="heading">
                                        <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.pro;S.title=undefined;end",self.props.link_video.clone())>
                                            <figure class="image" style=" max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon" /></figure>
                                        </a>
                                    </p>
                                    <h2>{"MXPlayer(Pro)"}</h2>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }        
    }
}