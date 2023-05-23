# Created by Zap installer
[ -f "$HOME/.local/share/zap/zap.zsh" ] && source "$HOME/.local/share/zap/zap.zsh"
plug "zsh-users/zsh-autosuggestions"
plug "zap-zsh/supercharge"
plug "zsh-users/zsh-syntax-highlighting"
#plug "wintermi/zsh-starship"
plug "wintermi/zsh-rust"
# plug 'zsh-users/zsh-history-substring-search'

# theme
plug "zap-zsh/zap-prompt"

# =============================================================================
#
# To initialize zoxide, add this to your configuration (usually ~/.zshrc):
eval "$(zoxide init zsh)"

eval "$(starship init zsh)"

# The Fuck alias
alias yeet='thefuck'

# alias for dc that runs cd instead.  # eval `thefuck --alias dc='cd'`

# Source: Command Line Cheat Sheets by Elijah Manor.
alias tldrf='tldr --list | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr'

path+=("$HOME/.cargo/bin")
export PATH

# User specific environment
if ! [[ "$PATH" =~ "$HOME/.local/bin:$HOME/bin:" ]]; then
	PATH="$HOME/.local/bin:$HOME/bin:$PATH"
fi
export PATH

export VISUAL=nvim
export EDITOR=nvim

# open a filtered history using fzf and copy the selected command to the clipboard using xsel -i -b.
alias hist="history | awk '{\$1=\"\"; print \$0}' | fzf --height 40% --reverse --tac | xsel -i -b"

# pnpm
export PNPM_HOME="/home/lloyd/.local/share/pnpm"
case ":$PATH:" in
  *":$PNPM_HOME:"*) ;;
  *) export PATH="$PNPM_HOME:$PATH" ;;
esac
# pnpm end

# region_start: deno
#
# Manually add the directory to your $HOME/.bashrc (or similar)
  export DENO_INSTALL="/home/lloyd/.deno"
  export PATH="$DENO_INSTALL/bin:$PATH"
# Run '/home/lloyd/.deno/bin/deno --help' to get started
# region_end: deno

# FZF VIM OPENER
# @source https://edward-rees.com/terminal-tricks/
function __fsel_files() {
  setopt localoptions pipefail no_aliases 2> /dev/null
  #eval find ./ -type f -not -path '*/.git/*' -print | fzf -m "$@" | while read item; do
  # if git repository:
  # git ls-files
 # eval find ./ -type f -print | fzf -m "$@" | while read item; do
 eval fd | fzf -m "$!" | while read item; do
    echo -n "${(q)item} "
  done
  local ret=$?
  echo
  return $ret
}

function fzf-vim {
    selected=$(__fsel_files)
    if [[ -z "$selected" ]]; then
        zle redisplay
        return 0
    fi
    zle push-line # Clear buffer
    BUFFER="nvim $selected";
    zle accept-line
}
zle -N fzf-vim
bindkey "^v" fzf-vim


# It's worth noting that zsh has its own built-in correction mechanism called correct. You can enable it by adding the following line to your .zshrc file:
#
# setopt correct
#
# With correct enabled, zsh will attempt to correct your command if it detects a spelling mistake or other error. You can also use the nocorrect command to disable correction for a specific command. For example:
#
# nocorrect dc
#
# This will prevent zsh from attempting to correct dc if it is mistyped.
