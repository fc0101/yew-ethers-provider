use yew::prelude::*;
use yew_ethers_provider::{ConnectButton, ConnectWalletProvider};

#[function_component(App)]
fn app() -> Html {

    html! {
        <div>
            <ConnectWalletProvider>
                <ConnectButton />
            </ConnectWalletProvider>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}