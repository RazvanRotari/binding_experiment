#!/usr/bin/env bash

cd build
ninja -j2 && ./bin/binding_example
