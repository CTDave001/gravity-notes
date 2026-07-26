import { writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const bundleIdentifier = process.env.IOS_BUNDLE_ID?.trim();
const developmentTeam = process.env.APPLE_DEVELOPMENT_TEAM?.trim();

if (!bundleIdentifier) {
  throw new Error('IOS_BUNDLE_ID is required.');
}

if (
  !/^[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)+$/.test(bundleIdentifier) ||
  bundleIdentifier.endsWith('.app')
) {
  throw new Error(
    'IOS_BUNDLE_ID must be a reverse-DNS identifier and must not end in ".app".',
  );
}

if (!developmentTeam || !/^[A-Z0-9]{10}$/.test(developmentTeam)) {
  throw new Error(
    'APPLE_DEVELOPMENT_TEAM must be the 10-character Apple Developer Team ID.',
  );
}

const outputPath = resolve(
  import.meta.dirname,
  '..',
  'src-tauri',
  'tauri.ios.ci.conf.json',
);
const config = {
  $schema: '../node_modules/@tauri-apps/cli/config.schema.json',
  identifier: bundleIdentifier,
  bundle: {
    iOS: {
      developmentTeam,
    },
  },
};

await writeFile(outputPath, `${JSON.stringify(config, null, 2)}\n`, {
  encoding: 'utf8',
  mode: 0o600,
});

console.log(
  `Prepared iOS CI configuration for ${bundleIdentifier} and team ${developmentTeam}.`,
);
