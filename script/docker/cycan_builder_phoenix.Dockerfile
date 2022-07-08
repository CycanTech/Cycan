# This is the build stage for Cycan. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /cycan
COPY . /cycan
RUN rustup install nightly-2021-03-15-x86_64-unknown-linux-gnu
RUN rustup default nightly-2021-03-15-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-15-x86_64-unknown-linux-gnu
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the Cycan binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Multistage Docker image for Cycan: a platform for web3" \
	io.cycan.image.type="builder" \
	io.cycan.image.authors="tech@cycan.network" \
	io.cycan.image.vendor="Cycan Technologies" \
	io.cycan.image.description="Cycan" \
	io.cycan.image.source="https://github.com/CycanTech/Cycan/blob/${VCS_REF}/script/docker/cycan_builder_phoenix.Dockerfile" \
	io.cycan.image.documentation="https://github.com/CycanTech/Cycan"

COPY --from=builder /cycan/target/release/phoenix /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /cycan cycan && \
	mkdir -p /data /cycan/.local/share/cycan && \
	chown -R cycan:cycan /data && \
	ln -s /data /cycan/.local/share/cycan && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/phoenix --version
	
COPY --from=builder /cycan/keystore /cycan/keystore

USER cycan
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/phoenix"]
