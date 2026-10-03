#!/usr/bin/env node
/**
 * Downloads the pre-built agentml binary for the current platform.
 * Falls back to cargo install if no pre-built binary is available.
 */
const { execSync } = require('child_process');
const fs = require('fs');
const https = require('https');
const os = require('os');
const path = require('path');

const VERSION = require('../package.json').version;
const REPO = 'Nom-nom-hub/agentml';

function getPlatform() {
  const platform = os.platform();
  const arch = os.arch();
  const archMap = { x64: 'x86_64', arm64: 'aarch64' };
  const platformMap = { linux: 'linux', darwin: 'macos', win32: 'windows' };
  const mappedArch = archMap[arch];
  const mappedPlatform = platformMap[platform];
  if (!mappedArch || !mappedPlatform) return null;
  return `${mappedArch}-${mappedPlatform}`;
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, { headers: { 'User-Agent': 'agentml-npm-installer' } }, (res) => {
      if (res.statusCode === 302 || res.statusCode === 301) {
        file.close();
        return download(res.headers.location, dest).then(resolve, reject);
      }
      if (res.statusCode !== 200) {
        file.close();
        fs.unlinkSync(dest);
        return reject(new Error(`Download failed: HTTP ${res.statusCode}`));
      }
      res.pipe(file);
      file.on('finish', () => { file.close(); resolve(); });
    }).on('error', (err) => { fs.unlinkSync(dest); reject(err); });
  });
}

async function main() {
  const binDir = __dirname;
  const binName = os.platform() === 'win32' ? 'agentml.exe' : 'agentml';
  const binPath = path.join(binDir, binName);

  if (fs.existsSync(binPath)) {
    console.log('agentml binary already installed.');
    return;
  }

  const platform = getPlatform();
  if (!platform) {
    console.log(`Unsupported platform: ${os.platform()}-${os.arch()}. Trying cargo install...`);
    execSync('cargo install agentml', { stdio: 'inherit' });
    return;
  }

  const url = `https://github.com/${REPO}/releases/download/v${VERSION}/agentml-${platform}.tar.gz`;
  const tmpFile = path.join(os.tmpdir(), `agentml-${VERSION}.tar.gz`);

  console.log(`Downloading agentml v${VERSION} for ${platform}...`);
  try {
    await download(url, tmpFile);
    const extractDir = path.join(os.tmpdir(), `agentml-extract-${Date.now()}`);
    fs.mkdirSync(extractDir, { recursive: true });
    execSync(`tar -xzf "${tmpFile}" -C "${extractDir}"`);
    // Find the binary in the extracted files
    const files = fs.readdirSync(extractDir, { recursive: true });
    const binFile = files.find(f => path.basename(f) === binName);
    if (!binFile) throw new Error('Binary not found in archive');
    fs.copyFileSync(path.join(extractDir, binFile), binPath);
    fs.chmodSync(binPath, 0o755);
    fs.unlinkSync(tmpFile);
    fs.rmSync(extractDir, { recursive: true, force: true });
    console.log(`agentml v${VERSION} installed successfully!`);
  } catch (err) {
    console.error(`Pre-built binary download failed: ${err.message}`);
    console.log('Falling back to cargo install (requires Rust)...');
    execSync('cargo install agentml', { stdio: 'inherit' });
  }
}

main().catch(err => { console.error(err); process.exit(1); });
