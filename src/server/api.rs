use bitcoin::{address::NetworkUnchecked, Address, Txid};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[rpc(server)]
pub trait FaucetRpc {
    #[method(name = "fund")]
    async fn fund(&self, request: FundRequestParams) -> RpcResult<Txid>;
}

pub type RecipientWithAmount = (Address<NetworkUnchecked>, u64);

#[derive(serde::Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    struct Case {
        raw: &'static str,
        expected: FundRequestParams,
    }

    fn do_test_fund_request_deser(case: Case) {
        let got: FundRequestParams = serde_json::from_str(case.raw).unwrap();

        assert_eq!(got, case.expected);
    }

    #[test]
    fn test_deser_fund_request_single() {
        let case = Case {
            raw: r#"["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", 100000000]"#,
            expected: FundRequestParams::Single((
                Address::from_str("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh").unwrap(),
                100000000,
            )),
        };

        do_test_fund_request_deser(case);
    }

    #[test]
    fn test_deser_fund_request_same_amount() {
        let case = Case {
            raw: r#"[["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"], 100000000]"#,
            expected: FundRequestParams::SameAmount(
                vec![Address::from_str("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh").unwrap(); 2],
                100000000,
            ),
        };

        do_test_fund_request_deser(case);
    }

    #[test]
    fn test_deser_fund_request_multiple() {
        let case = Case {
            raw: r#"[
                ["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", 100000000],
                ["bc1qj53cennpes9zshh0ul2ur9r07756g3crr8hxh9", 200000000]
            ]"#,
            expected: FundRequestParams::Multiple(vec![
                (
                    Address::from_str("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh").unwrap(),
                    100000000,
                ),
                (
                    Address::from_str("bc1qj53cennpes9zshh0ul2ur9r07756g3crr8hxh9").unwrap(),
                    200000000,
                ),
            ]),
        };

        do_test_fund_request_deser(case);
    }
}
