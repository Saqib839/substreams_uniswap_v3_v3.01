# prerequisites:
# install  go, protobuf and rust
                                sudo snap install go

                                chmod +x ~/go
                                export PATH=$PATH:$(go env GOPATH)/bin
                                echo 'export PATH=$PATH:$(go env GOPATH)/bin' >> ~/.bashrc
                                source ~/.bashrc


                                mkdir -p ~/bin
                                curl -sSL https://github.com/bufbuild/buf/releases/download/v1.21.0/buf-Linux-x86_64 -o ~/bin/buf
                                chmod +x ~/bin/buf
                                echo 'export PATH=$PATH:$HOME/bin' >> ~/.bashrc
                                source ~/.bashrc
                                buf --version

                                curl https://sh.rustup.rs -sSf | sh
                                source $HOME/.cargo/env
                                rustc --version
                                cargo --version
                                buf --version

# Install the substreams CLI
Used for connecting to endpoints, streaming data in real time, and packaging custom modules.

Homebrew installation
brew install streamingfast/tap/substreams
Pre-compiled binary installation
There are several CLI binaries available for different operating systems. Choose the correct platform in the CLI releases page.

# If you are on MacOS, you can use the following command:

        LINK=$(curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | awk "/download.url.*$(uname -s | tr '[:upper:]' '[:lower:]')_$(uname -m)/ {print \$2}" | sed 's/"//g')
        curl -L  $LINK  | tar zxf -

# If you are on Linux, you can use the following command:
# Use correct binary for your platform
        LINK=$(curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | awk "/download.url.*linux_$(uname -m)/ {print \$2}" | sed 's/"//g')
        curl -L  $LINK  | tar zxf -

git clone https://github.com/streamingfast/substreams
        cd substreams
        go install -v ./cmd/substreams
        Important: Add $HOME/go/bin to the system path if it's not already present.

# Validation of installation
Run the substreams CLI passing the --version flag to check the success of the installation.
            substreams --version

# Generate substreams auth token 
            substreams auth 
            . ./.substreams.env

# Once installed go to subtreams packages and download desired package (uniswap_v3_v...) 
# For gui 
    substreams gui https://spkg.io/streamingfast/uniswap-v3-v0.2.10.spkg graph_out 
# For cli 
    substreams run -e mainnet.eth.streamingfast.io:443 https://spkg.io/streamingfast/uniswap-v3-v0.2.10.spkg graph_out