use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub score: String,
    pub sticker: String,
    pub name: String,
}

pub struct Card
{
    props: Props
}
impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
        {
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <div class="card">
                <h3>{self.props.score.clone()}</h3>
                <i class="bx bx-heart"></i>
                <div class="con-img">
                    <img src={self.props.sticker.clone()} alt={self.props.name.clone()} />
                </div>
                <div class="con-text">
                    <h2>
                        {self.props.name.clone()}
                    </h2>
                </div>
            </div>
        }        
    }
}
