#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd $SCRIPT_DIR
export DOCKER_BUILDKIT=1
docker build . -t feed-bouncer
docker image prune -f
