// BakerySwap.rs

use super::dex::BaseDex;
use super::Dex;
use async_trait::async_trait;
use ethers::{prelude::*, types::Address};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BakerySwap {
    base_dex: BaseDex,
}

static BAKERYSWAP_ROUTER_ABI_JSON: &'static [u8] =
    include_bytes!("../../resources/BakerySwapRouterABI.json");

impl BakerySwap {
    pub fn new(
        provider: Arc<NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>>,
        router_address: Address,
    ) -> Self {
        Self {
            base_dex: BaseDex::new(provider, router_address),
        }
    }
}

#[async_trait]
impl Dex for BakerySwap {
    async fn initialize(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.base_dex
            .create_router_contract(BAKERYSWAP_ROUTER_ABI_JSON)
            .await
    }

    fn clone_box(&self) -> Box<dyn Dex + Send + Sync> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        "BakerySwap"
    }

    fn router_contract(
        &self,
    ) -> Result<
        &Contract<NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>>,
        Box<dyn Error + Send + Sync + 'static>,
    > {
        self.base_dex.router_contract()
    }

    fn provider(
        &self,
    ) -> Arc<NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        self.base_dex.provider()
    }

    fn router_address(&self) -> Address {
        self.base_dex.router_address()
    }
}