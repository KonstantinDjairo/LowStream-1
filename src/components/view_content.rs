use yew::prelude::*;

use crate::switch::{AppAnchor, AppRoute};

pub struct Content;
impl Component for Content {
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
            <div class="container" style="padding-top: 20px">
                    <h2 style="color: white; font-size: 200%; font-weight: bold; position: flex; padding-left: 30px; top: 38pc; line-height: 80%;">
                        {"Recomendados"}
                    </h2>
                </div>
                <div class="content">
                    <div class="con-cards">
                    <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(102)>
                            <div class="card">
                                <h3>{"4.6"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://freepngimg.com/thumb/anime/2-2-anime-png-pic.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Chuunibyou Demo Koi Ga Shitai!"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(450)>
                            <div class="card">
                            <h3>{"4.1"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://pbs.twimg.com/media/EQTDY4EXYAEVaf6.png:large" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Uzaki-chan Wa Asobitai!"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(78)>
                            <div class="card">
                                <h3>{"4.8"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://i.pinimg.com/originals/79/67/fe/7967feedae6a76b044fc407a1a3026cf.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Boku no Hero"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(324)>
                            <div class="card">
                                <h3>{"4.9"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/16825168-8b56-4bef-80cb-89abe4d1be01/dcw71ku-ab9b3fa0-5f70-4e38-84c4-ff52497fc273.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3sicGF0aCI6IlwvZlwvMTY4MjUxNjgtOGI1Ni00YmVmLTgwY2ItODlhYmU0ZDFiZTAxXC9kY3c3MWt1LWFiOWIzZmEwLTVmNzAtNGUzOC04NGM0LWZmNTI0OTdmYzI3My5wbmcifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6ZmlsZS5kb3dubG9hZCJdfQ.uLjGHYRxDt9Dzvj60iF-o72WtyJtrGj91uRahE9cMAc" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Naruto"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(147)>
                            <div class="card">
                                <h3>{"4.2"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://i.pinimg.com/originals/9f/ed/f4/9fedf491e770e5a440bc04225fa634bf.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Fullmetal Alchemist"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(393)>
                            <div class="card">
                                <h3>{"4.9"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/7c6f1009-9bed-42c1-90b2-5672c47100ef/d7axq72-b7025794-b93d-4abc-9f3f-acee56ee5499.png/v1/fill/w_1024,h_1402,strp/shingeki_no_kyojin_png_by_bloomsama_d7axq72-fullview.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD0xNDAyIiwicGF0aCI6IlwvZlwvN2M2ZjEwMDktOWJlZC00MmMxLTkwYjItNTY3MmM0NzEwMGVmXC9kN2F4cTcyLWI3MDI1Nzk0LWI5M2QtNGFiYy05ZjNmLWFjZWU1NmVlNTQ5OS5wbmciLCJ3aWR0aCI6Ijw9MTAyNCJ9XV0sImF1ZCI6WyJ1cm46c2VydmljZTppbWFnZS5vcGVyYXRpb25zIl19.3Ru6w3-GQ4Xal4lb9R5CPQywclD-0sd0YJ-gy7OD210" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Shingeki no Kyojin"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(342)>
                            <div class="card">
                                <h3>{"4.6"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/7c6f1009-9bed-42c1-90b2-5672c47100ef/d6mwl0m-ca65a3e0-1430-4695-8514-bb4422c4803e.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3sicGF0aCI6IlwvZlwvN2M2ZjEwMDktOWJlZC00MmMxLTkwYjItNTY3MmM0NzEwMGVmXC9kNm13bDBtLWNhNjVhM2UwLTE0MzAtNDY5NS04NTE0LWJiNDQyMmM0ODAzZS5wbmcifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6ZmlsZS5kb3dubG9hZCJdfQ.aZSFHcsl8grZRId5AYxTCZJ3ogv-gwZo07cPwn6QHnI" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"One Piece"}
                                    </h2>
                                </div>
                            </div>
                        </AppAnchor>
                        </div>
                        <div class="padding-40px">
                        <a  href="http://lowstream.tk/?id=109" target="_blank">
                            <div class="card">
                                <h3>{"4.9"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://www.nicepng.com/png/full/53-531079_death-note-png-death-note.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Death Note"}
                                    </h2>
                                </div>
                            </div>
                        </a>
                        </div>
                        <div class="padding-40px">
                    <AppAnchor route=AppRoute::Eps(213)>
                            <div class="card">
                                <h3>{"4.7"}</h3>
                                <i class="bx bx-heart"></i>
                                <div class="con-img">
                                    <img src="https://i.pinimg.com/originals/4b/81/f1/4b81f122dac472b696fb2cecbde9ca19.png" alt=""/>
                                </div>
                                <div class="con-text">
                                    <h2>
                                        {"Hunter x Hunter"}
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