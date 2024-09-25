# LoongArch64-MikuOS

LoongArch64 OS written in Rust

## Usage

### Setup Rust Develop Environment

```bash
make setup
```

> qemu-system-loongarch64 8.X required

### Build

```bash
make build
```

### Run

```bash
make run LOG=<log_level>
# or simply make
```

> log_level option: ERROR, WARN, INFO, DEBUG, TRACE

### Debug

```bash
make debug
```

Then at the same dir:

```bash
make connect
```

## License

MIT
