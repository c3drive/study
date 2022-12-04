FROM ruby:3.0

# throw errors if Gemfile has been modified since Gemfile.lock
# RUN bundle config --global frozen 1

WORKDIR /usr/src/app

# copy Gemfile Gemfile.lock
COPY Gemfile Gemfile.lock ./

# without production group in Gemfile
RUN bundle config set --local without 'production'
# install the Gems written in the Gemfile 
# /usr/local/bundle
RUN bundle install

COPY . .

# CMD ["./your-daemon-or-script.rb"]

# javaruntimeがないとrails serverコマンドが失敗する
RUN apt-get update
RUN apt-get install nodejs -y