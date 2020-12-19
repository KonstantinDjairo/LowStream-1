use yew::prelude::*;

pub struct Player;
impl Component for Player
{
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self
    {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <>
                <section class="hero is-medium is-light has-background" style="padding-top: 30px;">
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
                                    <video controls=true autoplay="" data-state="subtitles" name="Video Player" width="600" height="360" style="border-radius: 18px; box-shadow: 0px 0px 18px rgba(0, 0, 0, 70%)">
                                        <source src="https://5.orezraey.workers.dev/0:/Animes/Shingeki%20no%20Kyojin:%20The%20Final%20Season/M%C3%BAltiplas%20Legendas%20-%201080p%252F720p%252F480p/1080p/Shingeki%20no%20Kyojin%20-%20The%20Final%20Season%20-%2001%20%5B1080p%5D%5BM%C3%BAltiplas%20Legendas%5D.mkv" type="video/x-matroska;"/>
                                    </video>
                                </div>
                            <div class="tags">
                            </div>
                        </div>
                    </div>
                </section>

            </>
        }
    }
}