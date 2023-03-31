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
    let response = gloo::net::http::Request::get("https://httpbin.org/response-headers")
        .query([
            ("freeform", "I_set_this"),
            ("Access-Control-Expose-Headers", "*"),
            (
                "Authorization",
                "Bearer q39280jrhiesnthyuwfljguywflpjgyuwlfjuielaf.q3;94857jfruisenhdriefsnea",
            ),
            ("access-control-allow-credentials", "false"),
        ])
        .send()
        .await
        .expect("Error sending request");

    for (key, value) in response.headers().entries() {
        gloo::console::log!(key, value);
    }
}
