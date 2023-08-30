FROM ruby:3.2.2-slim-bookworm
COPY hello.rb .
COPY Gemfile .
RUN bundle install
CMD [ "ruby", "hello.rb" ]