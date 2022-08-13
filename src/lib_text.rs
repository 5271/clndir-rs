use termion::{color::*, style};

pub fn generic_error(msg: String) {
  println!(
    "{}{}[ Error ]{}{} {}",
    Fg(Red),
    style::Bold,
    Fg(Reset),
    style::Reset,
    msg
  );
}

pub fn config_error(line: u16, msg: String) {
  println!(
    "{}{}[ Config Error | Line: {} ]{}{} {}",
    Fg(Red),
    style::Bold,
    line,
    Fg(Reset),
    style::Reset,
    msg
  );
}

/*pub fn config_warning (line : u16, msg : String) {
  println!("{}{}[ Config Warning | Line: {} ]{}{} {}",
      Fg(Yellow),
      style::Bold,
      line,
      Fg(Reset),
      style::Reset,
      msg);
}*/

pub fn generic_warning(msg: String) {
  println!(
    "{}{}[ Warning ]{}{} {}",
    Fg(Yellow),
    style::Bold,
    Fg(Reset),
    style::Reset,
    msg
  );
}

pub fn info(msg: String) {
  println!(
    "{}{}[ Info ]{}{} {}",
    Fg(LightBlue),
    style::Bold,
    Fg(Reset),
    style::Reset,
    msg
  );
}

pub fn help_panel() {
  println!(
"{}{}clndir{} {}
A directory cleaner with many features and configuration options.

{}{}USAGE:{}
  clndir [ARGS AND DIRECTORIES TO CLEAN]...

{}{}ARGS:{}
  {}-h --help         {}Display this message
  {}-v --version      {}Show version of the program
  {}-s --silent       {}Program will display no errors or warnings
  {}-d --default      {}Program will not read the configs and remain with the default values
  {}-o --output       {}Program will display more information about what it's doing
  {}-m --no-misc      {}Makes program not throw all other files not included in sorting directories
  {}-n --name-sorting {}Program first checks if the file fits to any sorting directory by it's name
  {}  --only-name     {}Program sorts only by name
  {}  --only-format   {}Program sorts only by format",
  Fg(Green), style::Bold, style::Reset, env!("CARGO_PKG_VERSION"),
  Fg(Yellow), style::Bold, style::Reset,
  Fg(Yellow), style::Bold, style::Reset, 
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,
  Fg(Green), style::Reset,);
}

pub fn version_panel() {
  println!(
    "{}clndir{} {}",
    Fg(Green),
    style::Reset,
    env!("CARGO_PKG_VERSION")
  );
}