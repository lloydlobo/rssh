# Created by Zap installer
[ -f "$HOME/.local/share/zap/zap.zsh" ] && source "$HOME/.local/share/zap/zap.zsh"
plug "zsh-users/zsh-autosuggestions"
plug "zap-zsh/supercharge"
plug "zsh-users/zsh-syntax-highlighting"
plug "wintermi/zsh-starship"
plug "wintermi/zsh-rust"
plug 'zsh-users/zsh-history-substring-search'

# theme
plug "zap-zsh/zap-prompt" 

# =============================================================================
#
# To initialize zoxide, add this to your configuration (usually ~/.zshrc):
eval "$(zoxide init zsh)"

eval "$(starship init zsh)"

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