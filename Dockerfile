FROM leptos-builder-gnu as builder

WORKDIR /work

COPY . .

RUN mkdir -p target/site
RUN cargo leptos build --release

FROM scratch as app

ENV LEPTOS_OUTPUT_NAME=personal_site
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT=3001

USER 10001

WORKDIR /app

COPY --chown=10001:10001 --from=builder /work/target/site/ ./site/
COPY --chown=10001:10001 --from=builder /work/target/server/release/personal_site .

EXPOSE 3000:3000

ENTRYPOINT [ "/app/personal_site" ]
