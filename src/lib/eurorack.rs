pub mod er301;
pub mod ii;

#[derive(Debug)]
pub struct Command {
  pub command_number: u8,
  pub args: &'static [Arg],
  pub required: usize,
}

#[derive(Debug)]
pub struct Arg {
  pub name: &'static str,
  pub argtype: Bufsize,
}

#[derive(Debug)]
pub enum Bufsize {
  U8,
  S16,
  S16V,
}
#[derive(Debug)]
pub enum EuroModules {
  Er301,
}
