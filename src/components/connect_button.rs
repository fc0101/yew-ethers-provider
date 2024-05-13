use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

use crate::UserWallet;

#[function_component]
pub fn ConnectButton() -> Html {

    let user_wallet = use_context::<UserWallet>().expect(
        "You have to wrap ContextProvider.",
    );

    let connect = {
        let user_wallet = user_wallet.clone();
        Callback::from(move |_| {
            let user_wallet = user_wallet.clone();
            spawn_local(async move {
                user_wallet.connect().await;
            });
        })
    };

    html! {
        <button onclick={connect}>{ "Connect Wallet" }</button>
    }
}