use {
    crate::clipboard,
    anyhow::{self},
    crossterm::{
        event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent},
        queue,
        terminal::{
            Clear,
            ClearType,
        },
    },
    std::io::Write,
    termimad::*,
};

pub const ESC: KeyEvent = KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE };
pub const TAB: KeyEvent = KeyEvent { code: KeyCode::Tab, modifiers: KeyModifiers::NONE };
pub const CONTROL_C: KeyEvent = KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL };
pub const CONTROL_V: KeyEvent = KeyEvent { code: KeyCode::Char('v'), modifiers: KeyModifiers::CONTROL };
pub const CONTROL_X: KeyEvent = KeyEvent { code: KeyCode::Char('x'), modifiers: KeyModifiers::CONTROL };

/// The view covering the whole terminal, with its widgets and current state
pub struct View {
    area: Area,
    drawable: bool, // is the area big enough
    label_skin: MadSkin, // the skin used to render the 3 labels
    introduction: MadView,
    login_label_area: Area,
    login_input: InputField,
    password_label_area: Area,
    password_input: InputField,
    comments_label_area: Area,
    comments_input: InputField,
    focus: Focus, // where is the current focus : an input or the introduction text
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
    Introduction,
    Login,
    Password,
    Comments,
}

impl Focus {
    pub fn next(self) -> Self {
        use Focus::*;
        match self {
            Introduction => Login,
            Login => Password,
            Password => Comments,
            Comments => Introduction,
        }
    }
}

impl Default for View {
    /// Create the view with all its widgets
    fn default() -> Self {
        let mut label_skin = MadSkin::default();
        label_skin.paragraph.align = Alignment::Right;
        label_skin.headers[1].align = Alignment::Right;
        let mut view = Self {
            area: Area::uninitialized(),
            drawable: false,
            label_skin,
            introduction: MadView::from(
                MD_INTRO.to_owned(),
                Area::uninitialized(),
                MadSkin::default(),
            ),
            login_label_area: Area::uninitialized(),
            login_input: InputField::default(),
            password_label_area: Area::uninitialized(),
            password_input: InputField::default(),
            comments_label_area: Area::uninitialized(),
            comments_input: InputField::default(),
            focus: Focus::Login,
        };
        view.login_input.set_normal_style(CompoundStyle::with_fgbg(gray(22), gray(2)));
        view.password_input.set_normal_style(CompoundStyle::with_fgbg(gray(22), gray(2)));
        view.comments_input.set_normal_style(CompoundStyle::with_fgbg(gray(22), gray(2)));
        view.password_input.password_mode = true;
        view.set_focus(Focus::Login);
        view.comments_input.new_line_on(InputField::ENTER);
        view.comments_input.set_str(MD_COMMENTS_VALUE);
        view
    }
}

impl View {
    pub fn new(area: Area) -> Self {
        let mut view = Self::default();
        view.resize(area);
        view
    }
    pub fn focused_input(&mut self) -> Option<&mut InputField> {
        match self.focus {
            Focus::Login => Some(&mut self.login_input),
            Focus::Password => Some(&mut self.password_input),
            Focus::Comments => Some(&mut self.comments_input),
            _ => None,
        }
    }
    fn set_focus(&mut self, focus: Focus) {
        self.focus = focus;
        self.login_input.set_focus(focus == Focus::Login);
        self.password_input.set_focus(focus == Focus::Password);
        self.comments_input.set_focus(focus == Focus::Comments);
    }
    pub fn focus_next(&mut self) {
        self.set_focus(self.focus.next());
    }
    pub fn resize(&mut self, area: Area) {
        self.drawable = area.width >= 20 && area.height >= 15;
        if self.drawable {
            let h = 4 + (area.height - 15) / 2;
            let intro_area = Area::new(1, 1, area.width - 3, h);
            self.introduction.resize(&intro_area);
            let y = intro_area.bottom() + 2;
            let half_width = (area.width -3 ) / 2;
            self.login_label_area = Area::new(1, y, half_width, 1);
            self.login_input.change_area(half_width + 2, y, half_width);
            let y = y + 2;
            self.password_label_area = Area::new(1, y, half_width, 1);
            self.password_input.change_area(half_width + 2, y, half_width);
            let y = y + 2;
            let h = area.height - 1 - h;
            self.comments_label_area = Area::new(1, y, half_width, h);
            self.comments_input.set_area(
                Area::new(half_width + 2, y, half_width, area.height - y - 1)
            );
        }
        self.area = area;
    }
    pub fn apply_key_event(&mut self, key: KeyEvent) -> bool {
        if key == ESC {
            self.set_focus(Focus::Introduction);
            true
        } else if key == TAB {
            self.focus_next();
            true
        } else if let Some(input) = self.focused_input() {
            input.apply_key_event(key) || {
                if key == CONTROL_C {
                    clipboard::copy_from_input(input)
                } else if key == CONTROL_X {
                    clipboard::cut_from_input(input)
                } else if key == CONTROL_V {
                    clipboard::paste_into_input(input)
                } else {
                    false
                }
            }
        } else {
            self.introduction.apply_key_event(key)
        }
    }
    pub fn apply_mouse_event(&mut self, mouse_event: MouseEvent) {
        if self.login_input.apply_mouse_event(mouse_event, false) {
            self.set_focus(Focus::Login);
        } else if self.password_input.apply_mouse_event(mouse_event, false) {
            self.set_focus(Focus::Password);
        } else if self.comments_input.apply_mouse_event(mouse_event, false) {
            self.set_focus(Focus::Comments);
        } else {
            self.set_focus(Focus::Introduction);
        }
    }
    /// draw the view (not flushing)
    pub fn queue_on<W: Write>(&mut self, w: &mut W) -> anyhow::Result<()> {
        queue!(w, Clear(ClearType::All))?;
        if self.drawable {
            let skin = &self.label_skin;
            self.introduction.write_on(w)?;
            skin.write_in_area_on(w, "## Login:", &self.login_label_area)?;
            self.login_input.display_on(w)?;
            skin.write_in_area_on(w, "## Password:", &self.password_label_area)?;
            self.password_input.display_on(w)?;
            skin.write_in_area_on(w, MD_COMMENTS_LABEL, &self.comments_label_area)?;
            self.comments_input.display_on(w)?;
        } else {
            self.introduction.skin.write_in_area_on(
                w,
                "*Sorry*, this terminal is **way** too small!",
                &self.area,
            )?;
        }
        Ok(())
    }
}

static MD_INTRO: &str = r#"# Scrollable Texts and Inputs

This example demonstrates scrollable texts, simple and multiline inputs, and automatically adapting to terminal resizing.

- use **ctrl**-**Q** to quit the application
- use the mouse or the **tab** key to give the focus to a widget
- use the **page-up** and **page-down** keys to scroll multi-line inputs when necessary
"#;

static MD_COMMENTS_LABEL: &str = r#"## Comments:
Use **enter** to add a newline
(if you want to build an application using **enter** for field validation, you can ask the input to handle **alt**-**enter** as newline instead)
"#;

static MD_COMMENTS_VALUE: &str = r#"This is the same kind of input than the previous ones
but with a higher area and allowed to create newlines.

Try editing it with long sentences
or scroll it
or resize the terminal.
"#;


