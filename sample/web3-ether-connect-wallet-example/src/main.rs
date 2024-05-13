use yew::prelude::*;
use yew_ethers_provider::ConnectButton;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <ConnectButton> </ConnectButton>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}