/*!

*/
#[macro_use(select)]
extern crate crossbeam;
#[macro_use]
extern crate lazy_static;

use crossterm::{
    AlternateScreen,
    Attribute,
    ClearType,
    Color::*,
    KeyEvent,
    Terminal,
    TerminalCursor,
};
use termimad::*;
use std::{
    collections::HashSet,
    fs,
    os::unix::fs::MetadataExt, // TODO windows compatibility...
    path::{Path, PathBuf},
    sync::{atomic::{AtomicU64, Ordering}, Arc},
    thread,
};
use crossbeam:: {
    channel::{Receiver, unbounded},
};

struct FileInfo {
    path: PathBuf,
    file_count: u64,
    size: u64,
    is_dir: bool,
}
impl FileInfo {
    /// implements a very crude file walker (much could be optimized)
    fn from_dir(path: PathBuf) -> FileInfo {
        let mut file_count = 1;
        let mut size = 0;
        let mut inodes = HashSet::<u64>::default(); // to avoid counting twice an inode
        let mut dirs: Vec<PathBuf> = Vec::new();
        dirs.push(path.clone());
        while let Some(dir) = dirs.pop() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for e in entries.flatten() {
                    file_count += 1;
                    if let Ok(md) = e.metadata() {
                        if md.is_dir() {
                            dirs.push(e.path());
                        } else if md.nlink() > 1 {
                            if !inodes.insert(md.ino()) {
                                // it was already in the set
                                continue; // let's not add the size
                            }
                        }
                        size += md.len();
                    }
                }
            }
        }
        FileInfo {
            path,
            file_count,
            size,
            is_dir: true,
        }
    }
}

const SIZE_NAMES: &[&str] = &["", "K", "M", "G", "T", "P", "E", "Z", "Y"];
/// format a number of as a string
pub fn u64_to_str(mut v: u64) -> String {
    let mut i = 0;
    while v >= 1200 && i < SIZE_NAMES.len() - 1 {
        v >>= 10;
        i += 1;
    }
    format!("{}{}", v, &SIZE_NAMES[i])
}

fn compute_children(root: &Path) -> Receiver<FileInfo> {
    lazy_static! {
        static ref PROC: PathBuf = Path::new("/proc").to_path_buf();
    }
    let (tx_comp, rx_comp) = unbounded();
    for entry in root.read_dir().expect("read_dir call failed").flatten() {
        if entry.path() == *PROC {
            continue; // size of this dir doesn't mean anything useful, let's just forget it
        }
        if let Ok(md) = entry.metadata() {
            if md.is_file() {
                tx_comp.send(FileInfo {
                    path: entry.path(),
                    file_count: 1,
                    size: md.len(),
                    is_dir: false,
                }).unwrap();
            }
            if md.is_dir() {
                let tx_comp = tx_comp.clone();
                thread::spawn(move||{
                    let fi = FileInfo::from_dir(entry.path());
                    tx_comp.send(fi).unwrap();
                });
            }
        }
    }
    rx_comp
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.headers[0].compound_style = CompoundStyle::with_attr(Attribute::Bold);
    skin.headers[0].align = Alignment::Left;
    skin.italic.set_fg(ansi(225));
    skin.bold = CompoundStyle::with_fg(Blue);
    skin
}

struct Screen<'t> {
    pub title: String,
    pub list_view: ListView<'t, FileInfo>,
    skin: &'t MadSkin,
    dimensions: (u16, u16),
    total_size: Arc<AtomicU64>,
}
impl<'t> Screen<'t> {
    pub fn new() -> Self {
        lazy_static! {
            static ref SKIN: MadSkin = make_skin();
        }
        let total_size = Arc::new(AtomicU64::new(0));
        let column_total_size = Arc::clone(&total_size);
        let columns = vec![
            ListViewColumn::new(
                "name",
                10, 50,
                Box::new(|fi: &FileInfo| ListViewCell::new(
                    fi.path.file_name().unwrap().to_string_lossy().to_string(),
                    if fi.is_dir { &SKIN.bold } else { &SKIN.paragraph.compound_style },
                )),
            ).with_align(Alignment::Left),
            ListViewColumn::new(
                "file count",
                10, 10,
                Box::new(|fi: &FileInfo| ListViewCell::new(
                    u64_to_str(fi.file_count),
                    &SKIN.paragraph.compound_style,
                )),
            ).with_align(Alignment::Right),
            ListViewColumn::new(
                "size",
                6, 8,
                Box::new(|fi: &FileInfo| ListViewCell::new(
                    u64_to_str(fi.size),
                    &SKIN.paragraph.compound_style,
                )),
            ).with_align(Alignment::Right),
            ListViewColumn::new(
                "size",
                15, 17,
                Box::new(move |fi: &FileInfo| {
                    let total_size = column_total_size.load(Ordering::Relaxed);
                    ListViewCell::new(
                        if total_size > 0 {
                            let part = (fi.size as f64) / (total_size as f64);
                            let mut s = format!("{:>3.0}% ", 100.0 * part);
                            for i in 0..10 {
                                s.push(if (i as f64) < (10.0 * part) - 0.5 { '█' } else { ' ' });
                            }
                            s
                        } else {
                            "".to_owned()
                        },
                        if fi.is_dir { &SKIN.bold } else { &SKIN.paragraph.compound_style },
                    )
                }),
            ).with_align(Alignment::Left),
        ];
        let area = Area::new(0, 1, 10, 10);
        let mut list_view = ListView::new(area, columns, &SKIN);
        list_view.sort(Box::new(|a, b| b.size.cmp(&a.size)));
        Self {
            title: "Welcome".to_owned(),
            skin: &SKIN,
            list_view,
            dimensions: (0, 0),
            total_size,
        }
    }
    pub fn add_to_total_size(&mut self, to_add: u64) {
        self.total_size.fetch_add(to_add, Ordering::Relaxed);
    }
    pub fn set_total_size(&mut self, total_size: u64) {
        self.total_size.store(total_size, Ordering::Relaxed);
    }
    pub fn display(&mut self) {
        let (w, h) = terminal_size();
        if (w, h) != self.dimensions {
            Terminal::new().clear(ClearType::All).unwrap();
            self.dimensions = (w, h);
            self.list_view.area.width = w;
            self.list_view.area.height = h - 4;
            self.list_view.update_dimensions();
        }
        self.skin.write_in_area(
            &self.title,
            &Area::new(0, 0, w, 1),
        ).unwrap();
        self.skin.write_in_area(
            "Hit *ctrl-q* to quit, *PageUp* or *PageDown* to scroll",
            &Area::new(0, h-2, w, 1),
        ).unwrap();
        self.list_view.display().unwrap();
    }
}

fn main() {
    let _alt_screen = AlternateScreen::to_alternate(true);
    let cursor = TerminalCursor::new();
    cursor.hide().unwrap();
    let mut screen = Screen::new();

    let root = Path::new("/");
    screen.title = format!("# **{}** *computing...*", root.as_os_str().to_string_lossy());

    let event_source = EventSource::new();
    let rx_user = event_source.receiver();

    let rx_comp = compute_children(&root);

    loop {
        screen.display();
        select! {
            recv(rx_comp) -> comp => {
                if let Ok(fi) = comp {
                    screen.add_to_total_size(fi.size);
                    screen.list_view.add_row(fi);
                } else {
                    //break;
                    // This happens on computation end (channel closed).
                    screen.title = format!("# **{}**", root.as_os_str().to_string_lossy());
                }
            }
            recv(rx_user) -> user_event => {
                if let Ok(user_event) = user_event {
                    let mut quit = false;
                    match user_event {
                        Event::Key(KeyEvent::Ctrl('q')) => {
                            quit = true;
                        }
                        Event::Key(KeyEvent::PageUp) => {
                            screen.list_view.try_scroll_pages(-1);
                        }
                        Event::Key(KeyEvent::PageDown) => {
                            screen.list_view.try_scroll_pages(1);
                        }
                        Event::Wheel(lines_count) => {
                            screen.list_view.try_scroll_lines(lines_count);
                        }
                        _ => {
                            //input_field.apply_event(&user_event);
                        }
                    };
                    event_source.unblock(quit); // this will lead to channel closing
                } else {
                    // The channel has been closed, which means the event source
                    // has properly released its resources, we may quit.
                    break;
                }
            }
        }
    }

    cursor.show().unwrap();
}

