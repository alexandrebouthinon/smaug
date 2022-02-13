####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

ENV USER=einherjar
ENV APP=einherjar
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /$APP

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

ENV USER=einherjar
ENV APP=einherjar

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /var/app

COPY --from=builder /$APP/target/release/$APP /var/

USER $USER:$USER

CMD ["/var/app/einherjar"]