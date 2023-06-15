# Scenic

[![crate::scenic](
https://img.shields.io/crates/v/scenic?label=crates&style=flat)](
https://crates.io/crates/scenic)

Presentation tool that utilizes LaTex Beamer package.


## Install

```zsh
% cargo install scenic
```


## Usage

```zsh
# write stdout into a file for now :)
% scenic example/slide.latex > dst/result.pdf
```

```zsh
# eg. zathura
% zathura --mode=presentation dst/result.pdf
```


## Build

```zsh
% make build
```


## Test

```zsh
% make test
```


## Requirements

* [Tectonic](https://github.com/tectonic-typesetting/tectonic)
* [DejaVu](https://dejavu-fonts.github.io)
* [TeX Gyre Pagella](
https://www.gust.org.pl/projecets/e-foundry/tex-gyre/pagella)


## License

`GPL-3.0-or-later`

```txt
Scenic
Copyright (C) 2023 Yasuhiro Яша Asaka

This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `show c' for details.
```
