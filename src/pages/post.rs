// use crate::{
//     data::FetchServiceExample,
//     switch::{AppAnchor, AppRoute},
// };
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub animeName: String,
}

pub struct Post;
impl Component for Post {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.props.neq_assign(props)
        false
    }

    fn view(&self) -> Html
    {
        html!{

            <>
            <section class="hero is-medium is-dark is-bold" style="padding-top: 40px">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                {"Seja muito Bem Vindo!"}
                            </h1>
                            <h2 class="subtitle">
                                {"Estamos muito agredecidos por está com gente até aqui >_<"}
                            </h2>
                        </div>
                    </div>
                </section>

                <div class="container is-max-desktop" >
                    <div class="notification primary" >
                        <div class="field">
                        <label class="label">{"Name"}</label>
                        <div class="control">
                            <input class="input" type="text" placeholder="Insira seu nome"/>
                        </div>
                        </div>
                        <div class="field">
                            <label class="label">{"Username"}</label>
                        <div class="control has-icons-left has-icons-right">
                            <input class="input" type="text" placeholder="belotoDeCera"/> //is-success
                            <span class="icon is-small is-left">
                                <i class="fas fa-user"></i>
                            </span>
                            <span class="icon is-small is-right">
                                <i class="fas fa-check"></i>
                            </span>
                        </div>
                        // <p class="help">{"Oba! Esse nick está disponível :)"}</p>
                        </div>
                        <div class="field">
                        <label class="label">{"Email"}</label>
                        <div class="control has-icons-left has-icons-right">
                            <input class="input" type="email" placeholder="hello@example.com"/> //is-danger
                            <span class="icon is-small is-left">
                            <i class="fas fa-envelope"></i>
                            </span>
                            <span class="icon is-small is-right">
                            <i class="fas fa-exclamation-triangle"></i>
                            </span>
                        </div>
                        // <p class="help is-danger">{"This email is invalid"}</p>
                        </div>
                        <div class="field">
                        <label class="label">{"Senha"}</label>
                        <div class="control">
                            <input class="input" type="password" placeholder="Insira uma senha"/>
                        </div>
                        </div>
                        <div class="field">
                        <div class="control">
                            <label class="checkbox">
                            <input type="checkbox"/>
                            {" Eu aceito com os "} <a href="#">{"termos e condições."}</a>
                            </label>
                        </div>
                        </div>
                        <div class="field is-grouped">
                        <div class="control">
                            <button class="button is-link">{"Registrar"}</button>
                        </div>
                        <div class="control">
                            <button class="button is-link is-light" href="lowstreamcast.web.app">{"Cancelar"}</button>
                        </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
