FROM debian:buster-slim
ARG APP=/usr/src/app

ARG RELEASE

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 9090

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

ADD https://codeload.github.com/codecrunchers/family-tree-backend/tar.gz/v${RELEASE}
COPY static/ .

RUN chown -R $APP_USER:$APP_USER ${APP}/family_tree_backend && chmod u+x ${APP}/family_tree_backend


USER $APP_USER
WORKDIR ${APP}

CMD ["./family_tree_backend"]
