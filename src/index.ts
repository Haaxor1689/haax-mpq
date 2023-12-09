#!/usr/bin/env node

import path from 'node:path';
import { build } from './archiver';
import { existsSync, lstatSync } from 'node:fs';

const args = process.argv.slice(2);
if (args.length === 0 || args.length > 2) {
  console.log('Incorrect number of arguments passed');
  process.exit(1);
}

let directoryPath = path.normalize(args[0]);

// Handle trailing slash
if (directoryPath.endsWith('\\') || directoryPath.endsWith('/')) {
  directoryPath = directoryPath.slice(0, -1);
}

// Check if directory exists
if (existsSync(directoryPath) && !lstatSync(directoryPath).isDirectory()) {
  console.log('First argument must be a directory');
  process.exit(1);
}

let archivePath = args.length === 1 ? directoryPath : path.normalize(args[1]);

// Handle extension
if (!archivePath.toLocaleLowerCase().endsWith('.mpq')) {
  archivePath = `${directoryPath}.mpq`;
}

build({ directoryPath, archivePath });
