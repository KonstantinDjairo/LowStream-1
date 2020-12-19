<h1 align="center"> LowStream </h1>
<p align="justify"> Aplicacao de streaming performatica e de baixo custo de hardware </p>
<img alt="Website" src="https://img.shields.io/website?down_message=offline&up_message=online&url=https%3A%2F%2Flowstream.tk"> 
<blockquote>By Alexandroviski, André, $enick & Pablo</blockquote> <a href="https://lowstreamcast.web.app/">Entrar no site(provisório)</a>

<h3>Objetivos</h3>

  - [x] Implentação do wasm no projeto; **Concluido :heavy_check_mark:**
  - [x] Implentação do Bulma.io; **Concluido :heavy_check_mark:**
  - [x] Player de video **Concluido :heavy_check_mark:**
  - [ ] Suporte ao ybc; **Em desenvolvimento :warning:**
  - [x] Aplicar dinamismo nas funcionalidades; **Concluido :heavy_check_mark:**
  - [x] Router; **Concluido :heavy_check_mark:**
  - [ ] Framework Diesel;
  - [ ] Upload videos;
    - [ ] Like e Dislike;
    - [ ] Comentários nos videos;
    - [ ] ~~Download video~~ 
  - [ ] Database para forms de logins;
  - [ ] ~~Adcionar acesso de moderador~~ 
  - [ ] transação descentralizada;
  
  <h3>ATENÇÃO!</h3>
    *EXECUTE TODOS OS COMANDOS EM ```LOWSTREAM/```, E NÃO EM ```LOWSTREAM/algum_diretorio```*
  
  
  <h3>Build</h3>
  
  <h4>Trunk</h4>
  
  Precisa ter instaldo o compilador *Rust*
  Depois de instalado, agora instale os seguintes ferramentas do rustup.

  
  ```
  rustup target add wasm32-unknown-unknown
  ```
  
  ```
  cargo install trunk && cargo install trunk wasm-bindgen-cli
  ```
  Depois de tudo instalado nos conformes, execute o comando:
  *Se quiser pode dá só o comando ```trunk serve```, irá funcionar da mesma forma*
  
  ```
  trunk build && trunk serve
  ```

  <h3>Build para Linux</h3>

				  
  <h4>wasm-bindgen-cli</h4>
  
  Pirmeiro instale o gerador de JavaScript.

  ```
  cargo install wasm-bindgen-cli
  
  ```
  
  Ferramenta do Rustup com suporte a WebAssembly.

  ```
  rustup target add wasm32-unknown-unknown
  ```

  Trunk é um construtor de aplicativo web em WASM para 
Rust, é necessária a instalção.
 
  ```
  cargo install trunk && cargo install trunk wasm-bindgen-cli
  ```
  
  O comando abaixo irá gerar um arquivo .wasm.

  ```
  cargo build --target wasm32-unknown-unknown

  ```
  Esse comando irá gerar um conjunto de arquivos
contendo o WebAssembly compilado do seu aplicativo e um
wrapper JavaScript que carregará o binário Wasm e o
executará;


  ```
  wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/debug/appname.wasm --no-typescript
  ```
  O comando abaixo vai tornar o servidor ativo, para 
que seu acesso possa ser possibilitado.

  ```
  trunk serve
  ```
  Sempre que houver alguma modificação, o seu .wasm precisa ser atualizado então execute:
  
  ```
  cargo build --target wasm32-unknown-unknown && trunk serve

  ```

  <img alt="Noooo a imagem n presta T_T" src="https://i.pinimg.com/564x/05/a8/0e/05a80e4c78c3bd767650229f0407e162.jpg"> 
