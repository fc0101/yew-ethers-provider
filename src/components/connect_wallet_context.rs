use yew::prelude::*;
use web3::transports::eip_1193::{Eip1193, Provider};

#[derive(Clone, Debug, PartialEq)]
pub struct UserWallet {
    pub is_connected: UseStateHandle<bool>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ConnectWalletProvider(props: &Props) -> Html {
    let is_connected = use_state(|| false);

    let ctx = use_state(|| UserWallet {
        is_connected,
    });
    html! {
        <ContextProvider<UserWallet> context={(*ctx).clone()}>
            {for props.children.iter()}
        </ContextProvider<UserWallet>>
    }
}

impl UserWallet {
    pub async fn connect(&self) {
        
        let provider: Provider = Provider::default().unwrap().unwrap();
        
        let transport = Eip1193::new(provider);
        
        let web3 = web3::Web3::new(transport);
        
        web3.eth().request_accounts().await.unwrap();
        self.is_connected.set(true);
    }
    
}