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

# more for less
#
# @source https://github.com/Mic92/dotfiles/blob/main/home/.bashrc
export LESS=-R # use -X to avoid sending terminal initialization
export LESS_TERMCAP_mb=$'\e[01;31m'
export LESS_TERMCAP_md=$'\e[01;31m'
export LESS_TERMCAP_me=$'\e[0m'
export LESS_TERMCAP_se=$'\e[0m'
export LESS_TERMCAP_so=$'\e[01;44;33m'
export LESS_TERMCAP_ue=$'\e[0m'
export LESS_TERMCAP_us=$'\e[01;32m'# more for less

# history
#
# @source https://github.com/Mic92/dotfiles/blob/main/home/.bashrc
export HISTIGNORE="&:ls:[bf]g:exit:reset:clear:cd*"
export HISTCONTROL="ignoreboth:erasedups"
export HISTSIZE=1000
export HISTFILESIZE=2000


# Source: Command Line Cheat Sheets by Elijah Manor.
alias tldrf='tldr --list | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr'

export VISUAL=nvim
export EDITOR=nvim

# ```shell
# $ set -o vi   # Enable Vi editing mode
# $ ls          # Use Vi commands to edit the command
# $ Esc k       # Move to previous command in history
# $ set +o vi   # Disable Vi editing mode
# $ ls          # Regular command-line interface
# ```

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

# region_start: deno
#
# Manually add the directory to your $HOME/.bashrc (or similar)
export DENO_INSTALL="/home/lloyd/.deno"
export PATH="$DENO_INSTALL/bin:$PATH"
# Run '/home/lloyd/.deno/bin/deno --help' to get started
# region_end: deno

# Press Ctrl + R to open the reverse search prompt.
#
# Type in a search term and press Enter.
#
# The fzf utility will open a fuzzy search interface with your command history filtered by the search term.
#
# Use the arrow keys to navigate the search results and select a command.
#
# Press Enter to execute the selected command.
#
# Bind fzf to Ctrl+R for reverse history search
if [[ $- =~ .*i.* ]]; then
	bind -x '"\C-r": fzf-history-widget'
fi
# Define the fzf-history-widget function
function fzf-history-widget() {
	local cmd
	cmd=$(history | awk '{$1=""; print $0}' | fzf --height 40% --reverse --tac)
	if [[ -n $cmd ]]; then
		# echo -n "$cmd" | xclip -selection clipboard
		echo -n "$cmd" | xsel -ib # copy to clipboard (not primary)
		# This causes the selected command to be inserted at the current cursor position in the shell's input buffer.
		READLINE_LINE="$cmd"
		READLINE_POINT=${#READLINE_LINE}
	fi
}

## zsh
# Bind fzf to Ctrl+R for reverse history search
# if [[ -n "$(command -v fzf)" ]]; then
# 	bindkey '^r' fzf-history-widget
# 	zle -N fzf-history-widget
# fi

# if [[ -n "$(command -v fzf)" ]]; then: checks if the fzf utility is installed on the system.
# if [[ -n "$(command -v fzf)" ]]; then
# 	echo "fzhist: fzf reverse search on steroids"
# 	bind -x '"\C-r": "$(fc -l 1 | fzf | sed '\''s/^[[:space:]]*//'\'' | cut -d '\'' '\'' -f 2-)"'
# fi

