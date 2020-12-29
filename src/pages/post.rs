use yew::prelude::*;

use crate::{
    pages,
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub animeName: String,
    pub background: String
}

pub struct Eps
{
    link: ComponentLink<Self>,
    name: String

}
impl Component for Eps {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
        {
            link,
            name: props.animeName
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="hero is-medium is-dark is-bold has-background" style="padding-top: 40px">
                    <img src="https://i.pinimg.com/originals/30/94/e0/3094e0fd1114787639e8e334a840ca02.jpg" class="hero-background is-transparent"/>
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                {self.name.to_uppercase().clone()}
                            </h1>
                            <h2 class="subtitle">
                                {"Somos uma plataforma de streaming simples, performática  e funcional."}
                            </h2>
                        </div>
                    </div>
                </section>
            </>
        }
    }
}
