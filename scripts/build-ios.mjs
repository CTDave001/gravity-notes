import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';

const viteBin = resolve(import.meta.dirname, '..', 'node_modules', 'vite', 'bin', 'vite.js');
const result = spawnSync(process.execPath, [viteBin, 'build'], {
  stdio: 'inherit',
  env: {
    ...process.env,
    VITE_GRAVITY_PLATFORM: 'ios',
  },
});

if (result.error) {
  throw result.error;
}

process.exit(result.status ?? 1);
