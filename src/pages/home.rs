use yew::prelude::*;
// use crate::{
//     switch::{AppAnchor, AppRoute},
// };

pub struct Home;
impl Component for Home
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
                <section class="hero is-medium is-dark is-bold has-background">
                    <img src="https://i.pinimg.com/originals/30/94/e0/3094e0fd1114787639e8e334a840ca02.jpg" class="hero-background is-transparent"/>
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                {"Seja muito Bem-Vindo(a)"}
                                <span class="tag is-white">
                                        {"new"}
                                </span>
                            </h1>
                            <h2 class="subtitle">
                                {"Somos uma plataforma de streaming simples, performática  e funcional."}
                            </h2>
                        </div>
                    </div>
                </section>                

                <section style="background-color: #25262F;">
                    <div class="container has-text-centered" style="padding-top: 10px">
                            <h1 class="title" style="color: white">
                                <strong>{"Acesse a aba animes acima"}</strong>
                            </h1>
                    </div>
                </section>                
            </>
        }
    }
}