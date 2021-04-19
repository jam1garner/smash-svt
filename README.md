# smash-csb

A Rust library/cli for working with `commonsoundtable.csb` files from Smash Ultimate.

### Example CLI usage

```
smash-csb 0.8.0
A tool for converting between Smash Ultimate common sound table files and yaml

USAGE:
    csb [OPTIONS] <in-file> <out-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --labels <labels>    newline-separated hash labels to use

ARGS:
    <in-file>
    <out-file>
```

Convert to yaml:

```
csb commonsoundtable.csb csb.yaml
```

Convert back:

```
csb csb.yaml commonsoundtable.csb
```

By default, the CLI uses `ParamLabels.csv` from the current directory, however other labels can be passed using `--labels` or `-l`.

### Example Library Usage

```rust
use csb::CsbFile;

let mut file = CsbFile::open("commonsoundtable.csb")?;

for entry in file.entries() {
    println!("name: {}", entry.character_name);
}

file.save("commonsoundtable.csb")?;
```
