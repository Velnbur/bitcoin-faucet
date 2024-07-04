# `bitcoin-faucet`

A simple service for creating "funding" transaction to your local development
environment with batteries included.

## Usage

Usage example can be viewed in [`docker-compose.yaml`](./docker-compose.yaml)
with configs for faucet service and the Bitcoin node in [`configs`](./configs).

So just start a docker compose from project root:

```sh
docker compose up -d
```

And send bitcoins to your node using local `curl`:

```sh
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", 100000000]]}' \
     http://127.0.0.1:18777
```


Or one inside the container:

```sh
docker compose exec faucet curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"fund","params":[["bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", 100000]]}' \
     http://127.0.0.1:18777
```


## Developement

Build docker container:

```sh
nix build .#packages.aarch64-linux.docker
                    # ^- insert you arch
                    
docker load < result # load result image into daemon
```
