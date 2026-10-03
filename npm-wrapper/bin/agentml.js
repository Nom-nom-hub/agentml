#!/usr/bin/env node
/**
 * Proxy to the agentml binary. Runs the downloaded binary,
 * or falls back to a cargo-installed agentml on PATH.
 */
const { spawnSync } = require('child_process');
const fs = require('fs');
const os = require('os');
const path = require('path');

const binName = os.platform() === 'win32' ? 'agentml.exe' : 'agentml';
const localBin = path.join(__dirname, binName);

let binPath = localBin;
if (!fs.existsSync(localBin)) {
  // Fall back to agentml on PATH (e.g. cargo install)
  binPath = 'agentml';
}

const result = spawnSync(binPath, process.argv.slice(2), { stdio: 'inherit' });
process.exit(result.status ?? 1);
