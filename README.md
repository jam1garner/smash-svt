# smash-svt

A Rust library/CLI for working with sound_volume_table.svt files in Smash Ultimate 

### Example CLI usage

```
smash-svt 0.8.0
A tool for converting between Smash Ultimate 'sound volume table' files and yaml

USAGE:
    svt <in-file> <out-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <in-file>
    <out-file>
```

Convert to yaml:

```
svt sound_volume_table.svt svt.yaml
```

Convert back:

```
svt svt.yaml sound_volume_table.svt
```

### CLI Install (Linux/Mac OS/etc)

```
cargo install --git https://github.com/jam1garner/smash-svt --features=cli
```

### Example Library Usage

```rust
use svt::SvtFile;

let mut file = SvtFile::open("sound_volume_table.svt")?;

for entry in file.entries() {
    println!("{}: {:?}", entry.id, entry.knobs);
}

file.save("sound_volume_table.svt")?;
```
