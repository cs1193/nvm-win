FROM rust:1.43.1

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

RUN apt-get update -yq
RUN apt-get install -y --no-install-recommends apt-utils dialog 2>&1
RUN apt-get install -y git openssh-client less iproute2 procps lsb-release
RUN apt-get install -y lldb python3-minimal libpython3.7
RUN apt-get install -y editorconfig

RUN rustup update 2>&1
RUN rustup component add rls rust-analysis rust-src rustfmt clippy 2>&1

RUN groupadd --gid ${USER_GID} ${USERNAME}
RUN useradd -s /bin/bash --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME}
RUN apt-get install -y sudo
RUN echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME}
RUN chmod 0440 /etc/sudoers.d/${USERNAME}

RUN apt-get autoremove -y
RUN apt-get clean -y
RUN rm -rf /var/lib/apt/lists/*

ENV DEBIAN_FRONTEND=noninteractive
