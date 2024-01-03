# RAVN Mobile CI/CD Installer

## Running

Running the following commands will download a copy of the installer, ready to be run as `installer`.

### Download 

#### macOS

```shell
curl -sL https://github.com/ravnhq/mobile-cicd-installer/releases/download/0.1.0/installer-macos -o installer \
  && chmod +x installer
```

#### Linux

```shell
curl -sL https://github.com/ravnhq/mobile-cicd-installer/releases/download/0.1.0/installer-linux -o installer \
  && chmod +x installer
```

### Executing

Once downloaded just run the following command to start in interactive mode:

```shell
./installer -i
```

Or explore more options by running:

```shell
./installer -h
```

## Compiling

All instructions to build are packed in the `justfile` file, if you already have `just` installed you can use any of the
following recipes to produce a binary for your platform:

- `release-macos`: For a Universal macOS binary (ARM and x86)
    - `release-macos-arm`: For an ARM macOS binary
    - `release-macos-x86`: For an x86 macOS binary
- `release-linux`: For a statically linked Linux binary

## About

### Why Rust?

Rust is a good language to build CLI tools as it doesn't require the user to install any runtime (e.g. Node or Python),
streamlining the setup process by only requiring a single download that works out-of-the-box.
