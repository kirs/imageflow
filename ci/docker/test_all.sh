#!/bin/bash

set -e
set -x

#export EXTRA_DOCKER_BUILD_PARAMS=--no-cache

docker_images=(build_if_gcc54 build_if_gcc49 build_if_gcc48)
for DOCKER_IMAGE in "${docker_images[@]}"
do
  ./test.sh ${DOCKER_IMAGE} stable
  ./test.sh ${DOCKER_IMAGE} nightly
done

