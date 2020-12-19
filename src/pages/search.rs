use yew::prelude::*;
pub struct Search;
impl Component for Search {
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

    fn view(&self) -> Html
    {
        html!{
            <div class="level-item" style="padding-top: 80px;">
                <div class="field has-addons">
                    <p class="control">
                    <input class="input" type="text" placeholder="Find a post"/>
                    </p>
                    <p class="control">
                    <button class="button">
                        {"Pesquisar"}
                    </button>
                    </p>
                </div>
            </div>
        }        
    }
}
