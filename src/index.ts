#!/usr/bin/env node

import path from 'node:path';
import { build } from './archiver';

const args = process.argv.slice(2);
if (args.length === 0 || args.length > 2) {
  console.log('Incorrect number of arguments passed');
  process.exit(1);
}

const directoryPath = path.normalize(args[0]);
const archivePath =
  args.length === 1 ? `${directoryPath}.mpq` : path.normalize(args[1]);

build({ directoryPath, archivePath });
