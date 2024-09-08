##### code modified from repo: https://github.com/breynard0/iced_inline_tweak

# Example for how to use iced and inline_tweak
### A simple integration for how to use the inline_tweak crate with iced. A few things to note:

- You may need to specify std::column when using the tweak! macro
- Hot-reloading enums is not currently supported
- For debug mode, the window is redraw in every frame (changing a value from the struct will trigger a redraw)
- For release mode, the window is redraw only when the state of the struct is changed
- You can´t change the layout content, only the values of his configuration 

### Thanks for the original idea to @breynard0 and the inline_tweak crate!