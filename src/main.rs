#![recursion_limit = "1024"]
use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

macro_rules! classes {
    ($classe:expr, $classe_condition:expr) => {
        format!("{} {}", $classe, $classe_condition)
    };
}

mod pages;
use pages::{
    home::Home, login::Login, page_not_found::PageNotFound, player::Player, register::Register, post::Eps, posts::LoadPosts,
    search::Search,
};

mod data;
use data::FetchServiceExample;

mod switch;
use switch::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

pub enum Msg {
    ToggleNav,
    ActionBottom,
}

struct Model {
    link: ComponentLink<Self>,
    navbar: bool,
    notification: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar: false,
            notification: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNav => {
                self.navbar = !self.navbar;
                true
            }
            Msg::ActionBottom => {
                self.notification = !self.notification;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <>
                {self.nav()}
                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        })
                    />
                </main>
            </>
        }
    }
}

impl Model {
    fn nav(&self) -> Html {
        let Self {
            ref link, navbar, ..
        } = *self;

        let active_class = if navbar { "is-active" } else { "" };

        html! {
                <>
            <nav class="navbar is-black is-transparent is-fixed-top" style="background-color: rgba(0, 0, 0, 50%); backdrop-filter: blur(10px);">
                <div class="navbar-brand is-rounded is-dark">
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                    <img src="https://www.pngkey.com/png/full/308-3085243_logo-rust-programming-language-logo.png" width="28" height="28" alt="LowStream" style="position: absolute; top: 8px;"/>
                    <h3 style="position: absolute; top: 8px; right: -120px"><strong>{"LowStream"}</strong></h3>
                    </AppAnchor>
                    <div class="navbar-burger" data-target="navbarExampleTransparentExample" onclick=link.callback(|_| Msg::ToggleNav) style="position: absolute; top: 8px; right: 16px; background-color: black; border-radius: 100%">
                    <span></span>
                    <span></span>
                    <span></span>
                    </div>
                </div>

                <div id="navbarExampleTransparentExample" class=classes!("navbar-menu", active_class) style="position: absolute; top: 5px; right: 16px; background-color: rgb(0, 0, 0, 100%); border-radius: 28px;">
                    <div class="navbar-start">
                    <a class=classes!("navbar-burger", active_class) onclick=link.callback(|_| Msg::ToggleNav)>
                        <span></span>
                        <span></span>
                        <span></span>
                    </a>
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a onclick=link.callback(|_| Msg::ToggleNav) style="color: white">{ "Home" }</a>
                    </AppAnchor>
                    <AppAnchor classes="navbar-item" route=AppRoute::Animes>
                            <a onclick=link.callback(|_| Msg::ToggleNav) style="color: white">{ "Animes" }</a>
                    </AppAnchor>
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a onclick=link.callback(|_| Msg::ToggleNav) style="color: white">{ "Filmes" }</a>
                    </AppAnchor>
                    <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a onclick=link.callback(|_| Msg::ToggleNav) style="color: white">{ "Series" }</a>
                    </AppAnchor>
                    <div class="navbar-item has-dropdown is-hoverable" style="background-color: rgba(0, 0, 0, 0%); backdrop-filter: blur(10px);">
                        <a class="navbar-link" style="color: white">
                        {"Mais"}
                        </a>
                        <div class="navbar-dropdown is-boxed" style="background-color: rgba(0, 0, 0);">
                        <AppAnchor classes="navbar-item" route=AppRoute::Data>
                            <a onclick=link.callback(|_| Msg::ToggleNav) style="color: white">{ "Data" }</a>
                        </AppAnchor>
                        <a class="navbar-item" onclick=link.callback(|_| Msg::ToggleNav) style="color: white">
                            {"Contact us"}
                        </a>
                        <a class="navbar-item" href="https://github.com/LowStream-Community/LowStream/issues/new/choose" onclick=link.callback(|_| Msg::ToggleNav) style="color: white">
                            {"Issues"}
                        </a>
                        <hr class="navbar-divider"/>
                        <a class="navbar-item" href="https://github.com/lowstream-community/LowStream" onclick=link.callback(|_| Msg::ToggleNav) style="color: white">
                            {"GitHub"}
                        </a>
                        </div>
                    </div>
                    </div>

                    <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons" onclick=link.callback(|_| Msg::ToggleNav)>
                            <AppAnchor classes="button is-light is-rounded" route=AppRoute::Search>
                                { "Pesquisar" }
                            </AppAnchor>
                            <AppAnchor classes="button is-dark is-rounded" route=AppRoute::Player>
                                { "Random" }
                            </AppAnchor>
                        </div>
                    </div>
                    </div>
                </div>
                </nav>
                </>
        }
    }

    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            // AppRoute::Ep => {
            //     html! { <FetchServiceExample /> }
            // }
            AppRoute::Animes => {
                html! { <LoadPosts  /> }
            }
            AppRoute::Eps(id) => {
                html! { <Eps animeName=id /> }
            }
            AppRoute::Data => {
                html! { <FetchServiceExample /> }
            }
            AppRoute::Search => {
                html! { <Search /> }
            }
            AppRoute::Player => {
                html! { <Player /> }
            }
            AppRoute::Login => {
                html! { <Login /> }
            }
            AppRoute::Register => {
                html! { <Register /> }
            }
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    App::<Model>::new().mount_to_body();
}
