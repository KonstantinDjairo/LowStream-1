use yew::prelude::*;

use crate::{
    switch::{AppAnchor, AppRoute},
    components::card::Card
};

pub struct Ecchi;
impl Component for Ecchi {
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
            <>
                <div class="container">
                    <h2 style="color: white; font-size: 200%; font-weight: bold; position: flex; padding-left: 30px; top: 38pc; line-height: 80%;">
                        {"Ecchi"}
                    </h2>
                </div>
                <div class="content">
                    <div class="con-cards">
                        <div class="padding-40px">
                            <AppAnchor route=AppRoute::Eps(200)>
                                <Card 
                                    score="4.6".to_string() 
                                    sticker="https://highschooldxdrpg-maniahotel.weebly.com/uploads/2/1/3/2/21328416/5181061.png?442".to_string() 
                                    name="High School DxD".to_string() 
                                    />
                            </AppAnchor>
                        </div>
                        <div class="padding-40px">
                            <AppAnchor route=AppRoute::Eps(403)>
                                <Card 
                                    score="4.1".to_string() 
                                    sticker="https://i.pinimg.com/originals/84/76/00/847600e526013b08c6f02d1a227eb4c2.png".to_string() 
                                    name="Shokugeki no Shoma".to_string() 
                                    />
                            </AppAnchor>
                        </div>
                        <div class="padding-40px">
                            <AppAnchor route=AppRoute::Eps(393)>
                                <Card 
                                    score="4.8".to_string() 
                                    sticker="https://static.zerochan.net/Nishikinomiya.Anna.full.2338518.png".to_string() 
                                    name="Shimoneta".to_string() 
                                    />
                            </AppAnchor>
                        </div>
                        <div class="padding-40px">
                            <AppAnchor route=AppRoute::Eps(336)>
                                <Card 
                                    score="4.9".to_string() 
                                    sticker="https://thicc.mywaifulist.moe/waifus/15517/b2103ecb978045890d878ebb0831a146d40b6fb2331e2436e1b5bc0491f7d843_thumb.png".to_string() 
                                    name="No Game No Life".to_string() 
                                    />
                            </AppAnchor>
                        </div>
                        <div class="padding-40px">
                            <AppAnchor route=AppRoute::Eps(335)>
                                <Card 
                                    score="4.2".to_string() 
                                    sticker="https://1.bp.blogspot.com/-d1tDdXg-5b8/VyVi9UOAYZI/AAAAAAAAF4U/NrRSic34ONMRRDdPTMGViTJyIwoC3KvlQCLcB/s1600/nekomonogatari__shiro____anime_icon_by_ggonido-d6d9s4t.png".to_string() 
                                    name="Nisemonogatari".to_string() 
                                    />
                            </AppAnchor>
                        </div>
                        <div class="padding-40px">
                            <AppAnchor route=AppRoute::Eps(329)>
                                <Card 
                                    score="4.9".to_string() 
                                    sticker="https://bestofcomicbooks.com/wp-content/uploads/2019/03/Tsubasa-Hanekawa-near-nude.png".to_string() 
                                    name="Nekomonogatari (Kuro)".to_string() 
                                    />
                            </AppAnchor>
                        </div>
                    </div>
                </div>
            </>
        }        
    }
}