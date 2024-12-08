FROM ubuntu:latest
LABEL authors="lordran"

ENTRYPOINT ["top", "-b"]