#!/bin/bash

LINK=$(curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | awk "/download.url.*linux_$(uname -m)/ {print \$2}" | sed 's/"//g')
curl -L  $LINK  | tar zxf -
mv substreams /usr/bin/substreams
