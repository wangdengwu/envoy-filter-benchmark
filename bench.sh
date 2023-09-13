#!/bin/bash

set -o errexit
set +x

do_benchmark() {
  docker compose down
  docker compose up -d --build
  sleep 35
  docker compose down
  # shellcheck disable=SC2035
  mv *.json ../"$1"
}

bench_model() {
  echo "====== $1 ======"

  cd "$1"

  # shellcheck disable=SC2035
  rm -rf *.json

  echo "====== start benchmark ======"

  do_benchmark "$2"

  echo "====== end benchmark ======"

  cd ..
}

do_process_report(){
  cd "$1"
  docker compose down
  docker compose up --build
  docker compose down
  cd ..
}

rm -rf report-passthrough/*.json
rm -rf report-basic-auth/*.json

report_passthrough_dir="report-passthrough"

report_basic_auth_dir="report-basic-auth"

report_into_readme="report-into-readme"

echo "****** start passthrough benchmark ******"

bench_model envoy-base $report_passthrough_dir

bench_model golang-passthrough $report_passthrough_dir

bench_model lua-passthrough $report_passthrough_dir

bench_model wasm-rust-passthrough $report_passthrough_dir

bench_model wasm-tinygo-passthrough $report_passthrough_dir

echo "****** end passthrough benchmark ******"

echo "****** start basic auth benchmark ******"

bench_model golang-basic-auth $report_basic_auth_dir

bench_model lua-basic-auth $report_basic_auth_dir

bench_model wasm-rust-basic-auth $report_basic_auth_dir

bench_model wasm-tinygo-basic-auth $report_basic_auth_dir

echo "****** end basic auth benchmark ******"

echo "****** start process report ******"

do_process_report $report_into_readme

echo "****** end process report ******"

echo "****** now you can read README.md to see the result ******"
