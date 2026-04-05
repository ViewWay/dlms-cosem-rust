# Fuzz Testing

This directory contains fuzz targets for the dlms-cosem-rust workspace.

## Prerequisites

Install `cargo-fuzz`:

```bash
cargo install cargo-fuzz
```

## Running Fuzz Tests

Run fuzz targets:

```bash
# Run HDLC parser fuzzing
cargo fuzz run hdlc_parser
```

## Fuzz Targets

| Target | Description |
|--------|-------------|
| `hdlc_parser` | Fuzzes `HdlcParser::feed()` with random byte sequences. Should never panic. |

## Notes

- Fuzzing is computationally intensive. Consider running with a time limit or on a server.
- Fuzz corpus will be generated in `fuzz/fuzz_inputs/`.
- Crashes (if any) will be saved to `fuzz/artifacts/`.

## CI Integration

Add fuzz tests to CI to catch regressions early:

```yaml
- name: Fuzz tests
  run: cargo fuzz run hdlc_parser -- -max_total_time=60
```
