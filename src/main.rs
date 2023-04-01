use gloo::net::http::RequestCredentials;
use yew::{function_component, html, Html};
use yew_hooks::use_effect_once;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app_component() -> Html {
    use_effect_once(|| {
        gloo::console::log!("loaded");

        wasm_bindgen_futures::spawn_local(async {
            make_the_call().await;
        });

        || {}
    });

    html! {
        <h1>{"Exploring CORS"}</h1>
    }
}

async fn make_the_call() {
    let response = gloo::net::http::Request::get("http://localhost:3000")
        .mode(gloo::net::http::RequestMode::Cors)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .expect("Error sending request");

    for (key, value) in response.headers().entries() {
        gloo::console::log!(key);
    }

    let body = response.text().await.unwrap();

    gloo::console::log!("body:", body);
}
