use yew::prelude::*;

use gloo_console::log;
use wasm_bindgen::JsValue;

#[function_component]
pub fn ConnectButton() -> Html {
    let connect = {
        Callback::from(|_|{
            log!(JsValue::from("click 'Connect Wallet' button"));
        })
    };

    html! {
        <div onclick={connect}>{ "Connect Wallet" }</div>
    }
}
