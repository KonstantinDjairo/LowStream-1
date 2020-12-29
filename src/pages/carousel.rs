use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: usize,
    image: Vec<String>,
    conteudo: Html,
}

#[derive(Debug)]
enum Msg {
    MoveToLeft,
    MoveToRight,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            image: vec![
                String::from("https://cdn.wallpapersafari.com/12/0/Af3WOb.jpg"),
                String::from("https://animekayo.com/wp-content/uploads/2020/07/480-4.jpg"),
                String::from(
                    "https://ninotaku.de/wp-content/uploads/2020/05/dxd2-scaled-e1588784140196.jpg",
                ),
            ],
            conteudo: html! {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MoveToRight => {
                if self.value == self.image.len() {
                    self.value = 0;
                }

                self.conteudo = html! {
                    <img src=format!("{}", self.image[self.value].clone())/>
                };
                self.value += 1;
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
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {

            <>
                <div>


                    <button onclick = self.link.callback(|_| Msg::MoveToLeft)>{ "Move to left" }</button>

                    <button onclick = self.link.callback(|_| Msg::MoveToRight)>{ "Move to right" }</button>

                        {self.conteudo.clone()}
                </div>


            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
