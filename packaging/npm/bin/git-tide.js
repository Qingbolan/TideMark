#!/usr/bin/env node
/*
TideMark
========

File: packaging/npm/bin/git-tide.js
Description: npm command entrypoint for forwarding to the native TideMark `git-tide` binary.

Responsibility:
- Delegate Git plugin CLI invocation to resolved native executable.

Architectural Position:
- Node command shim for Git subcommand plugin usage.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
*/

const { runBinary } = require("../lib/runner");

runBinary("git-tide", process.argv.slice(2)).catch((error) => {
  console.error(`error: ${error.message}`);
  process.exitCode = 1;
});
