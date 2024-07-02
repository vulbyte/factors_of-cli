# factors_of-cli

just a little command line tool to save having to manually have to factor numbers as it can be a pain in the ass

## 💻 install script:

> note: this is only a valid install script for unix systems with bash support

requirements:

- [git](https://git-scm.com/downloads)
- [cargo](https://www.rust-lang.org/tools/install)

that should be it. once done simply run:

```
curl -SL https://raw.githubusercontent.com/vulbyte/factors_of-cli/master/install_factors_of.sh -o install_factors_of.sh
sudo sh install_factors_of.sh
```

## 🎓 how to use:

simply type:

```
factors_of <number>
```

replacing "<number>" with your desired positive integer

## 🚩 optional flags:

`-all` to print inversion, aka, by default factors_of ignores any number where a < b, so it will then include nyumbers past a > b

`-tests` to print the current number being tested, not practical, but can be used for debugging, or simply to monitor really long tiles

## ⚠️ note:

- only works on integers from 1 to + 2^64

## enjoy! - vulbyte
