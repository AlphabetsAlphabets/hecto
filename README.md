# Reminder
I'm bored with working on this project I'll stop working on it. I might return to it in the future. Contributions are still welcome.

# Hecto
Hecto is my attempt at making a very basic clone of vim/nvim. As for contributing feel free to do so. You can either add features, remove redudant ones, improve current ones etc. I am new to GitHub so it serves as practice for me when you do shoot in a pull request.

# Keybindings
Since there aren't a lot of keybindings in Hecto (yet), a dedicated wikipage is not needed (for now). I've done my best to make the keybindings as close to vim's as possible, but, there are definitely noticable differences if you come from vim.

- Capitalization on the keys are **important**, `J` is different from `j`.
- You may find yourself not being able to type. Look at the bottom left and make sure MODE is INSERT.
- **QUIT WITH CTRL+Q**
- **SAVE FILES WITH ALT+W**
- Use `:` to enter command mode, in order perform various commands.
    - Use `alt+j`, and `alt-k` to scroll up and down respectively.
    - `alt+d` to deselect your current selection.

### Moving up, down, left, and right
`h, j, k, l`  
Will be doing that. You can't use the arrow keys because I was too lazy to implement them. It's not hard, just a lot of typing.

`s, S`  
moving to the start of the first character in each line, and the final character of each line respectively.

`w, b`  
Moving forward a character until the next non-ascii alphabetic character, and moving backward a character until the first non-ascii alphabetic character is found respectively.

### Vertical movement
`J, K`
Scrolling up and down a page respectively.

`G, gg`
Scrolling to bottom and top of the page respectively.

