use bitcoin::{address::NetworkUnchecked, Address, Txid};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[rpc(server)]
pub trait FaucetRpc {
    #[method(name = "fund")]
    async fn fund(&self, request: FundRequestParams) -> RpcResult<Txid>;
}

pub type RecipientWithAmount = (Address<NetworkUnchecked>, u64);

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum FundRequestParams {
    /// Send specified amount to a single address.
    Single(RecipientWithAmount),

    /// Send the same amount to multiple addresses.
    SameAmount(Vec<Address<NetworkUnchecked>>, u64),

    /// Send different amounts to multiple addresses.
    Multiple(Vec<RecipientWithAmount>),
}

impl From<FundRequestParams> for Vec<RecipientWithAmount> {
    fn from(value: FundRequestParams) -> Self {
        use FundRequestParams as Params;

        match value {
            Params::Single(recipient) => vec![recipient],
            Params::SameAmount(addresses, amount) => addresses
                .into_iter()
                .map(|address| (address, amount))
                .collect(),
            Params::Multiple(recipients) => recipients,
        }
    }
}
