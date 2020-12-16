use yew::prelude::*;

pub struct Home;
impl Component for Home {
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
                <section class="hero is-medium is-primary is-bold" style="padding-top: 40px">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                {"O melhor Streaming de videos uwu"}
                            </h1>
                            <h2 class="subtitle">
                                {"Aproveite as novidades!"}
                            </h2>
                        </div>
                    </div>
                </section>
            </>
        }
    }
}

// impl Home
// {

// }
