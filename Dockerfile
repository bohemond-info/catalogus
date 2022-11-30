# To build:
#  docker build . --rm -t bohemond:local .
# To build w/bash for debugging:
#  docker build --rm --build-arg FINAL_BASE=bash -t bohemond:local .
# To run:
#  docker run -d --name bohemond -p 8000:8000 bohemond:local
#  -> should make app available at http://localhost:8000
# To stop:
#  docker stop bohemond
# To remove:
#  docker rm bohemond
# To debug:
#  docker run -it --entrypoint /bin/sh bohemond:local
# To get logs:
#  docker ps -a     (Get the CONTAINER ID)
#  docker logs <CONTAINER ID>
ARG PROJECT=bohemond
ARG RUST_BUILDER_CHANNEL=stable
# ARG RUST_BUILDER_TAG=latest
ARG RUST_BUILDER_TAG=1.48.0
ARG JS_BUILDER_TAG=latest
ARG FINAL_BASE=scratch

# Compile Rust Back-end
FROM ekidd/rust-musl-builder:$RUST_BUILDER_TAG as rust_builder
ARG PROJECT
ENV SRC_BE=api-$PROJECT
ENV SRC_LIB=data_transcriber
ENV ROCKET_ENV=production

WORKDIR /usr/app/
USER root
RUN chown -R rust:rust /usr/app
# Hack to make this build work both locally and in Github Actions
RUN  chown -R rust:rust /opt/rust/rustup/* || true
USER rust

# rocket_cors still relies on the nightly build
# Can this just draw from the rust-toolchain.yaml?
RUN rustup toolchain install nightly
RUN rustup override set nightly
RUN rustup target add x86_64-unknown-linux-musl

COPY --chown=rust ./$SRC_BE ./$SRC_BE
COPY --chown=rust ./$SRC_LIB ./$SRC_LIB

RUN cargo update --manifest-path=$SRC_BE/Cargo.toml
RUN cargo build --release --manifest-path=$SRC_BE/Cargo.toml
RUN cargo install --target x86_64-unknown-linux-musl --path $SRC_BE/
RUN ls -la
RUN ls -la $SRC_BE/

# Compile Javascript Front-end
#FROM node:$JS_BUILDER_TAG as js_builder
#ARG PROJECT
#ARG SRC_FE=wfe-$PROJECT
#ENV NODE_ENV=production
#
#COPY $SRC_FE ./frontend_build
#WORKDIR frontend_build
#
#RUN node -v && yarn -v && npm -v
#RUN yarn install --pure-lockfile --production=false --audit
#RUN yarn run build

# Final Build Stage
#FROM $FINAL_BASE
FROM ekidd/rust-musl-builder:$RUST_BUILDER_TAG
ARG PROJECT
ENV APP_PROFILE=production
ENV NODE_ENV=production
ENV APP=api-$PROJECT

COPY --from=rust_builder /home/rust/.cargo/bin/$APP .
COPY --from=rust_builder /usr/app/$APP/Rocket.toml .
COPY --from=rust_builder /usr/app/$APP/$PROJECT.yaml .
COPY --from=rust_builder /usr/app/$APP/resources ./resources
#COPY --from=js_builder --chown=1000 ./frontend_build/dist ./static
RUN ls -la

USER 1000
# This has to be hard-coded while using scratch as base
CMD ["./api-bohemond"]
ENTRYPOINT ["./api-bohemond"]
