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

Send 1 BTC to `bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt`:


```sh
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[["bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt", 100000000]]}' \
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

Send 1 BTC to [`bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt`, `bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt`]:

```sh
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[[["bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt", "bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt"], 100000000]]}' \
     http://127.0.0.1:18777
```

Send 1 to `bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt` and 2 to `bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt`:

```sh
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[[["bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt", 100000000], ["bcrt1qkqnzdx9krpzuqlultkcdet3v5um75exnzfm9kt", 200000000]]]}' \
     http://127.0.0.1:18777
```

[`fund`]: #fund
