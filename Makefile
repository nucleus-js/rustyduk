build:
	cargo build

test: test-file test-dir test-zip test-app

test-file: target/debug/nucleus test.js test-error.js
	$< --no-bundle test.js -- 0 5 10

test-dir: target/debug/nucleus
	$< test-app -- 1 2 3

test-zip: target/debug/nucleus target/test-app.zip
	$^ -- 4 5 6

test-app: target/app
	$< 7 8 9

test-app-tiny: target/app-tiny
	$< 10 11 12

target/app: target/debug/nucleus test-app/* test-app/deps/*
	$< test-app -o $@

target/app-tiny: target/debug/nucleus test-app/* test-app/deps/*
	$< test-app -l -o $@

target/test-app.zip: target/debug/nucleus test-app/* test-app/deps/*
	$< test-app -z -o $@

target/debug/nucleus: src/**
	cargo build
