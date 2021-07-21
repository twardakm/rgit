FROM ubuntu:16.04 AS prepare-env-ubuntu-16-04

FROM prepare-env-ubuntu-16-04 AS build-ubuntu-16-04
RUN apt update
RUN apt install -y pkg-config rustc libssl-dev

FROM build-ubuntu-16-04 AS debug-build-ubuntu-16-04
WORKDIR /opt/rgit
COPY . .
RUN cargo build

FROM build-ubuntu-16-04 AS release-build-ubuntu-16-04
WORKDIR /opt/rgit
COPY . .
RUN cargo build --release

FROM scratch AS debug-ubuntu-16-04
COPY --from=debug-build-ubuntu-16-04 /opt/rgit/target/debug/rgit target/ubuntu-16-04/debug/

FROM scratch AS release-ubuntu-16-04
COPY --from=release-build-ubuntu-16-04 /opt/rgit/target/release/rgit target/ubuntu-16-04/release/
