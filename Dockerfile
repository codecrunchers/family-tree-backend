
FROM debian:buster-slim
ARG APP=/usr/src/app

ARG FTBE_RELEASE

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata curl\
    && rm -rf /var/lib/apt/lists/*

EXPOSE 9090

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

WORKDIR ${APP}
RUN curl -SL "https://github.com/codecrunchers/family-tree-backend/releases/download/${FTBE_RELEASE}/family_tree_backend" --output family_tree_backend 
RUN  chown -R $APP_USER:$APP_USER family_tree_backend && chmod u+x family_tree_backend
USER $APP_USER
CMD ["./family_tree_backend"]
