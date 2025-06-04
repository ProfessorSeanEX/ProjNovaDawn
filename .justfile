# Justfile – OmniCode Boot Tasks

default:
  just --summary

build:
  gcc -o bin/helloworld apps/helloworld.c

clean:
  rm -rf bin/*

run:
  ./bin/helloworld
