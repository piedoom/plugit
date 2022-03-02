# plugit

Plugit is a plugin bundler ~~stolen~~ largely borrowed from the code in [`nih-plug`'s `xtask` crate](https://github.com/robbert-vdh/nih-plug/tree/master/xtask/src).

Plugit is separated into a library and a command line utility powered by clap.

## Installing

```
cargo install --git https://github.com/piedoom/plugit
```

## Usage

```
ARGS:
    <INPUT_PATH>    Optional absolute path of an input library to be bundled      
OPTIONS:
    -d, --debug              Optionally specify that the target compiled in debug mode    
    -f, --format <FORMAT>    Optional output format override    
    -h, --help               Print help information    
    -t, --target <TARGET>    Optional bundle target override
```

## Arguments

### Positional

#### Input path

Input path to plugin library to be bundled. If this is blank, `plugit` will attempt to search for a cargo project.

#### Target

Optional target override for the bundler. Valid values can be viewed in the `target` `FromStr` module implementation.

#### Format

Optional format override for the bundler. Only supports VST3, so this doesn't do anything currently.

#### Debug

If relying on using a cargo project directory instead of specifying a file, this flag indicates the plugin was built in debug mode. Otherwise, `plugit` will always look in the release folder.

## Tested

Everything but windows x64 is untested!