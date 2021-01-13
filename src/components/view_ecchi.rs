use yew::prelude::*;

use crate::switch::{AppAnchor, AppRoute};

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
                    <AppAnchor route=AppRoute::Eps(199)>
                            <div class="card">
                                <h3>{"4.6"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://highschooldxdrpg-maniahotel.weebly.com/uploads/2/1/3/2/21328416/5181061.png?442" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"High School DxD"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(402)>
                            <div class="card">
                            <h3>{"4.1"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://i.pinimg.com/originals/84/76/00/847600e526013b08c6f02d1a227eb4c2.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Shokugeki no Shoma"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(392)>
                            <div class="card">
                                <h3>{"4.8"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://static.zerochan.net/Nishikinomiya.Anna.full.2338518.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Shimoneta"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(335)>
                            <div class="card">
                                <h3>{"4.9"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://thicc.mywaifulist.moe/waifus/15517/b2103ecb978045890d878ebb0831a146d40b6fb2331e2436e1b5bc0491f7d843_thumb.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"No Game No Life"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(334)>
                            <div class="card">
                                <h3>{"4.2"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://1.bp.blogspot.com/-d1tDdXg-5b8/VyVi9UOAYZI/AAAAAAAAF4U/NrRSic34ONMRRDdPTMGViTJyIwoC3KvlQCLcB/s1600/nekomonogatari__shiro____anime_icon_by_ggonido-d6d9s4t.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Nisemonogatari"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(328)>
                            <div class="card">
                                <h3>{"4.9"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://bestofcomicbooks.com/wp-content/uploads/2019/03/Tsubasa-Hanekawa-near-nude.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Nekomonogatari (Kuro)"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                    </div>
                </div>
            </>
        }        
    }
}