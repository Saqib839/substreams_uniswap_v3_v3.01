# Uniswap v3 Substreams package

[![Substreams Package](https://img.shields.io/badge/streamingfast%2Funiswap-v3?logo=bitcoin&logoColor=orange&label=spkg.io&color=blue)](https://substreams.dev/streamingfast/uniswap-v3/latest)
[![Open in DevPod!](https://devpod.sh/assets/open-in-devpod.svg)](https://devpod.sh/open#https://github.com/streamingfast/substreams-uniswap-v3)

[Substreams](https://substreams.streamingfast.io)-based Uniswap-v3 Substreams-powered-subgraph.

This module emits EntityChanges, and are written directly to `graph-node` without any AssemblyScript mappings therein.

This module covers all of the entities of the original `v3-subgraph` by the Uniswap team. It syncs much faster.

Launch the _devpod_ above and run:

```bash
make build
substreams gui map_pools_created \
  -e mainnet.eth.streamingfast.io:443 \
  -t +150
```

## Stream Entity changes

First, [authenticate](https://substreams.streamingfast.io/reference-and-specs/authentication), and run `sftoken` in your shell session.

This runs the `graph_out` module against a Substreams cluster:

```bash
substreams run https://github.com/streamingfast/substreams-uniswap-v3/releases/download/v0.2.8/substreams.spkg \
  graph_out \
  -e mainnet.eth.streamingfast.io:443 \
  -t +150
[...]
{
  "@module": "graph_out",
  "@block": 12369621,
  "@type": "sf.substreams.entity.v1.EntityChanges",
  "@data": {
    "entityChanges": [
      {
        "entity": "Factory",
        "id": "1f98431c8ad98523631ae4a59f267346ea31f984",
        "ordinal": "1",
        "operation": "CREATE",
        "fields": [
          {
            "name": "id",
            "newValue": {
              "string": "1f98431c8ad98523631ae4a59f267346ea31f984"
            }
          },
          {
            "name": "poolCount",
            "newValue": {
              "bigint": "0"
            }
          },
[...]
       ]
      },
     {
        "entity": "Pool",
        "id": "1d42064fc4beb5f8aaf85f4617ae8b3b5b8bd801",
        "ordinal": "927",
        "operation": "UPDATE",
        "fields": [
          {
            "name": "totalValueLockedToken0",
            "newValue": {
              "bigdecimal": "0.9999999999999999240000000000003427709097170609759698726797493006923644998096278868615627288818359375"
            },
            "oldValue": {
              "bigdecimal": "0"
            }
          }
        ]
      },
[...]
    ]
  }
}
```

## Hack on it

### Build `substreams-uniswap-v3`

```bash
make build
[...]
substreams run  -e mainnet.eth.streamingfast.io:443 substreams.yaml graph_out -t +150
[...]
try it :)
```

### Pack everything to release

```bash
substreams pack substreams.yaml
```
