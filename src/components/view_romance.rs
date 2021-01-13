use yew::prelude::*;

use crate::switch::{AppAnchor, AppRoute};

pub struct Romance;
impl Component for Romance {
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
                <div class="hero-body">
                    <div class="container">
                        <h2 style="color: white; font-size: 200%; font-weight: bold; position: flex; padding-left: 30px; top: 38pc; line-height: 80%;">
                            {"Romance"}
                        </h2>
                    </div>
                </div>
                    <div class="content">
                        <div class="con-cards">
                        <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(85)>
                                <div class="card">
                                    <h3>{"4.6"}</h3>
                                    <i class="bx bx-heart"></i>
                                    <div class="con-img">
                                        <img src="https://2.bp.blogspot.com/-y6b2yUEskoQ/Upd0sAPUgEI/AAAAAAAAYcY/75bT_PnNcF8/s1600/bokura2.png" alt=""/>
                                    </div>
                                    <div class="con-text">
                                        <h2>
                                            {"Bokura ga ita"}
                                        </h2>
                                    </div>
                                </div>
                            </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(351)>
                                <div class="card">
                                <h3>{"4.1"}</h3>
                                    <i class="bx bx-heart"></i>
                                    <div class="con-img">
                                        <img src="https://vignette.wikia.nocookie.net/ookamishoujotokuroouji/images/c/c0/Erika.png/revision/latest?cb=20141205233615" alt=""/>
                                    </div>
                                    <div class="con-text">
                                        <h2>
                                            {"Ookami Shoujo to Kuro Ouji"}
                                        </h2>
                                    </div>
                                </div>
                            </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(42)>
                                <div class="card">
                                    <h3>{"4.2"}</h3>
                                    <i class="bx bx-heart"></i>
                                    <div class="con-img">
                                        <img src="https://yukimoongfx.files.wordpress.com/2016/07/anohana_menma.png?w=584" alt=""/>
                                    </div>
                                    <div class="con-text">
                                        <h2>
                                            {"anohana"}
                                        </h2>
                                    </div>
                                </div>
                            </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(440)>
                                <div class="card">
                                    <h3>{"4.8"}</h3>
                                    <i class="bx bx-heart"></i>
                                    <div class="con-img">
                                        <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/b41fd7f8-4e06-43f7-9955-e14b879b0475/d8h28br-31de4b9d-f89b-4ca5-abd8-3756f20b3af5.png/v1/fill/w_788,h_1014,strp/_toradora__taiga_and_ryuuji_render_by_dracumarcy567_d8h28br-pre.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD0xMjg2IiwicGF0aCI6IlwvZlwvYjQxZmQ3ZjgtNGUwNi00M2Y3LTk5NTUtZTE0Yjg3OWIwNDc1XC9kOGgyOGJyLTMxZGU0YjlkLWY4OWItNGNhNS1hYmQ4LTM3NTZmMjBiM2FmNS5wbmciLCJ3aWR0aCI6Ijw9MTAwMCJ9XV0sImF1ZCI6WyJ1cm46c2VydmljZTppbWFnZS5vcGVyYXRpb25zIl19.3gf7qMvFA2iG73lFpT2Bk9gtyQpexyhSO-Dz1EFAZxA" alt=""/>
                                    </div>
                                    <div class="con-text">
                                        <h2>
                                            {"Toradora!"}
                                        </h2>
                                    </div>
                                </div>
                            </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(253)>
                                <div class="card">
                                    <h3>{"4.9"}</h3>
                                    <i class="bx bx-heart"></i>
                                    <div class="con-img">
                                        <img src="https://i.pinimg.com/originals/5a/65/49/5a6549c7e90fe13a87c9fa07bf086ad4.png" alt=""/>
                                    </div>
                                    <div class="con-text">
                                        <h2>
                                            {"Kimi ni Todoke"}
                                        </h2>
                                    </div>
                                </div>
                            </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(376)>
                                <div class="card">
                                    <h3>{"4.9"}</h3>
                                    <i class="bx bx-heart"></i>
                                    <div class="con-img">
                                        <img src="https://r7.hiclipart.com/path/450/558/570/relife-myanimelist-television-slice-of-life-anime-75a92bf4e55263518acc76b77430fdc2.png" alt=""/>
                                    </div>
                                    <div class="con-text">
                                        <h2>
                                            {"ReLife"}
                                        </h2>
                                    </div>
                                </div>
                            </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(107)>
                                <div class="card">
                                    <h3>{"4.2"}</h3>
                                    <i class="bx bx-heart"></i>
                                    <div class="con-img">
                                        <img src="https://darli-fra.jp/assets/img/pc/kv_main.png" alt=""/>
                                    </div>
                                    <div class="con-text">
                                        <h2>
                                            {"Darling in the FranXX"}
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