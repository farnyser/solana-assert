# Assert program

Simple program to demonstrate how to assert a token balance is <= or >= to a given amount

## CLI usage

Assert wallet balance is > 1 for token `7PprzfgySGixbCwR4bhuX7oBsns3wi7F9AU2gV1cWAP7` (some token mint address on devnet)

```
assert_client \
    -c https://api.devnet.solana.com \
    -w solana_id.json \
    assert-greater-than 1000000000 \
    --mint 7PprzfgySGixbCwR4bhuX7oBsns3wi7F9AU2gV1cWAP7
```

### Help

```Assert CLI

Usage: assert_client [OPTIONS] --wallet <WALLET> <COMMAND>

Commands:
  assert-lower-than    
  assert-greater-than  
  help                 Print this message or the help of the given subcommand(s)

Options:
  -w, --wallet <WALLET>            The wallet pubkey
  -c, --cluster-url <CLUSTER_URL>  [default: http://localhost:8899]
  -h, --help                       Print help
```
