// use std::time::Duration;
use yew::{
    prelude::*,
    // services::interval::{IntervalService, IntervalTask},
};

// use std::{thread, time::Instant};

use rand::prelude::*;

pub struct Model {
    pub link: ComponentLink<Self>,
    pub value: usize,
    pub image: Vec<String>,
    pub conteudo: Html,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub background: Vec<String>,
}

#[derive(Debug)]
pub enum Msg {
    MoveToLeft,
    MoveToRight,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::MoveToRight);
        callback.emit(Msg::MoveToRight);
        Self {
            link,
            value: 0,
            image: props.background,
            conteudo: html! {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MoveToRight => {
                let mut rng = rand::thread_rng();
                self.value = rng.gen_range(0, self.image.len());


                self.conteudo = html! {
                    <section class="hero is-medium is-dark is-bold has-background">
                        <img src=format!("{}", self.image[self.value].clone()) class="hero-background img-fluid is-transparent" style="filter: blur(8px);"/>
                        <div class="hero-body">
                            <div class="container" style="padding-top: 60px">
                                <h1 class="title">
                                    {"Animes"}
                                </h1>
                            </div>
                        </div>
                    </section>
                };
            }
            Msg::MoveToLeft => {
                if self.value == 0 {
                    self.value = self.image.len() - 1;
                } else {
                    self.value -= 1;
                }
                self.conteudo = html! {
                    <img src=format!("{}", self.image[self.value].clone())/>
                };
                
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // let Self { conteudo, link, .. } = self;
        html! {

            <>
                <div>
                    // <button onclick = self.link.callback(|_| Msg::MoveToRight)>{ "random" }</button>
                    {self.conteudo.clone()}
                </div>
            </>
        }
    }
}