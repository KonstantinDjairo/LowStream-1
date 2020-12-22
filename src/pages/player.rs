use yew::prelude::*;

pub struct Player;
impl Component for Player {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="hero is-medium is-dark has-background" style="padding-top: 30px;">
                    <img src="https://i.imgur.com/0ve4gmx.jpg" class="hero-background is-transparent"/>
                        <div class="hero-body">
                            <div class="container">
                                <h1 class="title">
                                    {"Shingeki no Kyojin"}
                                </h1>
                                <h2 class="subtitle">{"Final Season"} <a href="/router/authors/1443460666139885527" class="has-text-weight-semibold">
                                    {": Ep. 1"}
                                    </a>
                                </h2>
                                <span class="tag is-danger">
                                    {"Ação"}
                                </span>
                            <div class="context has-text-centered" style="padding-top: 40px; padding-bottom: 40px;">
                                <video controls=true autoplay="" width="800" height="550" style="border-radius: 18px; box-shadow: 0px 0px 18px rgba(0, 0, 0, 70%)">
                                    <source src="https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv" type="video/x-matroska"/>
                                    <track label="Português" kind="subtitles" srclang="pt-BR" src="https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%20x265/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.vtt" default=true/>
                                </video>
                                <div class="card notransition" style="background-color: rgba(0, 0, 0, 50%); backdrop-filter: blur(10px); color: white">
                                        <header class="card-header" style="color: white">
                                            <p class="card-header-title"><span class="icon"><i aria-hidden="true" class="fa fa-play-circle"></i></span>{" Play in android "}<span class="icon"></span></p>
                                        </header>
                                        <div class="card-content">
                                            <div class="content">
                                                <div class="columns is-mobile is-multiline has-text-centered">
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="iina://weblink?url=https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv"
                                                                class="">
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/iina.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"IINA"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="potplayer://https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv">
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/potplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"PotPlayer"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="vlc://https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv">
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/vlc.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"VLC"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="thunder://QUFodHRwczovLzUub3JlenJhZXkud29ya2Vycy5kZXYvMDovQW5pbWVzL1NoaW5nZWtpJTIwbm8lMjBLeW9qaW46JTIwVGhlJTIwRmluYWwlMjBTZWFzb24vTSVDMyVCQWx0aXBsYXMlMjBMZWdlbmRhcyUyMC0lMjAxMDgwcCUyNTJGNzIwcCUyNTJGNDgwcC8xMDgwcC9TaGluZ2VraSUyMG5vJTIwS3lvamluJTIwLSUyMFRoZSUyMEZpbmFsJTIwU2Vhc29uJTIwLSUyMDAxJTIwJTVCMTA4MHAlNUQlNUJNJUMzJUJBbHRpcGxhcyUyMExlZ2VuZGFzJTVELm1rdlpa">
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
                                                            <a href="nplayer-https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv">
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/nplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"nPlayer"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="intent:https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv#Intent;package=com.mxtech.videoplayer.ad;S.title=undefined;end">
                                                                <figure class="image is-48x48" style="margin: 0px auto;"><img src="https://cdn.jsdelivr.net/gh/Aicirou/goindex-theme-acrou@v2.0.8/dist/images/player/mxplayer.png" class="icon"/></figure>
                                                            </a>
                                                        </p>
                                                        <p>{"MXPlayer(Free)"}</p>
                                                    </div>
                                                    <div class="column">
                                                        <p class="heading">
                                                            <a href="intent:https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv#Intent;package=com.mxtech.videoplayer.pro;S.title=undefined;end">
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
