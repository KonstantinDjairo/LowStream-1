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
                <div class="card-content">
                    <div >
                        <div class="columns is-mobile is-multiline has-text-centered">
                            <div class="column">
                                <p class="heading">
                                    <a href=format!("iina://weblink?url={}",self.props.link_video.clone())>
                                        <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/iina.png" class="icon"/></figure>
                                    </a>
                                </p>
                                <p>{"IINA"}</p>
                            </div>
                            <div class="column">
                                <p class="heading">
                                    <a href=format!("potplayer://{}",self.props.link_video.clone())>
                                        <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/potplayer.png" class="icon"/></figure>
                                    </a>
                                </p>
                                <p>{"PotPlayer"}</p>
                            </div>
                            <div class="column">
                                <p class="heading">
                                    <a href=format!("vlc://{}",self.props.link_video.clone())>
                                        <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/vlc.png" class="icon"/></figure>
                                    </a>
                                </p>
                                <p>{"VLC"}</p>
                            </div>
                            <div class="column">
                                <p class="heading">
                                    <a href=format!("thunder://{}",self.props.link_video.clone())>
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
                                    <a href=format!("nplayer-{}",self.props.link_video.clone())>
                                        <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/nplayer.png" class="icon"/></figure>
                                    </a>
                                </p>
                                <p>{"nPlayer"}</p>
                            </div>
                            <div class="column">
                                <p class="heading">
                                    <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.ad;S.title=undefined;end",self.props.link_video.clone())>
                                        <figure class="image" style=" max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon"/></figure>
                                    </a>
                                </p>
                                <p>{"MXPlayer(Free)"}</p>
                            </div>
                            <div class="column">
                                <p class="heading">
                                    <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.pro;S.title=undefined;end",self.props.link_video.clone())>
                                        <figure class="image" style=" max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon" /></figure>
                                    </a>
                                </p>
                                <p>{"MXPlayer(Pro)"}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }        
    }
}