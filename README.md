# uil-checker

A CLI checking uil files for parsing errors. The goal is to use this in your CI environment.

## Example

```bash
cargo build
./target/debug/uil-checker --markup ./assets/markup/test.xml \
 --style ./assets/style/test.style --deps ./assets/deps/test.deps
```

## Usage

```
Usage: uil-checker [--markup <markup_file>]
                   ([--deps <deps_file>] |
                    [--style <style_file> --deps <deps_file>])
       uil-checker --help

Options:
  -h, --help                   Show this message.
  -m, --markup <markup_file>   Check the given markup file.
  -s, --style <style_file>     Check the given style file.
  -d, --deps <deps_file>       Check the given deps file.
```

Note that in the current configuration if you specify a `style` file,
you have to specify a `deps` file too. Even if your style file does
not have any dependency.

## Todos

* [ ] Add support for folders.
* [ ] Allow style file only.
