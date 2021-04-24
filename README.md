# smash-fnv

A Rust library/CLI for working with sound_volume_fighter_num_table.fnv files in Smash Ultimate 

(fnv not to be confused with Fowler–Noll–Vo, which was made use of in previous entries of the game)

### Example CLI usage

```
smash-fnv 0.8.0
A tool for converting between Smash Ultimate 'sound volume fighter num table' files and yaml

USAGE:
    fnv <in-file> <out-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <in-file>
    <out-file>
```

Convert to yaml:

```
fnv sound_volume_fighter_num_table.fnv fnv.yaml
```

Convert back:

```
fnv fnv.yaml sound_volume_fighter_num_table.fnv
```


### Example Library Usage

```rust
use fnv::FnvFile;

let mut file = FnvFile::open("sound_volume_fighter_num_table.fnv")?;

for entry in file.entries() {
    println!("{}: {:?}", entry.id, entry.vols);
}

file.save("sound_volume_fighter_num_table.fnv")?;
```
