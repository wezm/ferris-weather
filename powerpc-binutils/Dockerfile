FROM debian:stable

RUN apt update && apt install -y --no-install-recommends binutils-powerpc-linux-gnu

ENTRYPOINT ["powerpc-linux-gnu-objcopy"]
