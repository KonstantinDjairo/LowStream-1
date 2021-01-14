use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub video: String,
    pub type_video: String,
    pub poster: String
}

pub struct Video
{
    video: String,
    type_video: String,
    poster: String
}

impl Component for Video {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
        {
            video: props.video,
            type_video: props.type_video,
            poster: props.poster
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
            <video controls=true autoplay="" poster=self.poster.clone() width="100%" height="550" style="border-radius: 18px; box-shadow: 0px 0px 18px rgba(0, 0, 0, 70%)">
                <source src=self.video.clone() type=self.type_video.clone()/>
            </video>
        }
    }
}
