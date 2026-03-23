#!/usr/bin/env node
const { execFileSync } = require('child_process');
const path = require('path');

const PLATFORMS = {
  'darwin-arm64': 'cc-statusline-tui-darwin-arm64',
  'darwin-x64': 'cc-statusline-tui-darwin-x64',
  'linux-x64': 'cc-statusline-tui-linux-x64',
  'linux-arm64': 'cc-statusline-tui-linux-arm64',
};

const key = `${process.platform}-${process.arch}`;
const pkg = PLATFORMS[key];

if (!pkg) {
  console.error(`[cc-statusline] Unsupported platform: ${key}`);
  console.error('Supported: darwin-arm64, darwin-x64, linux-x64, linux-arm64');
  console.error('Build from source: cargo install cc-statusline-tui');
  process.exit(1);
}

let binPath;
try {
  // Resolve binary from the platform-specific npm package
  const pkgDir = path.dirname(require.resolve(`${pkg}/package.json`));
  binPath = path.join(pkgDir, 'bin', 'cc-statusline');
} catch {
  console.error(`[cc-statusline] Platform package "${pkg}" not installed.`);
  console.error('Try: npx cc-statusline-tui@latest');
  process.exit(1);
}

try {
  execFileSync(binPath, process.argv.slice(2), { stdio: 'inherit' });
} catch (e) {
  if (e.status) process.exit(e.status);
  process.exit(1);
}
