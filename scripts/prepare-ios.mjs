import { copyFile, mkdir, stat } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const projectRoot = resolve(import.meta.dirname, '..');
const appleDir = join(projectRoot, 'src-tauri', 'gen', 'apple');
const privacySource = join(projectRoot, 'src-tauri', 'PrivacyInfo.xcprivacy');
const privacyDestination = join(appleDir, 'PrivacyInfo.xcprivacy');

try {
  await stat(appleDir);
} catch {
  throw new Error(
    'The generated Apple project is missing. Run "npm run ios:init" on macOS first.',
  );
}

await mkdir(appleDir, { recursive: true });
await copyFile(privacySource, privacyDestination);
console.log(`Prepared iOS privacy manifest: ${privacyDestination}`);
