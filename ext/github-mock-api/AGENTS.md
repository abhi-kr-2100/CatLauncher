# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Project Overview

* This project implements a mock GitHub API server and library meant for integration testing.
* The project is being developed inside a Nix shell. See `flake.nix` for available programs.
* Run `cargo cov-term`, `cargo cov-lcov`, or `cargo cov-html` to run tests with coverage. Aim for 100% coverage.

## Coding Guidelines

* Avoid using `unwrap()`, `expect()`, and other methods that can cause panics. Use proper error handling with `Result`, `Option`, and the `?` operator instead.
