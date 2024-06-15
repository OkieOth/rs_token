![WIP](https://img.shields.io/badge/work%20in%20progress-red)

# rs_token
Example crate to provide a JWT token that refreshs itself before it expires

The repo was developed and tested on:
* Ubuntu 24.04
* docker 26.1.3
* Rust 1.77.2
* curl
* jq

# Usage
To start a test environment type `./compose_env.sh start`. It brings up a docker
compose env that contains the latest Keycloak Image. Keycloak is initialized
with a test realm and a test client (`./docker-compose/keycloak/import.json`)

To check Keycloak in the browser go to the following URL after starting the
compose env `http://localhost:8080/` login with user 'admin' and password
'secretpassword'

## Grab a client token with curl

```shell
curl --request POST \
  --url http://localhost:8080/realms/test-realm/protocol/openid-connect/token \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=test-client' \
  --data 'grant_type=client_credentials' \
  --data 'client_secret=test-client999'
```

```shell
# fish shell variante to refresh the token
set -gx TOKEN $(curl --request POST \
    --url http://localhost:8080/realms/test-realm/protocol/openid-connect/token \
    --header 'Content-Type: application/x-www-form-urlencoded' \
    --data 'client_id=test-client' \
    --data 'grant_type=client_credentials' \
    --data 'client_secret=test-client999' | jq -r .refresh_token)

curl --request POST \
  --url http://localhost:8080/realms/test-realm/protocol/openid-connect/token \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=test-client' \
  --data 'grant_type=refresh_token' \
  --data 'refresh_token=eyJhbGciOiJIUzUxMiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJjNjI3YzM5NC1iZWY3LTQwZGQtYjA0NC1lMjMwOWUzODlmY2QifQ.eyJpYXQiOjE3MTg0NzY4NDYsImp0aSI6IjhmZWU0YzhkLTJjNmUtNGQxMS1hMGQzLTRiNWRmODkzYjFjYSIsImlzcyI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MC9yZWFsbXMvdGVzdC1yZWFsbSIsImF1ZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MC9yZWFsbXMvdGVzdC1yZWFsbSIsInN1YiI6IjM3MThjMDdkLTRjMzYtNDNiMy1iNWY4LTNiMDZiOTQyNWFkNSIsInR5cCI6Ik9mZmxpbmUiLCJhenAiOiJ0ZXN0LWNsaWVudCIsInNpZCI6Ijc4ZjEzN2M5LWUzNGUtNDRjZi04MGUyLTBkOGMwZjhmM2M0NCIsInNjb3BlIjoiYWNyIGVtYWlsIGJhc2ljIHJvbGVzIHdlYi1vcmlnaW5zIG9mZmxpbmVfYWNjZXNzIHByb2ZpbGUifQ.b20qkP_CtE_xF5v64zNGKAoYdokcjPPI2alEBus9IShTx7DF6uwdndpgCYrc0sf0DfILuLJ2bC3fo3ZfhtQbPQ' \
  --data 'client_secret=test-client999'
```