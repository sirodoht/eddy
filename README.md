# eddy

`.editorconfig` file generator.

[EditorConfig](http://editorconfig.org/) is a way to have consistent coding styles 
between different editors/IDEs/users. For example, EditorConfig defines the number 
of indent spaces (or tabs) with the `indent_size` property. You can see all properties
[here](http://editorconfig.org/#file-format-details).

The editor/IDE (see supported ones [here](http://editorconfig.org/#download)) 
reads a file named `.editorconfig` which exists in your project and enforces the
properties defined there. This project generates this file from the shell.

See more at [editorconfig.org](http://editorconfig.org/).

## Usage

The default behaviour is to generate a copy of [`default-editorconfig`](default-editorconfig)
in the current directory.

Currently, you can override the `indent_size` option and the directory of the
generated file. You can do that with two optional cli arguments.

```
eddy [indent_size] [path]
```

By default, `eddy` will use `2` for [indent_size] and the current directory as [path].

## Examples

* `eddy 4`: It will generate an `.editorconfig` file with indent size of 4.
* `eddy 8 src`: It will generate an `.editorconfig` file with indent size of 8,
    inside the `src` directory. NB: The directory must exist beforehand.

## Compile

`eddy` is written in [Rust](https://www.rust-lang.org/).

```
cargo build --release
```

## License

MIT
