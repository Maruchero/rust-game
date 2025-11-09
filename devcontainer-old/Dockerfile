FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-22.04

ARG USERNAME=vscode
ENV DEBIAN_FRONTEND=noninteractive

# Install essential packages for building Rust games (Vulkan/GL, audio, input libs)
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    software-properties-common \
  && add-apt-repository -y universe \
  && apt-get update \
  && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    ca-certificates \
    cmake \
    pkg-config \
    clang \
    clang-format \
    ninja-build \
    libx11-dev \
    libxkbcommon-dev \
    libwayland-dev \
    libegl1-mesa-dev \
    libgl1-mesa-dev \
  libvulkan-dev \
  vulkan-tools \
    libpulse-dev \
    libsdl2-dev \
    libasound2-dev \
    libxcursor-dev \
    libxi-dev \
    libxrandr-dev \
    libudev-dev \
    python3 \
    python3-pip \
  && rm -rf /var/lib/apt/lists/*

# Install rustup for the non-root user (vscode)
USER ${USERNAME}
ENV HOME=/home/${USERNAME}
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH=${HOME}/.cargo/bin:${PATH}

USER root

# Create workspace directory and set ownership
RUN mkdir -p /workspace && chown ${USERNAME}:${USERNAME} /workspace

WORKDIR /workspace

CMD ["/bin/bash"]
