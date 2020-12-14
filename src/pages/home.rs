use yew::prelude::*;

pub struct Home;
impl Component for Home
{
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        unimplemented!
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <>
                <div class="sticky-xl-top">
                    <header class="masthead">
                        <div class="container d-flex h-100 align-items-center">
                            <div class="mx-auto text-center">
                                <h1 class="mx-auto my-0 text-uppercase">{"LowStream"}</h1>
                                <h2 class="text-white-50 mx-auto mt-2 mb-5">{"Assista com qualidade e performance no LowStream"}</h2>
                                <a class="btn btn-primary js-scroll-trigger" href="#about">{"Get Started"}</a>
                            </div>
                        </div>
                    </header>
                </div>
            </>
        }
    }
}

// impl Home
// {

// }