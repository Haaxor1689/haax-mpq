import fs from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';

import anymatch from 'anymatch';

const defaultPatchignore = `
# ignore git related stuff
.git/**
.github/**
.gitignore

# ignore all file formats that have nothing to do in patch
**/*.json
**/*.yml
**/*.yaml
**/*.exe
**/*.dll
**/*.db
**/*.csv
**/*.png
**/*.psd
**/*.txt
**/*.md
**/*.sql

.patchignore`;

const Patchignore = async (directoryPath: string) => {
  const patchignorePath = path.join(directoryPath, '.patchignore');
  const patchignore = (
    existsSync(patchignorePath)
      ? await fs.readFile(patchignorePath, { encoding: 'utf-8' })
      : defaultPatchignore
  )
    .split('\n')
    .map(v => v.trim())
    .filter(v => !!v && !v.startsWith('#'));
  return (relativePath: string) => anymatch(patchignore, relativePath);
};

export default Patchignore;
