use yew::prelude::*;

use crate::{
    switch::{AppAnchor, AppRoute},
    components::card::Card
};

pub struct Shounen;
impl Component for Shounen {
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
                            {"Shounen"}
                        </h2>
                    </div>
                </div>
                    <div class="content">
                        <div class="con-cards">
                        <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(109)>
                            <Card 
                                score="4.6".to_string() 
                                sticker="https://www.nicepng.com/png/full/53-531079_death-note-png-death-note.png".to_string() 
                                name="Death Note".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(394)>
                            <Card 
                                score="4.1".to_string() 
                                sticker="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/7c6f1009-9bed-42c1-90b2-5672c47100ef/d7axq72-b7025794-b93d-4abc-9f3f-acee56ee5499.png/v1/fill/w_1024,h_1402,strp/shingeki_no_kyojin_png_by_bloomsama_d7axq72-fullview.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD0xNDAyIiwicGF0aCI6IlwvZlwvN2M2ZjEwMDktOWJlZC00MmMxLTkwYjItNTY3MmM0NzEwMGVmXC9kN2F4cTcyLWI3MDI1Nzk0LWI5M2QtNGFiYy05ZjNmLWFjZWU1NmVlNTQ5OS5wbmciLCJ3aWR0aCI6Ijw9MTAyNCJ9XV0sImF1ZCI6WyJ1cm46c2VydmljZTppbWFnZS5vcGVyYXRpb25zIl19.3Ru6w3-GQ4Xal4lb9R5CPQywclD-0sd0YJ-gy7OD210".to_string() 
                                name="Shingeki no Kyojin".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(147)>
                            <Card 
                                score="4.2".to_string() 
                                sticker="https://i.pinimg.com/originals/9f/ed/f4/9fedf491e770e5a440bc04225fa634bf.png".to_string() 
                                name="Fullmetal Alchemist".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(77)>
                            <Card 
                                score="4.8".to_string() 
                                sticker="https://i.pinimg.com/originals/79/67/fe/7967feedae6a76b044fc407a1a3026cf.png".to_string() 
                                name="Boku no Hero".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(325)>
                            <Card 
                                score="4.9".to_string() 
                                sticker="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/16825168-8b56-4bef-80cb-89abe4d1be01/dcw71ku-ab9b3fa0-5f70-4e38-84c4-ff52497fc273.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3sicGF0aCI6IlwvZlwvMTY4MjUxNjgtOGI1Ni00YmVmLTgwY2ItODlhYmU0ZDFiZTAxXC9kY3c3MWt1LWFiOWIzZmEwLTVmNzAtNGUzOC04NGM0LWZmNTI0OTdmYzI3My5wbmcifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6ZmlsZS5kb3dubG9hZCJdfQ.uLjGHYRxDt9Dzvj60iF-o72WtyJtrGj91uRahE9cMAc".to_string() 
                                name="Naruto".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(136)>
                            <Card 
                                score="4.9".to_string() 
                                sticker="https://www.animeunited.com.br/wp-content/uploads/2018/08/Fairy-Tail-Dice-Magic.png".to_string() 
                                name="Fairy Tail".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(306)>
                            <Card 
                                score="4.2".to_string() 
                                sticker="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/7c6f1009-9bed-42c1-90b2-5672c47100ef/d6c2ljw-aed5c0dd-06e2-4379-a843-9a1249f8dc4b.png/v1/fill/w_575,h_864,strp/mirai_nikki_png_by_bloomsama_d6c2ljw-fullview.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD04NjQiLCJwYXRoIjoiXC9mXC83YzZmMTAwOS05YmVkLTQyYzEtOTBiMi01NjcyYzQ3MTAwZWZcL2Q2YzJsanctYWVkNWMwZGQtMDZlMi00Mzc5LWE4NDMtOWExMjQ5ZjhkYzRiLnBuZyIsIndpZHRoIjoiPD01NzUifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6aW1hZ2Uub3BlcmF0aW9ucyJdfQ.LQYKhBB66Nef7BZp76g9VAHMfwhiui6l666-uEHZAN4".to_string() 
                                name="Mirai Nikki".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(343)>
                            <Card 
                                score="4.9".to_string() 
                                sticker="https://i.pinimg.com/originals/a3/ea/bb/a3eabbe003c6c2c8de86f2eb21efb9ad.png".to_string() 
                                name="Noragami".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(44)>
                            <Card 
                                score="4.9".to_string() 
                                sticker="https://i.pinimg.com/originals/ce/25/a9/ce25a92e84110abfc509022004ddc3b1.png".to_string() 
                                name="Ao no Exorcist".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(30)>
                            <Card 
                                score="4.9".to_string() 
                                sticker="https://s-media-cache-ak0.pinimg.com/originals/d0/50/30/d05030f5913866031215c6856ac34adf.png".to_string() 
                                name="Akame ga Kill!".to_string() 
                                />
                        </AppAnchor>
                            </div>
                            <div class="padding-40px">
                        <AppAnchor route=AppRoute::Eps(322)>
                            <Card 
                                score="4.9".to_string() 
                                sticker="https://i.pinimg.com/originals/c6/68/ba/c668ba3d743bded8f97e77cd7163131d.png".to_string() 
                                name="Nanatsu no Taizai".to_string() 
                                />
                        </AppAnchor>
                            </div>
                        </div>
                    </div>
                </>
        }        
    }
}