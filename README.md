# svt_lib

A Rust library for reading and writing `sound_volume_table.svt` files from Super Smash Bros. for Nintendo 3DS and Wii U and Super Smash Bros. Ultimate.

## svt_yaml

A command-line program for creating and editing `sound_volume_table.svt` files using YAML. Drag and drop a `sound_volume_table.svt` file onto the executable to create a YAML file. Drag and drop a properly structured YAML file onto the executable to create a `sound_volume_table.svt` file. YAML files are text files, so they can be viewed and edited in any text editor.

Sample output from a `sound_volume_table.svt` file:

```yaml
entries:
- id: 0
  knobs:
  - 0.0
  - -1.0
  - 0.0
  - -1.5
- id: 1
  knobs:
  - 0.2
  - -0.8
  - 0.0
  - -0.7
```

### Usage

The latest prebuilt binary for Windows is available in [Releases](https://github.com/jam1garner/smash-svt/releases/latest).

`svt_yaml <input> [output]`<br>
`svt_yaml sound_volume_table.svt sound_volume_table.yaml`<br>
`svt_yaml sound_volume_table.yaml sound_volume_table.svt`<br>
