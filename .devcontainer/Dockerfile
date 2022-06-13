
# See here for image contents: https://github.com/f0i/devcontainer-dfinity
#FROM f0ide/devcontainer-dfinity

# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get -y install --no-install-recommends <your-package-list-here>

# [Optional] run this to upgrade to the latest version of dfx
# RUN yes | dfx upgrade

# [Optional] Uncomment if you want to install an additional version of node using nvm
# ARG EXTRA_NODE_VERSION=10
# RUN su node -c "source /usr/local/share/nvm/nvm.sh && nvm install ${EXTRA_NODE_VERSION}"

# [Optional] Uncomment if you want to install more global node modules
# RUN su node -c "npm install -g <your-package-list-here>"

# [Optional] Uncomment if you want to use Rust
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -t wasm32-unknown-unknown -y

ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    libudev-dev bash-completion rlwrap

RUN DPRINT_INSTALL="/usr/" (curl -fsSL https://dprint.dev/install.sh | sh)

 
