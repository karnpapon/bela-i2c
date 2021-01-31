use crate::lib::eurorack::*;

// cv use 14bits = 16284 to represent 0 ~ 10v.

pub const ADDRESSES: [usize; 3] = [0x31, 0x32, 0x33];
pub enum Cmd {
  Tr,
  TrTog,
  TrPulse,
  TrTime,
  TrPol,
  Cv,
  CvSlew,
  CvSet,
  CvOff,
}

pub struct Module {
  pub tr: Command,
  pub tr_tog: Command,
  pub tr_pulse: Command,
  pub tr_time: Command,
  pub tr_pol: Command,
  pub cv: Command,
  pub cv_slew: Command,
  pub cv_set: Command,
  pub cv_offset: Command,
}

pub const TR: Command = Command {
  command_number: 0x0,
  args: &[
    Arg {
      name: "port",
      argtype: Bufsize::U8,
    },
    Arg {
      name: "state",
      argtype: Bufsize::S16,
    },
  ],
  required: 1,
};
pub const TR_TOG: Command = Command {
  command_number: 0x01,
  args: &[Arg {
    name: "port",
    argtype: Bufsize::U8,
  }],
  required: 0,
};
pub const TR_PULSE: Command = Command {
  command_number: 0x5,
  args: &[Arg {
    name: "port",
    argtype: Bufsize::U8,
  }],
  required: 0,
};
pub const TR_TIME: Command = Command {
    // should be 0x2 ? according to teletype open-sources.
  command_number: 0x32,
  args: &[
    Arg {
      name: "port",
      argtype: Bufsize::U8,
    },
    Arg {
      name: "time",
      argtype: Bufsize::S16,
    },
  ],
  required: 1,
};
pub const TR_POL: Command = Command {
  command_number: 0x6,
  args: &[
    Arg {
      name: "port",
      argtype: Bufsize::U8,
    },
    Arg {
      name: "polarity",
      argtype: Bufsize::S16,
    },
  ],
  required: 1,
};
pub const CV: Command = Command {
  command_number: 0x10,
  args: &[
    Arg {
      name: "port",
      argtype: Bufsize::U8,
    },
    Arg {
      name: "volt",
      argtype: Bufsize::S16V,
    },
  ],
  required: 1,
};
pub const CV_SLEW: Command = Command {
  command_number: 0x12,
  args: &[
    Arg {
      name: "port",
      argtype: Bufsize::U8,
    },
    Arg {
      name: "time",
      argtype: Bufsize::S16V,
    },
  ],
  required: 1,
};
pub const CV_SET: Command = Command {
  command_number: 0x11,
  args: &[
    Arg {
      name: "port",
      argtype: Bufsize::U8,
    },
    Arg {
      name: "volt",
      argtype: Bufsize::S16V,
    },
  ],
  required: 1,
};
pub const CV_OFFSET: Command = Command {
  command_number: 0x15,
  args: &[
    Arg {
      name: "port",
      argtype: Bufsize::U8,
    },
    Arg {
      name: "offset",
      argtype: Bufsize::S16V,
    },
  ],
  required: 1,
};

pub fn cmd_from_string(cmd_name: &str) -> Option<Command> {
  match cmd_name {
    "tr" | "trigger" => Some(TR),
    "tr_tog" => Some(TR_TOG),
    "tr_pulse" | "trp" | "tr_p" => Some(TR_PULSE),
    "tr_time" => Some(TR_TIME),
    "tr_pol" => Some(TR_POL),
    "cv" => Some(CV),
    "cv_slew" => Some(CV_SLEW),
    "cv_set" => Some(CV_SET),
    "cv_off" => Some(CV_OFFSET),
    _ => None,
  }
}
