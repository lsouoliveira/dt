# dt

A command line tool to manage dotfiles.

## Usage

```
Usage: dt [OPTIONS] [COMMAND]

Arguments:
  [COMMAND]  Custom command to run

Options:
  -s, --sync    Fetch and pull changes from remote
  -r, --reload  Reload dotfiles (needs to be implemented as a custom command)
  -o, --open    Open the dotfiles (needs to be implemented as a custom command)
  -h, --help    Print help
```
## Config
A config file is expected to be at `~/.dt.yml`. Example config:

```
root: ~/.dotfiles
commands:
    nvim: nvim ~/.dotfiles/nvim
    reload: tangle ${HOME}/.dotfiles/README.md; rm -rf $HOME/.config/nvim; cp -rf ~/.dotfiles/nvim $HOME/.config; zsh
    open: nvim ~/.dotfiles/README.md
```
