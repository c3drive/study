#それ以外は基本はOS（例えばtouchコマンドはLinuxなのでLinuxOSを指定）
#LinuxOSが含まれていれば誰かが作ったイメージでもOK。
FROM node:25-slim

# RUNはレイヤーを作っていく
# RUNはできるだけ少なくすることがマスト
# 例えば、以下は&でつなげることで1行にできる
# RUN touch test
# RUN echo 'hello world' > test
# RUN touch test && echo 'hello world' > test

# しかしつなげすぎるとbuildの時間がかかるので
# 成系ではない開発段階では、ある程度キャッシュが使える細かい単位のレイヤーにする。
RUN apt-get update
RUN apt-get install -y \
	sudo \
	wget \
	vim

# 以下の書き方はsample_fileがルートフォルダに作成される。
# sample_dirも作成されている。
# 理由は1行ごとにレイヤーが作成されるのでtouch時にルートになってしまう
# RUN mkdir sample_dir
# RUN cd /sample_dir
# RUN touch sample_file

# 以下の書き方ならOK
# RUN mkdir sample_dir && \
#	cd /sample_dir && \
#	touch sample_file

# WORKDIRを使う（RUN mkdir sample_dirはなくても作ってくれる）
WORKDIR /app

# 環境変数
# ENV key1 value
# ENV key2=value
# ENV key3="v a l u e" key4=v\ a\ l\ u\ e
# ENV key5 v a l u e
# ENV PATH /opt/anaconda3/bin:$PATH

# こちらにあるファイルを持っていきたい場合（build_contextに置く必要あり）
# COPY something /new_dir/

# 圧縮ファイルを持っていきたい場合（build_contextに置く必要あり）
# ADD compressed.tar / 

# ENTRYPOINTはCMDと同じ形をとるがCMDと違いはrun時に上書きできない。
# また、ENTRYPOINTがある場合CMDはパラメータの形をとる（ENTRYPOINTの引数を取る）
# ・CMDにオプションを書くケース
# ENTRYPOINT ["ls"]
# CMD ["--help"]

# ・runをするときにオプションを書くケース
# ENTRYPOINT ["ls"]
# docker run <image> --help

# ・CMDもrunをするときもオプションを書くケース
# ENTRYPOINT ["ls"]
# CMD ["--help"]
# docker run <image> -la

# ・CMDに全部書くケース
# CMD ["ls", "--help", "-la"]

# CMDは最後に書く
# CMD ["env"]
