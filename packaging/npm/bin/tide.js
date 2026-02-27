#!/usr/bin/env node
/*
TideMark
========

File: packaging/npm/bin/tide.js
Description: npm command entrypoint for forwarding to the native TideMark `tide` binary.

Responsibility:
- Delegate CLI invocation to resolved native executable.

Architectural Position:
- Node command shim for package-manager installations.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
*/

const { runBinary } = require("../lib/runner");

runBinary("tide", process.argv.slice(2)).catch((error) => {
  console.error(`error: ${error.message}`);
  process.exitCode = 1;
});
