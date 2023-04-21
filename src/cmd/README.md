# Bash

## [Source](https://stackoverflow.com/a/12009972)

Bash is very common shell, for example. It uses Readline library to implement command line input. And so to say, it is very convenient to know Readline bindings since it is used not only in bash. For example, gdb also uses Readline to process input.

In Readline documentation you can find all navigation related bindings (and more): http://www.gnu.org/software/bash/manual/bash.html#Readline-Interaction

Short copy-paste if the link above goes down:

Bare Essentials

- Ctrl-b Move back one character.
- Ctrl-f Move forward one character.
- [DEL] or [Backspace] Delete the character to the left of the cursor.
- Ctrl-d Delete the character underneath the cursor.
- Ctrl-\_ or C-x C-u Undo the last editing command. You can undo all the way back to an empty line.

Movement

- Ctrl-a Move to the start of the line.
- Ctrl-e Move to the end of the line.
- Meta-f Move forward a word, where a word is composed of letters and digits.
- Meta-b Move backward a word.
- Ctrl-l Clear the screen, reprinting the current line at the top.

Kill and yank

- Ctrl-k Kill the text from the current cursor position to the end of the line.
- M-d Kill from the cursor to the end of the current word, or, if between words, to the end of the next word. Word boundaries are the same as those used by M-f.
- M-[DEL] Kill from the cursor the start of the current word, or, if between words, to the start of the previous word. Word boundaries are the same as those used by M-b.
- Ctrl-w Kill from the cursor to the previous whitespace. This is different than M- because the word boundaries differ.
- Ctrl-y Yank the most recently killed text back into the buffer at the cursor.
- M-y Rotate the kill-ring, and yank the new top. You can only do this if the prior command is C-y or M-y.

M is Meta key. For Max OS X Terminal you can enable "Use option as meta key" in Settings/Keyboard for that. For Linux its more complicated.

Update

Also note, that Readline can operate in two modes:

- emacs mode (which is the default)
- vi mode

To switch Bash to use vi mode:

$ set -o vi

Bonus

In macOS Terminal app (and in iTerm too) you can Option-Click to move the cursor (cursor will move to clicked position). This even works inside vim.

To revert the shell's command-line interface back to its default behavior after executing the set -o vi command, you can use the set +o vi command.

This will disable the Vi editing mode and revert the command-line interface to its default behavior. Here's an example:

```shell
$ set -o vi   # Enable Vi editing mode
$ ls          # Use Vi commands to edit the command
$ Esc k       # Move to previous command in history
$ set +o vi   # Disable Vi editing mode
$ ls          # Regular command-line interface
```

After executing set +o vi, the shell's command-line interface will no longer use Vi commands and will instead use its default behavior.

## [Source](https://stackoverflow.com/a/857312)

- Use Ctrl+x followed by Ctrl+e to open the current line in the editor specified by $FCEDIT or $EDITOR or emacs (tried in that order).
- If you ran the command earlier, hit Ctrl+r for a reverse history search and type option25 (in this case). The line will be displayed. Hit Tab to start editing at this point.
- Use history expansion with the s/// modifier. E.g. !-2:s/--option25/--newoption/ would rerun the second-to-last command, but replace option25. To modify the last ./cmd command, use the !string syntax: !./cmd:s/--option25/--newoption/
- Any delimiter may be used in place of / in the substitution.
- If editing the previous line, you can use quick substitution: ^--option25^--newoption
- Character search. This was mentioned by Pax, and can be done in regular emacs-mode with Ctrl+] for forward search, and Ctrl+Alt+] for backward search.

I recommend the second option. Ctrl+r is really handy and fast, no mucking about with editors, and you see the results before the command is run (unlike the history expansions).
