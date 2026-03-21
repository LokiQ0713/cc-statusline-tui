#!/usr/bin/env node
const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');

const REPO = 'LokiQ0713/claude-statusline-config';
const BIN_NAME = 'claude-statusline-config';
const VERSION = require('./package.json').version;

const TARGETS = {
  'darwin-arm64': 'aarch64-apple-darwin',
  'darwin-x64': 'x86_64-apple-darwin',
  'linux-x64': 'x86_64-unknown-linux-musl',
  'linux-arm64': 'aarch64-unknown-linux-musl',
};

function main() {
  const platform = process.platform;
  const arch = process.arch;
  const key = `${platform}-${arch}`;
  const target = TARGETS[key];

  if (!target) {
    console.log(`[claude-statusline-config] Unsupported platform: ${platform}-${arch}`);
    console.log('Supported platforms: darwin-arm64, darwin-x64, linux-arm64, linux-x64');
    console.log('You can build from source: https://github.com/' + REPO);
    process.exit(0); // exit gracefully, don't fail install
  }

  const asset = `claude-statusline-config-${target}.tar.gz`;
  const url = `https://github.com/${REPO}/releases/download/v${VERSION}/${asset}`;

  const binDir = path.join(os.homedir(), '.claude', 'statusline', 'bin');
  const binPath = path.join(binDir, BIN_NAME);
  const tmpDir = path.join(os.tmpdir(), `claude-statusline-config-${Date.now()}`);
  const tarPath = path.join(tmpDir, asset);

  console.log(`[claude-statusline-config] Downloading binary for ${platform}-${arch}...`);
  console.log(`[claude-statusline-config] URL: ${url}`);

  // Create directories
  fs.mkdirSync(tmpDir, { recursive: true });
  fs.mkdirSync(binDir, { recursive: true });

  download(url, tarPath, 0)
    .then(() => {
      // Extract tar.gz
      console.log('[claude-statusline-config] Extracting...');
      execSync(`tar xzf "${tarPath}" -C "${tmpDir}"`);

      // Find the binary in extracted files
      const extractedBin = findBinary(tmpDir);
      if (!extractedBin) {
        throw new Error('Binary not found in archive');
      }

      // Copy binary to destination
      fs.copyFileSync(extractedBin, binPath);
      fs.chmodSync(binPath, 0o755);

      console.log(`[claude-statusline-config] Installed to ${binPath}`);

      // Cleanup
      fs.rmSync(tmpDir, { recursive: true, force: true });
    })
    .catch((err) => {
      console.error('[claude-statusline-config] Failed to install binary:', err.message);
      console.error('[claude-statusline-config] Tip: Copy this error to AI for analysis');
      console.error('[claude-statusline-config] See https://github.com/' + REPO + '#troubleshooting');
      // Cleanup on error
      try {
        fs.rmSync(tmpDir, { recursive: true, force: true });
      } catch (_) {
        // ignore cleanup errors
      }
      process.exit(1);
    });
}

/**
 * Find the binary in the extracted directory.
 * The binary may be at the top level or inside a subdirectory.
 */
function findBinary(dir) {
  // Check top level first
  const direct = path.join(dir, BIN_NAME);
  if (fs.existsSync(direct)) return direct;

  // Check subdirectories (e.g., tar may extract into a folder)
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    if (entry.isDirectory()) {
      const nested = path.join(dir, entry.name, BIN_NAME);
      if (fs.existsSync(nested)) return nested;
    }
  }

  return null;
}

/**
 * Download a file from a URL, following redirects (up to 5).
 * Uses Node.js built-in https module.
 */
function download(url, dest, redirectCount) {
  if (redirectCount > 5) {
    return Promise.reject(new Error('Too many redirects'));
  }

  return new Promise((resolve, reject) => {
    const proto = url.startsWith('https') ? https : require('http');
    proto
      .get(url, { headers: { 'User-Agent': 'claude-statusline-config-npm' } }, (res) => {
        // Handle redirects (301, 302, 303, 307, 308)
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          res.resume(); // consume response to free memory
          return download(res.headers.location, dest, redirectCount + 1).then(resolve, reject);
        }

        if (res.statusCode !== 200) {
          res.resume();
          return reject(new Error(`Download failed: HTTP ${res.statusCode}`));
        }

        const file = fs.createWriteStream(dest);
        res.pipe(file);
        file.on('finish', () => {
          file.close(resolve);
        });
        file.on('error', (err) => {
          fs.unlink(dest, () => {}); // cleanup partial file
          reject(err);
        });
      })
      .on('error', reject);
  });
}

main();
