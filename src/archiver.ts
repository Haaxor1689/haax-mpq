import fs from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';

const {
  SFileCloseArchive,
  SFileCreateArchive,
  SFileCreateFile,
  SFileFinishFile,
  SFileFlushArchive,
  SFileWriteFile,
  SFileCompactArchive,
  SFileHasFile,
  SFileRemoveFile
} = require('stormlib-node');
import {
  HASH_TABLE_SIZE,
  MPQ_CREATE,
  MPQ_FILE
} from 'stormlib-node/dist/enums';
import Logger, { getTimeElapsed } from './logger';
import Patchignore from './patchignore';

type ArchiveBuildOptions = {
  archivePath: string;
  directoryPath: string;
};

export const build = async (input: ArchiveBuildOptions) => {
  const startTime = new Date();

  Logger.log(`Building archive "${path.basename(input.archivePath)}"...`);

  const getAllFiles = async (filePath: string): Promise<string[]> => {
    const relativePath = filePath
      .slice(input.directoryPath.length + 1)
      .replaceAll('\\\\', '/');
    if (await Patchignore.matches(relativePath)) return [];

    if (!(await fs.lstat(filePath)).isDirectory()) return [filePath];
    return (
      await Promise.all(
        (
          await fs.readdir(filePath)
        ).map(f => getAllFiles(path.join(filePath, f)))
      )
    ).flat();
  };

  const exists = await existsSync(input.archivePath);

  const files = Object.fromEntries(
    (await getAllFiles(input.directoryPath)).map(v => [v, 'add'] as const)
  );

  if (exists) await fs.unlink(input.archivePath);

  const hMpq = SFileCreateArchive(
    input.archivePath,
    MPQ_CREATE.ARCHIVE_V1,
    Math.max(
      Math.min(Object.keys(files).length, HASH_TABLE_SIZE.MAX),
      HASH_TABLE_SIZE.MIN
    )
  );

  try {
    for (const [file, event] of Object.entries(files)) {
      const relativePath = file
        .slice(input.directoryPath.length + 1)
        .replaceAll('\\\\', '/');

      if (await Patchignore.matches(relativePath)) {
        // Logger.log(`Ignored "${relativePath}"`);
        continue;
      }

      // Logger.log(
      //   `${
      //     event === 'add'
      //       ? 'Adding'
      //       : event === 'change'
      //       ? 'Updating'
      //       : 'Removing'
      //   } "${relativePath}"`
      // );

      if (SFileHasFile(hMpq, relativePath)) SFileRemoveFile(hMpq, relativePath);

      if (event === 'add' || event === 'change') {
        const buffer = (await fs.readFile(file)).buffer as ArrayBuffer;
        const hFile = SFileCreateFile(
          hMpq,
          relativePath,
          0,
          buffer.byteLength,
          0,
          MPQ_FILE.COMPRESS
        );
        SFileWriteFile(hFile, buffer, 0);
        SFileFinishFile(hFile);
      }
    }

    SFileFlushArchive(hMpq);
    Logger.log('Compressing the archive...');
    SFileCompactArchive(hMpq);

    Logger.log(
      `Archive "${path.basename(input.archivePath)}" built in ${getTimeElapsed(
        startTime
      )}s`
    );
  } catch (e) {
    Logger.error(e);
  } finally {
    SFileCloseArchive(hMpq);
  }
};
