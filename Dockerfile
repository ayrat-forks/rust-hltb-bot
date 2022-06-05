FROM public.ecr.aws/lambda/provided:latest
RUN yum install -y jq openssl-devel gcc zip
RUN curl https://sh.rustup.rs -sSf | sh /dev/stdin -y
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl
