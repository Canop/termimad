
<a name="v0.9.0"></a>
### v0.9.0 - 2020-11-26
- event source intercepts escape sequences and sends them (when finished) in a dedicated event variant
- breaking change: termimad event are no longer Copy

<a name="v0.8.30"></a>
### v0.8.30 - 2020-11-13
- add the FitStr utility taken from broot (correct string cutting taking real char width in cols)
(note that not all functions in termimad precisely take all chars width in cols into account)

<a name="v0.8.29"></a>
### v0.8.29 - 2020-10-15
- allow default value in template expansion

<a name="v0.8.29"></a>
### v0.8.29 - 2020-10-15
- allow default value in template expansion

<a name="v0.8.28"></a>
### v0.8.28 - 2020-10-11
- use the OwningTemplateExpander of Minimad 0.6.6

<a name="v0.8.27"></a>
### v0.8.27 - 2020-10-07
- fix inverted move_left and move_to_start

<a name="v0.8.26"></a>
### v0.8.26 - 2020-08-07
- upgrade crossterm to 0.17.7

<a name="v0.8.25"></a>
### v0.8.25 - 2020-07-13
- interpred lines with just ">" as empty quotes
- fix panic on wrapping long strings without space

<a name="v0.8.24"></a>
### v0.8.24 - 2020-06-22
- add a bunch of functions modifying the input (moving the cursor or deleting parts)

<a name="v0.8.23"></a>
### v0.8.23 - 2020-05-29
- fix uppercase letters not used in input field

<a name="v0.8.22"></a>
### v0.8.22 - 2020-05-25
- Prevent overflowing of large text from input field (some ellipsis added if necessary)

<a name="v0.8.21"></a>
### v0.8.21 - 2020-05-13
- EventSource: better manage channel errors

<a name="v0.8.20"></a>
### v0.8.20 - 2020-05-10
- relax the dependency version contraint on crossterm - Fix #18

<a name="v0.8.18"></a>
### v0.8.18 - 2020-05-05
- input fields now have a "focused" bool in their state

<a name="v0.8.17"></a>
### v0.8.17 - 2020-02-28
- added event handling functions in input_field for when you don't use termimad events or have a complex event dispatching

<a name="v0.8.16"></a>
### v0.8.16 - 2020-02-26
- key modifiers in click events in event_source
- the new experimental feature flag: `special-renders` lets you define replacement chars (for now) with a out of skin rendering (contact me if you're interested by this kind of feature)

<a name="v0.8.15"></a>
### v0.8.15 - 2020-02-22
- clear function in compound_style

<a name="v0.8.14"></a>
### v0.8.14 - 2020-02-16
- check w in double-click detection

<a name="v0.8.13"></a>
### v0.8.13 - 2020-02-08
- use crossterm 0.16.0 for slightly improved attributes storage

<a name="v0.8.12"></a>
### v0.8.13 - 2020-01-19
- use crossterm 0.15 to fix ctrl-J being read as Enter

<a name="v0.8.11"></a>
### v0.8.11 - 2020-01-19
- fix missing background on space after bullet in list

<a name="v0.8.10"></a>
### v0.8.10 - 2020-01-11
- use crossterm 0.14.2 for freeBSD compatibility

<a name="v0.8.9"></a>
### v0.8.9 - 2019-12-30
- fix the Enter key not recognized in combinations on some computers by normalizing '\r' and '\n' into 'Enter'

<a name="v0.8.8"></a>
### v0.8.8 - 2019-12-26
- allow language specification in code fences

<a name="v0.8.5"></a>
### v0.8.5 - 2019-12-20
- code fences support

<a name="v0.8.4"></a>
### v0.8.4 - 2019-12-16
- fix a compilation problem on windows (see https://github.com/Canop/termimad/issues/13#issuecomment-565848039)

<a name="v0.8.3"></a>
### v0.8.3 - 2019-12-15
- port to crossterm 0.14

<a name="v0.8.2"></a>
### v0.8.2 - 2019-11-29
- skin.print_expander makes using a text template less verbose

<a name="v0.8.1"></a>
### v0.8.1 - 2019-11-29
- TextView: draw the background till the end of line

<a name="v0.8.0"></a>
### v0.8.0 - 2019-11-24
- Templates allow going further in separating form from content

<a name="v0.7.6"></a>
### v0.7.6 - 2019-11-15
- fix skin's background not applied on empty lines at end of text_view
- use version minimad 0.4.3 to fix case of code not detected when following italic without space in between

<a name="v0.7.5"></a>
### v0.7.5 - 2019-11-13
- fix skin's background not applied on empty lines at end of text_view

<a name="v0.7.4"></a>
### v0.7.4 - 2019-11-11
- introduce inline templates, and especially the `mad_print_inline!` and `mad_write_inline!` macros
- add functions to shrink or extend a composite to a given width, using internal elision if possible

<a name="v0.7.3"></a>
### v0.7.3 - 2019-11-08
- add easy alternate to `skin.print_text` handling IO error

<a name="v0.7.2"></a>
### v0.7.2 - 2019-11-04
- incorporate crossterm 0.13.2 which fixes a regression on input reader

<a name="v0.7.1"></a>
### v0.7.1 - 2019-11-03
- compatibility with crossterm 0.13
- mouse support in stderr

<a name="v0.7.0"></a>
### v0.7.0 - 2019-09-22
- Displaying can be done on stderr or stdout, or in a subshell

<a name="v0.6.6"></a>
### v0.6.6 - 2019-10-05
- provide a default terminal width when the real one can't be measured

<a name="v0.6.5"></a>
### v0.6.5 - 2019-08-31
- list view: autoscroll on selection change
- list view: select_first_line & select_last_line

<a name="v0.6.4"></a>
### v0.6.4 - 2019-08-02
- add ProgressBar

<a name="v0.6.3"></a>
### v0.6.3 - 2019-08-01
- improvements of ListView

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

