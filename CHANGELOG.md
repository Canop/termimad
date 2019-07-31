<a name="v0.6.2"></a>
### v0.6.2 - 2019-07-31
- fix build inconsistencies due to lack of precise sub crate versionning in crossterm

<a name="v0.6.1"></a>
### v0.6.1 - 2019-07-29
- add modifiable style for the input_field

<a name="v0.6.0"></a>
### v0.6.0 - 2019-07-28
Some tools that were parts of several Termimad based applications are now shared here:
- an event source emmiting events on a crossbeam channel
- an input field
- a list view with auto-resized columns

<a name="v0.5.1"></a>
### v0.5.1 - 2019-07-21
- a few utilies exported for apps with specific usages (compute_scrollbar, spacing.print_str, etc.)

<a name="v0.5.0"></a>
### v0.5.0 - 2019-07-09
- different styles for inline_code and code_block
- rgb now longer a macro but a free function
- two more color functions: ansi and gray
- clean and complete documentation

<a name="v0.4.0"></a>
### v0.4.0 - 2019-07-02
- support for horizontal rule (line made of dashes)
- support for quote (line starting with '>')
- support for bullet style customization (including colors)
- better wrapping, less frequently breaks words
- Skin API *breaking changes* to allow for more customization

