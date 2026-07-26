import { copyFile, mkdir, readdir, stat } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const projectRoot = resolve(import.meta.dirname, '..');
const appleDir = join(projectRoot, 'src-tauri', 'gen', 'apple');
const privacySource = join(projectRoot, 'src-tauri', 'PrivacyInfo.xcprivacy');
const privacyDestination = join(appleDir, 'PrivacyInfo.xcprivacy');
const iconSourceDir = join(projectRoot, 'src-tauri', 'icons', 'ios');
const iconDestinationDir = join(
  appleDir,
  'Assets.xcassets',
  'AppIcon.appiconset',
);

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

try {
  await stat(iconDestinationDir);
} catch {
  throw new Error(
    `The generated iOS app icon catalog is missing: ${iconDestinationDir}`,
  );
}

const iconFiles = (await readdir(iconSourceDir))
  .filter((fileName) => fileName.toLowerCase().endsWith('.png'));

await Promise.all(
  iconFiles.map((fileName) =>
    copyFile(
      join(iconSourceDir, fileName),
      join(iconDestinationDir, fileName),
    ),
  ),
);

console.log(
  `Prepared ${iconFiles.length} branded iOS app icons: ${iconDestinationDir}`,
);
