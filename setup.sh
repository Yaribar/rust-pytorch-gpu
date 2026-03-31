#!/usr/bin/env bash
source env.sh
source post-install.sh
#append it to bash so every shell launches with it
echo 'source /opt/venv/bin/activate' >> ~/.bashrc
