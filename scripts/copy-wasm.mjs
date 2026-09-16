import { copyFileSync, mkdirSync, rmSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const srcDir = join(root, 'rust-pi', 'pkg');
const outDir = join(root, 'public', 'wasm');

mkdirSync(outDir, { recursive: true });
rmSync(outDir, { recursive: true, force: true });
mkdirSync(outDir, { recursive: true });

for (const file of ['rust_pi.js', 'rust_pi_bg.wasm']) {
  copyFileSync(join(srcDir, file), join(outDir, file));
  console.log(`copied ${file}`);
}

console.log('done');