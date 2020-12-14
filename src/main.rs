#![recursion_limit = "1024"]
use ybc::NavbarFixed::Bottom;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::prelude::*;

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                {self.nav()}

                <section class="hero is-medium is-primary is-bold">
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
                <div class="notification is-danger section-padding">
                <button class="delete"></button>
                    <strong>{"Atenção!"}</strong>
                    {"A equipe ainda está trabalhando no site, ainda há vários bugs, 
                    e não temos ainda uma data de previsão pra entrega do site. "}<a>
                    {"Porém possivelmente estará pronto para o ano de 2021."}</a> 
                    {"Obrigado por sua visita volte outra hora. uwu"}
                </div>
                 <footer class="footer">
                    <div class="content has-text-centered">
                    { "Powered by " }
                    <a>{ "Alexandroviski, AndréF, $enick & Pablo" }</a>
                    // { " and images from " }
                    // <a href="https://unsplash.com">{ "Unsplash" }</a>
                    // </br>
                    <a href="https://github.com/alexandroviski/LowStream">
                            <span class="icon">
                            <i class="fab fa-github"></i>
                            </span>
                            <strong>{"GitHub"}</strong>
                        </a>
                    </div>
                </footer>
            </>
        }
    }
}

impl Model {
    fn nav(&self) -> Html {
        html! {
                <>
            <nav class="navbar is-fixed-top" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item" href="https://lowstreamcast.netlify.app/">
                <img src="https://www.pngkey.com/png/full/308-3085243_logo-rust-programming-language-logo.png" width="28" height="28" alt="LowStream"/>
                <h3><strong>{"LowStream"}</strong></h3>
                </a>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="navbarBasicExample is-dark" class="navbar-menu">
                <div class="navbar-start">
                <a class="navbar-item">
                    {"Home"}
                </a>

                <a class="navbar-item">
                    {"Animes"}
                </a>

                <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">
                    {"More"}
                    </a>

                    <div class="navbar-dropdown">
                    <a class="navbar-item">
                        {"Movies"}
                    </a>
                    <a class="navbar-item">
                        {"Series"}
                    </a>
                    <a class="navbar-item">
                        {"About us"}
                    </a>
                    <hr class="navbar-divider"/>
                    <a class="navbar-item">
                        {"Report an issue"}
                    </a>
                    </div>
                </div>
                </div>

                <div class="navbar-end">
                <div class="navbar-item">
                    <div class="buttons">
                    // <a class="botton is-light">
                    //     <span class="icon">
                    //         <i class="magnify"></i>
                    //     </span>
                    // </a>
                    <a class="button is-light is-rounded">
                        <strong>{"Sign up"}</strong>
                    </a>
                    <a class="button is-dark is-rounded">
                        {"Log in"}
                    </a>
                    </div>
                </div>
                </div>
            </div>
            </nav>
                </>
        }
    }
}

fn main() {
    App::<Model>::new().mount_to_body();
}
