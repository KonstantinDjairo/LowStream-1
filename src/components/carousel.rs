// use std::time::{Duration, Instant};
use yew::{
    prelude::*,
    // services::interval::{IntervalService, IntervalTask},
};

use yewtil::NeqAssign;

// use std::{thread, time::Instant};

// const RESOLUTION: u64 = 500;
// const MIN_INTERVAL_MS: u64 = 50;

use rand::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub background: Vec<String>,
    // pub duration_ms: u64,
    //     // pub on_complete: Callback<()>,
    // #[prop_or_default]
    // pub on_progress: Callback<f64>,
}

pub struct Model {
    props: Props,
    link: ComponentLink<Self>,
    pub value: usize,
    // pub image: Vec<String>,
    pub conteudo: Html,
    // _task: IntervalTask,
    // start: Instant,
    // valor: f64,
}

#[derive(Debug)]
pub enum Msg {
    MoveToLeft,
    MoveToRight,
    // Update
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let interval = (props.duration_ms / RESOLUTION).min(MIN_INTERVAL_MS);
        // let task = IntervalService::spawn(
        //     Duration::from_millis(interval),
        //     link.callback(|_| Msg::Update),
        // );
        let callback = link.callback(|_msg: Msg| Msg::MoveToRight);
        callback.emit(Msg::MoveToRight);
        Self {
            props,
            link,
            value: 0,
            conteudo: html! {},
            // _task: task,
            // start: Instant::now(),
            // valor: 0.0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MoveToRight => {
                let mut rng = rand::thread_rng();
                self.value = rng.gen_range(0, self.props.background.len());


                self.conteudo = html! {
                    <section class="hero is-medium is-dark is-bold has-background">
                        <img src=format!("{}", self.props.background[self.value].clone()) class="hero-background img-fluid is-transparent"/>
                        <div class="hero-body">
                            <div class="container" style="padding-top: 60px">
                                <h1 class="title">
                                    {""}
                                </h1>
                            </div>
                        </div>
                    </section>
                };
            }
            Msg::MoveToLeft => {
                if self.value == 0 {
                    self.value = self.props.background.len() - 1;
                } else {
                    self.value -= 1;
                }
                self.conteudo = html! {
                    <img src=format!("{}", self.props.background[self.value].clone())/>
                };
                
            }
            // Msg::Update => {
            //     let duration = self.props.duration_ms;
            //     let elapsed = self.start.elapsed().as_millis() as u64;
            //     self.valor = elapsed as f64 / duration as f64;

            //     if elapsed > duration {
            //         // self.props.on_complete.emit(());
            //         let callback = self.link.callback(|_msg: Msg| Msg::MoveToRight);
            //         callback.emit(Msg::MoveToRight);
            //         self.start = Instant::now();
            //     } else {
            //         self.props.on_progress.emit(self.valor);
            //     }
            //     // true
            // }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        // let Self { conteudo, link, .. } = self;
        // let value = self.valor;
        html! {

            <>
                <div>
                    // <button onclick = self.link.callback(|_| Msg::MoveToRight)>{ "random" }</button>
                    {self.conteudo.clone()}
                    // <progress class="progress is-primary" value=value max=1.0>
                    //     { format!("{:.0}%", 100.0 * value) }
                    // </progress>
                </div>
            </>
        }
    }
}