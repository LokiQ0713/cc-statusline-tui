#!/usr/bin/env node
const { execFileSync } = require('child_process');
const { join } = require('path');
const { homedir } = require('os');
const { existsSync } = require('fs');

const bin = join(homedir(), '.claude', 'statusline', 'bin', 'cc-statusline');

if (!existsSync(bin)) {
  console.error('cc-statusline binary not found at', bin);
  console.error('Try reinstalling: npm install -g cc-statusline-tui');
  process.exit(1);
}

try {
  execFileSync(bin, process.argv.slice(2), { stdio: 'inherit' });
} catch (e) {
  if (e.status) process.exit(e.status);
  process.exit(1);
}
