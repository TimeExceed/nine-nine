FROM debian:10
COPY sources.list /etc/apt/
WORKDIR /opt/code/

RUN apt-get update \
    && apt-get clean
    
RUN apt-get install -y texlive texlive-xetex texlive-luatex texlive-lang-chinese \
    && apt-get clean

ENTRYPOINT ["lualatex"]
