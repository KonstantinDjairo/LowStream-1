use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    // pub background: String,
    // pub name: String,
    pub video: String,
    pub type_video: String
}

pub struct Video
{
    video: String,
    // background: String,
    // name: String,
    type_video: String
}

impl Component for Video {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
        {
            // background: props.background,
            // name: props.name,
            video: props.video,
            type_video: props.type_video
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
            // <nav class="navbar is-fixed-bottom">
                <div class="context has-text-centered" style="padding-top: 40px; padding-bottom: 40px;">
                                <video controls=true autoplay="" width="800" height="550" style="border-radius: 18px; box-shadow: 0px 0px 18px rgba(0, 0, 0, 70%)">
                                    <source src=self.video.clone() type=self.type_video.clone()/>
                                </video>
                                <div class="notransition" style="background-color: rgba(0, 0, 0, 50%); backdrop-filter: blur(10px); color: white; display: inline-block;
                                    width: 90%;
                                    min-width: 20rem;
                                    min-heigth: 20rem;
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
                                                    // <div class="column">
                                                    //     <p class="heading">
                                                    //         <a href=format!("iina://weblink?url={}",self.video.clone())>
                                                    //             <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/iina.png" class="icon"/></figure>
                                                    //         </a>
                                                    //     </p>
                                                    //     <p>{"IINA"}</p>
                                                    // </div>
                                                    // <div class="column">
                                                    //     <p class="heading">
                                                    //         <a href=format!("potplayer://{}",self.video.clone())>
                                                    //             <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/potplayer.png" class="icon"/></figure>
                                                    //         </a>
                                                    //     </p>
                                                    //     <p>{"PotPlayer"}</p>
                                                    // </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("vlc://{}",self.video.clone())>
                                                                <figure class="image" style="max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/vlc.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"VLC"}</p>
                                                    </div>
                                //         //     //         <div class="column">
                                //         //     //             <p class="heading">
                                //         //     //                 <a href=format!("thunder://{}",self.video.clone())>
                                //         //     //                     <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/thunder.png" class="icon"/></figure>
                                //         //     //                 </a>
                                //         //     //             </p>
                                //         //     //             <p>{"Thunder"}</p>
                                //         //     //         </div>
                                //         //     //         <div class="column">
                                //         //     //             <p class="heading">
                                //         //     //                 <a href="javascript:alert(&quot;暂未实现&quot;)">
                                //         //     //                     <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/aria2.png" class="icon"/></figure>
                                //         //     //                 </a>
                                //         //     //             </p>
                                //         //     //             <p>{"Aria2"}</p>
                                //         //     //         </div>
                                //         //     //         <div class="column">
                                //         //     //             <p class="heading">
                                //         //     //                 <a href=format!("nplayer-{}",self.video.clone())>
                                //         //     //                     <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/nplayer.png" class="icon"/></figure>
                                //         //     //                 </a>
                                //         //     //             </p>
                                //         //     //             <p>{"nPlayer"}</p>
                                //         //     //         </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.ad;S.title=undefined;end",self.video.clone())>
                                                                <figure class="image" style=" max-width: 40px; height: auto; margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"MXPlayer(Free)"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href=format!("intent:{}#Intent;package=com.mxtech.videoplayer.pro;S.title=undefined;end",self.video.clone())>
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
                        // </nav>
            </>
        }
    }
}
