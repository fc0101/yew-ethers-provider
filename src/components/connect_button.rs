use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;
use web3::transports::eip_1193::{Eip1193, Provider};

#[function_component]
pub fn ConnectButton() -> Html {
    let connect = {
        Callback::from(move |_| {
            spawn_local(async {
                connect_wallet().await;
            });
        })
    };

    html! {
        <div onclick={connect}>{ "Connect Wallet" }</div>
    }
}

async fn connect_wallet() {
    let provider: Provider = Provider::default().unwrap().unwrap();

    let transport = Eip1193::new(provider);

    let web3 = web3::Web3::new(transport);

    web3.eth().request_accounts().await.unwrap();
}
