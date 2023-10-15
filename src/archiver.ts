import fs from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';

import {
  SFileAddFileEx,
  SFileCloseArchive,
  SFileCreateArchive,
  SFileCompactArchive
} from 'stormlib-node';
import {
  HASH_TABLE_SIZE,
  MPQ_COMPRESSION,
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

  const matches = await Patchignore(input.directoryPath);

  Logger.log(`Building archive "${path.basename(input.archivePath)}"...`);

  const getAllFiles = async (filePath: string): Promise<string[]> => {
    const relativePath = filePath
      .slice(input.directoryPath.length + 1)
      .replaceAll('\\\\', '/');
    if (await matches(relativePath)) return [];

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
  if (exists) await fs.unlink(input.archivePath);

  const files = await getAllFiles(input.directoryPath);

  const hMpq = SFileCreateArchive(
    input.archivePath,
    MPQ_CREATE.ARCHIVE_V2,
    Math.max(Math.min(files.length, HASH_TABLE_SIZE.MAX), HASH_TABLE_SIZE.MIN)
  );

  try {
    for (const file of files) {
      const fullPath = file.replaceAll('\\\\', '/');
      const relativePath = file
        .slice(input.directoryPath.length + 1)
        .replaceAll('\\\\', '/');

      if (await matches(relativePath)) {
        Logger.log(`Ignored "${relativePath}"`);
        continue;
      }

      SFileAddFileEx(
        hMpq,
        fullPath,
        relativePath,
        MPQ_FILE.COMPRESS,
        MPQ_COMPRESSION.ZLIB,
        MPQ_COMPRESSION.NEXT_SAME
      );
    }

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
