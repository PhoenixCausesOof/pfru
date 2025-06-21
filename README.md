# pfru

## Description

`pfru` is an utility program to rename files using Lua string patterns.

## Usage

```
Usage: pfru [OPTIONS] --pat <PATTERN> --sub <SUBSTITUTION> [FILES]...

Arguments:
  [FILES]...  

Options:
      --cap <CAPTURES>      Assigns a name for each capture, in the order it was matched by the pattern
      --pat <PATTERN>       The pattern to match the filename with
      --sub <SUBSTITUTION>  The text that's going to replace the original filename. Enclose the captures you named with brackets in order to have them substitute into the text
      --diff                Shows all of the eventual changes should the replacement be done (without actually doing it)
  -h, --help                Print help
  ```

An example:

```bash
pfru ./* --pat='(.-)%.txt' --cap=filename --sub '{filename}.rs'
```
Changes the extension of all `.txt` files in the current directory to `rs`.

Essentially, the `--pat` flag will specify the pattern to be matched (with, or not, [captures](https://www.lua.org/pil/20.3.html)).   
The `--cap` flag will specify given names for each capture (in the order they are matched in), separated by commas.  
Finally, the `--sub` flag specifies the text that will **FULLY** substitute the original filename, allowing you to substitute-in the captures (by their name), a bit like Rust's `format!`.

Oh, and the files are given individually (i.e., it doesn't read directories). You could just use wildcards, though it may not be very efficient in the real world.

## Install

In the terminal, preferably in a dedicated folder.

```bash
git clone https://github.com/PhoenixCausesOof/pfru.git --depth=1
cargo install --path ./pfru
```
