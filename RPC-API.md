# `bitcoin-faucet` JSONRPC API specification

## Methods

- [`fund`]

## [`fund`]

Fund provided address/addresses with given amount/amounts. There are three
possible schems that could be accepted by this method:

* [`address`, `amount`] - send `amount` to `address`.
* [[`addresses`, ...], `amont`] - send `amount` to each address in `addresses`.
* [[`address`, `amount`], ...] - send `amount` to each address in the list.

Where address - is any valid bitcoin address, and amount - is a number of
satoshis to send.

The result is transaction id of the funding transaction.

### Examples

Send 1 BTC to `bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh`:


```sh
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", 100000000]]}' \
     http://127.0.0.1:18777
```

Response:

``` json
{
    "jsonrpc":"2.0",
    "result":"79d34c88cb9ce85b6e0f7048a7c31b4894c59576b1fca6e9cab07b24b27f5726",
    "id":"id"
}
```

Send 1 BTC to [`bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh`, `bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh`]:

```sh
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[[["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"], 100000000]]}' \
     http://127.0.0.1:18777
```

Send 1 to `bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh` and 2 to `bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh`:

```sh
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[[["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", 100000000], ["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", 200000000]]]}' \
     http://127.0.0.1:18777
```

[`fund`]: #fund
