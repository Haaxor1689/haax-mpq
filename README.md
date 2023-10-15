# haax-mpq

CLI utility to build MPQ archive from a folder.

## Usage

```
haax-mpq <directoryPath> <archivePath?>
```

- `directoryPath`: Full path to the folder
- `archivePath`: Full path to the output MPQ file. Default: `<directoryPath>.mpq`

## Windows executable

You can also use this CLI as a standalone Windows executable. Run these scripts after cloning to generate it:

```
pnpm i
pnpm run bundle
```

This will create a `haax-mpq.exe` in the root of this repository.
