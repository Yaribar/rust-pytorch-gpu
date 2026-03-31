FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld lshw \
   && apt-get autoremove -y && apt-get clean -y

RUN apt-get update && apt-get -y install --no-install-recommends \
   ffmpeg \
   gcc \
   pciutils \
   python3 \
   python3-venv \
   python3-pip

#create virtual environment at a shared location accessible to any user
COPY requirements.txt /tmp/
COPY Makefile /tmp/
RUN python3 -m venv /opt/venv \
   && /opt/venv/bin/pip --disable-pip-version-check --no-cache-dir install -r /tmp/requirements.txt \
   && chmod -R a+rwX /opt/venv \
   && rm -rf /tmp/requirements.txt
