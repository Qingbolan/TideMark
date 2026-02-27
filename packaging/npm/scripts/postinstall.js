#!/usr/bin/env node
/*
TideMark
========

File: packaging/npm/scripts/postinstall.js
Description: npm postinstall bootstrap for prefetching TideMark native binaries.

Responsibility:
- Warm local cache to provide first-command latency reduction for users.

Architectural Position:
- Package installation lifecycle hook in npm distribution flow.

Author: Silan.Hu
Email: silan.hu@u.nus.edu
Copyright (c) 2026-2027 easynet. All rights reserved.
*/

const { prefetchBinaries } = require("../lib/runner");

prefetchBinaries()
  .then(() => {
    process.stdout.write("tidemark: native binaries ready\n");
  })
  .catch((error) => {
    process.stderr.write(`tidemark: prefetch skipped (${error.message})\n`);
  });
