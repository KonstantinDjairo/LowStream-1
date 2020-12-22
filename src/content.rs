use yew::prelude::*;

mod data;
use data::FetchServiceExample;

pub struct Post
{
    pub index: u64,
    pub title: String,
    pub ep: String
}

pub struct Content;
impl Component for Content
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
                    <img src="IMAGEM.jpg" class="hero-background is-transparent"/>
                        <div class="hero-body">
                            <div class="container">
                                <h1 class="title">
                                    {"TITULO"}
                                </h1>
                                <h2 class="subtitle">{"SUB TITULO"} <a class="has-text-weight-semibold">
                                    {"EPISÓDIO"}
                                    </a=>
                                </h2>
                                <span class="tag is-danger">
                                    {"TAG"}
                                </span>
                        </div>
                    </div>
                </section>

            </>
        }
    }
}

impl Content
{

}