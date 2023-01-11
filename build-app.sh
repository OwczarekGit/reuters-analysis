#!/bin/bash

CLASSIFIER_DIR="classifier"
FRONTEND_DIR="frontend"
EXECUTOR_DIR="executor"
ROOT=$(pwd)

print_red(){
  echo -e "\033[1;91m$1\e[0m"
}
print_yellow(){
  echo -e "\033[1;93m$1\e[0m"
}
print_green(){
  echo -e "\033[1;92m$1\e[0m"
}
print_blue(){
  echo -e "\033[1;94m$1\e[0m"
}

check_for_cargo(){
  cargo -V > /dev/null || (print_red "Rust does not seem to be installed, install it before proceeding." && exit)
  print_green "Cargo ok."
}

check_for_docker(){
  docker-compose -v > /dev/null || (print_red "Docker does not seem to be installed, install it before proceeding." && exit)
  print_green "Docker ok."
}

check_for_mvn(){
  mvn -v > /dev/null || (print_red "Maven does not seem to be installed, install it before proceeding." && exit)
  print_green "MVN ok."
}

check_for_npm(){
  npm -v > /dev/null || (print_red "NPM does not seem to be installed, install it before proceeding." && exit)
  print_green "NPM ok."
}

generate_config(){
  sed --version > /dev/null || (print_red "Sed not available, cannot autogenerate application.yml config." && return)

  cd "$EXECUTOR_DIR/src/main/resources"

  print_blue "Generating config..."
  CPUS=$(nproc)
  WD_PATH="$ROOT/$CLASSIFIER_DIR"
  CLASSIFIER="$ROOT/$CLASSIFIER_DIR/target/release/analiza-danych"
  DATA="$ROOT/$CLASSIFIER_DIR/reuters"

  sed -i "s|<CPUS>|$CPUS|" application.yml
  sed -i "s|<WD_PATH>|$WD_PATH|" application.yml
  sed -i "s|<CLASSIFIER>|$CLASSIFIER|" application.yml
  sed -i "s|<DATA>|$DATA|" application.yml

  cd $ROOT
}

start_application(){
  print_blue "Building frontend..."
  cd $FRONTEND_DIR && npm install && npm start &
  cd $ROOT

  print_blue "Building classifier..."
  cd $CLASSIFIER_DIR && cargo build --release && cd $ROOT

  print_blue "Starting docker..."
  cd docker && docker-compose up -d && cd $ROOT

  print_blue "Starting backend..."
  cd $EXECUTOR_DIR && mvn package && mvn spring-boot:run
}

check_for_cargo
check_for_docker
check_for_mvn
check_for_npm

generate_config

start_application


