import anymatch from 'anymatch';

const patchignore = `
# ignore git related stuff
.git/**
.github/**
.gitignore

# ignore all file formats that have nothing to do in patch
**/*.json
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

.patchignore`
  .split('\n')
  .map(v => v.trim())
  .filter(v => !!v && !v.startsWith('#'));

const Patchignore = {
  matches: (relativePath: string) => anymatch(patchignore, relativePath)
};

export default Patchignore;
