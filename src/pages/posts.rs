use serde::Deserialize;
// use yewtil::NeqAssign;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
// use yew_router::agent::{RouteAgentDispatcher, RouteRequest};

use crate::{
    switch::{AppAnchor, AppRoute},
    pages::carousel,
};

// const CAROUSEL_DELAY_MS: u64 = 15000;

#[derive(Deserialize, Debug, Clone)]
pub struct Ep {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    eps: Vec<Ep>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Anime {
    anime: String,
    background: String,
    // dados: Vec<Content>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    animes: Vec<Anime>
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ReceiveResponse(Result<Struture, anyhow::Error>),
    Payload(String),
}

#[derive(Debug)]
pub struct LoadPosts {
    payload: String,
    debugged_payload: String,
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl LoadPosts {
    fn content(&self) -> Html
    {
        html!{
            <>
            // <div class="hero-body">
                                <div class="container" style="padding-top: 20px">
                                    <h2 style="color: white; font-size: 200%; font-weight: bold; position: flex; padding-left: 30px; top: 38pc; line-height: 80%;">
                                        {"Recomendados"}
                                    </h2>
                                </div>
                            // </div>
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
                                // </div>
                                </>
        }
    }
    fn content_ecchi(&self) -> Html
    {
        html!{
            <>
            // <div class="hero-body">
                                <div class="container">
                                    <h2 style="color: white; font-size: 200%; font-weight: bold; position: flex; padding-left: 30px; top: 38pc; line-height: 80%;">
                                        {"Ecchi"}
                                    </h2>
                                </div>
                            // </div>
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
    fn content_shounen(&self) -> Html
    {
        html!{
            <>
            <div class="hero-body">
                                <div class="container">
                                    <h2 style="color: white; font-size: 200%; font-weight: bold; position: flex; padding-left: 30px; top: 38pc; line-height: 80%;">
                                        {"Shounen"}
                                    </h2>
                                </div>
                            </div>
                                <div class="content">
                                    <div class="con-cards">
                                    <div class="padding-40px">
                                    <AppAnchor route=AppRoute::Eps(109)>
                                            <div class="card">
                                                <h3>{"4.6"}</h3>
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
                                        </AppAnchor>
                                        </div>
                                        <div class="padding-40px">
                                    <AppAnchor route=AppRoute::Eps(393)>
                                            <div class="card">
                                            <h3>{"4.1"}</h3>
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
                                    <AppAnchor route=AppRoute::Eps(77)>
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
                                    <AppAnchor route=AppRoute::Eps(136)>
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://www.animeunited.com.br/wp-content/uploads/2018/08/Fairy-Tail-Dice-Magic.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Fairy Tail"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </AppAnchor>
                                        </div>
                                        <div class="padding-40px">
                                    <AppAnchor route=AppRoute::Eps(305)>
                                            <div class="card">
                                                <h3>{"4.2"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/7c6f1009-9bed-42c1-90b2-5672c47100ef/d6c2ljw-aed5c0dd-06e2-4379-a843-9a1249f8dc4b.png/v1/fill/w_575,h_864,strp/mirai_nikki_png_by_bloomsama_d6c2ljw-fullview.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD04NjQiLCJwYXRoIjoiXC9mXC83YzZmMTAwOS05YmVkLTQyYzEtOTBiMi01NjcyYzQ3MTAwZWZcL2Q2YzJsanctYWVkNWMwZGQtMDZlMi00Mzc5LWE4NDMtOWExMjQ5ZjhkYzRiLnBuZyIsIndpZHRoIjoiPD01NzUifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6aW1hZ2Uub3BlcmF0aW9ucyJdfQ.LQYKhBB66Nef7BZp76g9VAHMfwhiui6l666-uEHZAN4" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Mirai Nikki"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </AppAnchor>
                                        </div>
                                        <div class="padding-40px">
                                    <AppAnchor route=AppRoute::Eps(342)>
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/a3/ea/bb/a3eabbe003c6c2c8de86f2eb21efb9ad.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Noragami"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </AppAnchor>
                                        </div>
                                        <div class="padding-40px">
                                    <AppAnchor route=AppRoute::Eps(44)>
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/ce/25/a9/ce25a92e84110abfc509022004ddc3b1.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Ao no Exorcist"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </AppAnchor>
                                        </div>
                                        <div class="padding-40px">
                                    <AppAnchor route=AppRoute::Eps(30)>
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://s-media-cache-ak0.pinimg.com/originals/d0/50/30/d05030f5913866031215c6856ac34adf.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Akame ga Kill!"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </AppAnchor>
                                        </div>
                                        <div class="padding-40px">
                                    <AppAnchor route=AppRoute::Eps(321)>
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/c6/68/ba/c668ba3d743bded8f97e77cd7163131d.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Nanatsu no Taizai"}
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
    fn content_romance(&self) -> Html
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
    fn view_json(&self) -> Html {
        fn search(card_name: String, writing: String) -> bool
        {
            let chars_name: Vec<char> = card_name.chars().collect();
            let writing_chars: Vec<char> = writing.chars().collect();
            let mut word: String = String::default();
            for (i, c) in writing_chars.iter().enumerate()
            {
                if c == &chars_name[i]
                {
                    word = chars_name.iter().collect::<String>();
                }
                else
                {
                    return false;
                }
            }
            true
        }
        let mut cards: Vec<Html> = Vec::new();
        let mut background: Vec<String> = Vec::new();
        let mut count = 0;
        match self.json {
            Some(ref content) => {
                for i in 0..content.animes.len()
                {
                    if search(content.animes[i].anime.clone().to_lowercase(), self.debugged_payload.clone().to_lowercase()) && count < 9
                    {
                        count += 1;
                        background.push(content.animes[i].background.clone());
                        cards.push(html!{
                            <li class="card" style="background: black">
                            <AppAnchor route=AppRoute::Eps(i as u64)>
                                <a class="card-image" style=format!("background-image: url({});", content.animes[i].background.clone()) loading="lazy">
                                    <a class="card-description">
                                        <strong><h2>{content.animes[i].anime.clone()}</h2></strong>
                                        <p>{"Assistir agora"}</p>
                                    </a>
                                </a>
                            </AppAnchor>
                            </li>
                        })
                    }
                }

                html! {
                    <>
                        <carousel::Model background=self.export_background()/>
                        <section style="background-color: #25262F;">
                        <div class="mx-auto" style="width: 250px;">
                                <div class="field has-addons" style="padding-top: 80px;">
                                    <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Encontre seu anime"/>
                                </div>
                            </div>
                            // <h3>{"Resultados:"}</h3>
                            <ul class="card-list con-cards" >
                                {for cards.clone()}
                            </ul>
                        </section>
                    </>
                }
            }
            None => {
                html! {}
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { 
                <>
                <carousel::Model background=vec!["https://jaqorbelize.com/wp-content/uploads/blur-1.png".to_string()]/>
                        <section style="background-color: #25262F;">
                        <div class="mx-auto" style="width: 250px;">
                            <div class="control is-loading field has-addons">
                                <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Carregando"/>
                            </div>
                            // <div class="field has-addons" style="padding-top: 80px;">
                            //     <input class="input is-rounded" type="text" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Encontre seu anime"/>
                            // </div>
                        </div>
                            <ul class="card-list" >
                                // {for cards.clone()}
                            </ul>
                            {self.content()}
                            {self.content_ecchi()}
                            {self.content_shounen()}
                        </section>
                </> 
            }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }

    fn export_background(&self) -> Vec<String>
    {
        let mut background: Vec<String> = Vec::new();
        match self.json
        {
            Some(ref content) => 
            {
                for i in 0..content.animes.len()
                {
                    background.push(content.animes[i].background.clone());
                }
            },
            None => {
                background.push("none".to_string())
            }
        }
        background
    }
}
impl Component for LoadPosts {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self {
            payload: String::default(),
            debugged_payload: format!("{}", "none"),
            fetch_task: None,
            json: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            Msg::Payload(payload) => {
                if payload != self.payload {
                    self.debugged_payload = format!("{}", payload);
                    if self.debugged_payload == ""
                    {
                        self.debugged_payload = format!("{}", "none");
                    }
                    self.payload = payload;
                    true
                } else {
                    false
                }
            }
            GetInfo => {
                let request = Request::get("https://gist.githubusercontent.com/GozoDeAvestruz/d8912a0733e758fbc89324b16b9cea44/raw/a0ee97ea423b96049ddb58126fece4a03effc703/cards.json")
                    .body(Nothing)
                    .expect("Não foi possível efetuar o request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Struture, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("Falha ao iniciar o request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(dados) => {
                        self.json = Some(dados);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() }   
                {self.content()}
                {self.content_ecchi()}
                {self.content_shounen()} 
                {self.content_romance()}
            </>
        }
    }
}