# .bashrc

# Source global definitions
if [ -f /etc/bashrc ]; then
	. /etc/bashrc
fi

# User specific environment
if ! [[ "$PATH" =~ "$HOME/.local/bin:$HOME/bin:" ]]; then
	PATH="$HOME/.local/bin:$HOME/bin:$PATH"
fi
export PATH

# Uncomment the following line if you don't like systemctl's auto-paging feature:
# export SYSTEMD_PAGER=

# User specific aliases and functions
if [ -d ~/.bashrc.d ]; then
	for rc in ~/.bashrc.d/*; do
		if [ -f "$rc" ]; then
			. "$rc"
		fi
	done
fi

# Source: Command Line Cheat Sheets by Elijah Manor.
alias tldrf='tldr --list | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr'

export VISUAL=nvim
export EDITOR=nvim

unset rc
. "$HOME/.cargo/env"

source /home/lloyd/.config/broot/launcher/bash/br

# =============================================================================
#
# To initialize zoxide, add this to your configuration (usually ~/.bashrc):
#
# ============================================================================= #
eval "$(zoxide init bash)"

# =============================================================================
#
# To initialize starship cross-shell prompt:
#
# ============================================================================= #
eval "$(starship init bash)"

# =============================================================================
#
# NVM for NodeJS:
#
# ============================================================================= #
# export NVM_DIR="$HOME/.nvm"
# [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
# [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion

# pnpm
export PNPM_HOME="/home/lloyd/.local/share/pnpm"
case ":$PATH:" in
*":$PNPM_HOME:"*) ;;
*) export PATH="$PNPM_HOME:$PATH" ;;
esac
# pnpm end
