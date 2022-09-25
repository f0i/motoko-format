ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    libudev-dev bash-completion rlwrap

RUN DPRINT_INSTALL="/usr/" (curl -fsSL https://dprint.dev/install.sh | sh)

 
