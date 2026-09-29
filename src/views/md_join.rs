
#[derive(Debug)]
struct State {
  pub output: String,
  pub remove_newlines: bool,
  pub in_a_codeblock: bool,
}
impl State {
  fn new() -> Self {
    Self{
      output: String::new(),
      remove_newlines: true,
      in_a_codeblock: false,
    }
  }
  // We use this method to handle where a line affects state for the next, ie.
  // explicit newlines
  fn add_line(
    &mut self,
    line: &str,
  ) {
    // Explicit newline
    if line.ends_with("  ") {
      self.remove_newlines = false;
      // Remove all the trailing spaces
      self.output.push_str(line.trim_end_matches(' '));
    }
    else if line.ends_with('\\') {
      self.remove_newlines = false;
      self.output.push_str(line);
      // pop off the '\\' before going on
      self.output.pop();
    }
    // Normal line
    else {
      self.output.push_str(line);
    };
    // Finally we add the newline after the text, always.
    self.output.push('\n');
  }
}

// Does line joining according to markdown syntax. Ie. normal newlines become
// blankspaces, unless otherwise indicated.
// (Currently doesn't do any line joining within block-quotes, otherwise should
// be correct.)
// (Has a quirk that it always adds a trailing newline to every line.)
fn join_joinable_lines(
  input: &str,
) -> String {
  // Construct state for parsing
  let mut state = State::new();
  // Go over each line, copying each into output
  // (Manual loop so we can progress it manually)
  let mut iter = input.lines().peekable();
  'lines: loop{
    let line = match iter.next() {
      Some(x) => x,
      None => { break; }
    };

    // Since codeblocks should prevent further logic we check for those first
    // Check for codeblock edges, with state tracking
    // As they are always handled like this they have precedence
    if line == "```" {
      state.in_a_codeblock = !state.in_a_codeblock;
      // the line after the codeblock end isn't allowed to join to it
      state.remove_newlines = false;
      state.add_line(line);
      continue;
    }
    // If we are in a codeblock we specifically do nothing and ignore md syntax
    if state.in_a_codeblock {
      state.add_line(line);
      continue;
    }
    // If indented codeblock we also need to flag that it isn't valid to join next
    // the next line to this one
    if
      line.starts_with('\t') ||
      line.starts_with("    ")
    {
      state.remove_newlines = false;
      state.add_line(line);
      continue;
    }

    // Similar handling for paragraphs
    if line == "" {
      state.remove_newlines = false;
      state.add_line(line);
      continue;
    }

    // Fancy recursion for block quotes, as they are allowed to contain nested
    // markdown
    if line.starts_with('>') {
      // Aggregate all lines that are part of this block (by start)
      let start_len = if line.starts_with("> ") { 2 } else { 1 };
      let mut block_lines = String::from(&line[start_len..]);
      while let Some(line) = iter.next_if(|s| s.starts_with('>')) {
        block_lines.push('\n');
        // Slice out potential indent, to prevent weird joins
        if line.starts_with("> ") {
          block_lines.push_str(&line[2..]);
        } else {
          block_lines.push_str(&line[1..]);
        }
      }
      // Then we recurse, as there can be markdown in the block
      let joined_entry_lines = join_joinable_lines(&block_lines);
      // And we add back the joined lines, with the indent put back
      for line in joined_entry_lines.lines() {
        // If the line is part of a blockquote from the inner we don't add a
        // space after the '>', otherwise we do.
        if line.starts_with('>') {
          state.output.push('>');
        }
        else {
          state.output.push_str("> ")
        }
        state.add_line(line);
      }
      // Finally prevent the next line from joining into the block
      state.remove_newlines = false;
      continue;
    }
    // Next look for line entry starts
    if
      // Unordered list start
      line.starts_with("- ") ||
      line.starts_with("* ") ||
      line.starts_with("+ ")
    {
      // Aggregate all lines that are part of this entry (by indentation)
      let entry_start = &line[..2];
      let mut entry_lines = String::from(&line[2..]);
      while let Some(line) = iter.next_if(|s| s.starts_with("  ") || s==&"") {
        entry_lines.push('\n');
        // Slice out indent, to prevent weird joins
        // (use get to handle if it is an empty line)
        entry_lines.push_str(line.get(2..).unwrap_or(""));
      }
      // Then we recurse, as there can be markdown in the entry
      let joined_entry_lines = join_joinable_lines(&entry_lines);
      // And we add back the joined lines, with the indent put back
      let mut first_loop = true;
      for line in joined_entry_lines.lines() {
        // The first line needs the list entry start.
        if first_loop {
          first_loop = false;
          state.output.push_str(entry_start);
          state.add_line(line);
        }
        // Don't add trailing spaces for empty lines
        else if line == "" {
          state.add_line(line);
        }
        // Otherwise add back the indentation
        else {
          state.output.push_str("  ");
          state.add_line(line);
        }
      }
      // Finally prevent the next line from joining into the list entry
      state.remove_newlines = false;
      continue;
    }
    //   Last we do fancy parsing of line contents
    //   Ordered list entry starts
    let mut first_loop = true;
    'chars: for (i, ch) in line.char_indices() {
      // Numbered list is recognized by possibly indented numbers
      if ch.is_numeric() { first_loop = false; continue 'chars; }
      // followed directly by a dot
      // If we get here it's a match, just add and take the next line
      if ch == '.' && !first_loop { 
        // Aggregate all lines that are part of this entry (by indentation)
        let entry_start = &line[..i + 2];
        let mut entry_lines = String::from(&line[i + 2..]);
        while let Some(line) = iter.next_if(|s| s.starts_with("  ") || s==&"") {
          entry_lines.push('\n');
          // Slice out indent, to prevent weird joins
          entry_lines.push_str(line.get(2..).unwrap_or(""));
        }
        // Then we recurse, as there can be markdown in the entry
        let joined_entry_lines = join_joinable_lines(&entry_lines);
        // And we add back the joined lines, with the indent put back
        let mut first_loop = true;
        for line in joined_entry_lines.lines() {
          // The first line needs the list entry start
          if first_loop {
            first_loop = false;
            state.output.push_str(entry_start);
            state.add_line(line);
          }
          // Don't add trailing spaces for empty lines
          else if line == "" {
            state.add_line(line);
          }
          // Otherwise add back the indentation
          else {
            state.output.push_str("  ");
            state.add_line(line);
          }
        }
        // Finally prevent next line from joining with the list entry
        state.remove_newlines = false;
        continue 'lines;
      };
      // Any other character before '.' or no digit before '.' means it isn't
      // an ordered list entry
      break 'chars;
    }

    // For each line finally check if the preceeding line precludes joining
    // If so we reset state and add the line without join
    // If so: no need to think, just reset state and add the line
    if !state.remove_newlines {
      state.remove_newlines = true;
      state.add_line(line);
      continue;
    }

    // If we get this far we can actually join the line with the preceeding.
    // Handle trying to join the first line to non-existent preceeding line
    if let Some(ch) = state.output.pop() {
      // Paranoid levels of insurance we don't delete any non-newline character
      // (shouldn't be reachable, as state.add_line ALWAYS adds '\n' after each line)
      if ch != '\n' { state.output.push(ch); }
      else { state.output.push(' '); }
    }
    state.add_line(line);
  }
  state.output
}
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn md_join_normal_lines(){
    assert_eq!(
      &join_joinable_lines("just\nsome\ntext\nto\njoin"),
      "just some text to join\n",
    )
  }
  #[test]
  fn md_join_paragraph() {
    assert_eq!(
      &join_joinable_lines("hello\nworld\n\nnice weather,\neh?\n"),
      "hello world\n\nnice weather, eh?\n"
    )
  }
  #[test]
  fn md_join_explicit_newlines() {
    assert_eq!(
      &join_joinable_lines("hello\nworld\\\nnice weather  \neh?\n"),
      "hello world\nnice weather\neh?\n"
    )
  }
  #[test]
  fn md_join_codeblock() {
    assert_eq!(
      &join_joinable_lines(
"Code:\n    source code
Other code:\n\tsourcerer\ncode
Other other code:\n```\nsourcerest\ncode\n```\nend\n"
      ),
"Code:\n    source code
Other code:\n\tsourcerer\ncode \
Other other code:\n```\nsourcerest\ncode\n```\nend\n"
    )
  }
  #[test]
  // Should be able to join lines within the same blockquote
  fn md_join_blockquote() {
    assert_eq!(
      &join_joinable_lines("> Hello\n> world!\n>> Nice\n>> weather!\n> Is\n> it?\n>> Yep!\n"),
      "> Hello world!\n>> Nice weather!\n> Is it?\n>> Yep!\n"
    )
  }
  // Lists should only join with indented lines
  #[test]
  fn md_join_list(){
    assert_eq!(
      &join_joinable_lines("- some\n+ list\n  to\n* join\nand not\n"),
      "- some\n+ list to\n* join\nand not\n",
    )
  }
  #[test]
  fn md_join_ordered_list() {
    assert_eq!(
      &join_joinable_lines("1. Fine\n  stuff\n244. Okay-ish other\nstuff\n"),
      "1. Fine stuff\n244. Okay-ish other\nstuff\n"
    )
  }
}
