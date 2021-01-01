use yew::prelude::*;
use crate::{
    switch::{AppAnchor, AppRoute},
};
use std::{thread, time};

// mod components;
// use components::{card_post::Card};

fn sleep()
{
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
}

// macro_rules! classes {
//     ($classe:expr, $classe_condition:expr) => {
//         {format!("{} {}", $classe, $classe_condition)}
//     };
// }

pub enum Msg
{
    Blur
}

pub struct Home
{
    string: String,
    iterator: i32,
    link: ComponentLink<Self>
}
impl Component for Home
{
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            string: String::default(),
            iterator: 8,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::Blur => {
                for i in self.iterator..0
                {
                    self.string = format!("filter: blur({}px)", i);
                    sleep();
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <>
                <section class="hero is-medium is-dark is-bold has-background" style="padding-top: 40px">
                    <img src="https://i.pinimg.com/originals/30/94/e0/3094e0fd1114787639e8e334a840ca02.jpg" class="hero-background is-transparent"/>
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                {"Seja muito Bem-Vindo(a)"}
                                <span class="tag is-white">
                                        {"new"}
                                </span>
                            </h1>
                            <h2 class="subtitle">
                                {"Somos uma plataforma de streaming simples, performática  e funcional."}
                            </h2>
                        </div>
                    </div>
                </section>                

                 <section style="background-color: #25262F;">
                 <div class="container has-text-centered" style="padding-top: 10px">
                        <h1 class="title" style="color: white">
                            <strong>{"Acesse a aba animes acima"}</strong>
                        </h1>
                </div>
                <div class="content">
                                    <div class="con-cards">
                                    <a href="http://lowstream.tk/?id=162" target="_blank">
                                            <div class="card">
                                                <h3>{"4.6"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://highschooldxdrpg-maniahotel.weebly.com/uploads/2/1/3/2/21328416/5181061.png?442" alt=""/>
                                                    <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"High School DxD"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=347" target="_blank">
                                            <div class="card">
                                            <h3>{"4.1"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/84/76/00/847600e526013b08c6f02d1a227eb4c2.png" alt=""/>
                                                    <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Shokugeki no Shoma"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=263" target="_blank">
                                            <div class="card">
                                                <h3>{"4.8"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://1.bp.blogspot.com/-nLhfDfrYSVo/X4nIPq_l2wI/AAAAAAAAC2Q/IORpXVXlyBgKo9c4vNnqeeV7QPud4_jvwCLcBGAsYHQ/s1600/Monster%2BMusume%2Bno%2BOishasan.png" alt=""/>
                                                    <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Monster Musume no Oisha-san"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=283" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://thicc.mywaifulist.moe/waifus/15517/b2103ecb978045890d878ebb0831a146d40b6fb2331e2436e1b5bc0491f7d843_thumb.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/5.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"No Game No Life"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=282" target="_blank">
                                            <div class="card">
                                                <h3>{"4.2"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://1.bp.blogspot.com/-d1tDdXg-5b8/VyVi9UOAYZI/AAAAAAAAF4U/NrRSic34ONMRRDdPTMGViTJyIwoC3KvlQCLcB/s1600/nekomonogatari__shiro____anime_icon_by_ggonido-d6d9s4t.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/6.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Nisemonogatari"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=276" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://bestofcomicbooks.com/wp-content/uploads/2019/03/Tsubasa-Hanekawa-near-nude.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/7.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Nekomonogatari (Kuro)"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                 <a href="http://lowstream.tk/?id=83" target="_blank">
                                            <div class="card">
                                                <h3>{"4.6"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://www.nicepng.com/png/full/53-531079_death-note-png-death-note.png" alt=""/>
                                                    <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Death Note"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=338" target="_blank">
                                            <div class="card">
                                            <h3>{"4.1"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/7c6f1009-9bed-42c1-90b2-5672c47100ef/d7axq72-b7025794-b93d-4abc-9f3f-acee56ee5499.png/v1/fill/w_1024,h_1402,strp/shingeki_no_kyojin_png_by_bloomsama_d7axq72-fullview.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD0xNDAyIiwicGF0aCI6IlwvZlwvN2M2ZjEwMDktOWJlZC00MmMxLTkwYjItNTY3MmM0NzEwMGVmXC9kN2F4cTcyLWI3MDI1Nzk0LWI5M2QtNGFiYy05ZjNmLWFjZWU1NmVlNTQ5OS5wbmciLCJ3aWR0aCI6Ijw9MTAyNCJ9XV0sImF1ZCI6WyJ1cm46c2VydmljZTppbWFnZS5vcGVyYXRpb25zIl19.3Ru6w3-GQ4Xal4lb9R5CPQywclD-0sd0YJ-gy7OD210" alt=""/>
                                                    <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Shingeki no Kyojin"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=116" target="_blank">
                                            <div class="card">
                                                <h3>{"4.2"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/9f/ed/f4/9fedf491e770e5a440bc04225fa634bf.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/6.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Fullmetal Alchemist"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=52" target="_blank">
                                            <div class="card">
                                                <h3>{"4.8"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/79/67/fe/7967feedae6a76b044fc407a1a3026cf.png" alt=""/>
                                                    <img class="blur" src="https://jaqorbelize.com/wp-content/uploads/blur-1.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Boku no Hero"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=273" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/16825168-8b56-4bef-80cb-89abe4d1be01/dcw71ku-ab9b3fa0-5f70-4e38-84c4-ff52497fc273.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3sicGF0aCI6IlwvZlwvMTY4MjUxNjgtOGI1Ni00YmVmLTgwY2ItODlhYmU0ZDFiZTAxXC9kY3c3MWt1LWFiOWIzZmEwLTVmNzAtNGUzOC04NGM0LWZmNTI0OTdmYzI3My5wbmcifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6ZmlsZS5kb3dubG9hZCJdfQ.uLjGHYRxDt9Dzvj60iF-o72WtyJtrGj91uRahE9cMAc" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/5.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Naruto"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=105" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://www.animeunited.com.br/wp-content/uploads/2018/08/Fairy-Tail-Dice-Magic.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/5.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Fairy Tail"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=255" target="_blank">
                                            <div class="card">
                                                <h3>{"4.2"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/7c6f1009-9bed-42c1-90b2-5672c47100ef/d6c2ljw-aed5c0dd-06e2-4379-a843-9a1249f8dc4b.png/v1/fill/w_575,h_864,strp/mirai_nikki_png_by_bloomsama_d6c2ljw-fullview.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD04NjQiLCJwYXRoIjoiXC9mXC83YzZmMTAwOS05YmVkLTQyYzEtOTBiMi01NjcyYzQ3MTAwZWZcL2Q2YzJsanctYWVkNWMwZGQtMDZlMi00Mzc5LWE4NDMtOWExMjQ5ZjhkYzRiLnBuZyIsIndpZHRoIjoiPD01NzUifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6aW1hZ2Uub3BlcmF0aW9ucyJdfQ.LQYKhBB66Nef7BZp76g9VAHMfwhiui6l666-uEHZAN4" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/6.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Mirai Nikki"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=289" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/a3/ea/bb/a3eabbe003c6c2c8de86f2eb21efb9ad.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/7.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Noragami"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=26" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/ce/25/a9/ce25a92e84110abfc509022004ddc3b1.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/7.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Ao no Exorcist"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=16" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://s-media-cache-ak0.pinimg.com/originals/d0/50/30/d05030f5913866031215c6856ac34adf.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/7.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Akame ga Kill!"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        <a href="http://lowstream.tk/?id=270" target="_blank">
                                            <div class="card">
                                                <h3>{"4.9"}</h3>
                                                <i class="bx bx-heart"></i>
                                                <div class="con-img">
                                                    <img src="https://i.pinimg.com/originals/c6/68/ba/c668ba3d743bded8f97e77cd7163131d.png" alt=""/>
                                                    <img class="blur" src="https://raw.githubusercontent.com/luisDanielRoviraContreras/img/master/7.png" alt=""/>
                                                </div>
                                                <div class="con-text">
                                                    <h2>
                                                        {"Nanatsu no Taizai"}
                                                    </h2>
                                                </div>
                                            </div>
                                        </a>
                                        </div>
                                            </div>
                </section>                
            </>
        }
    }
}