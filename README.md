# `bela-i2c`

parsing msg from any device to Eurorack by using [Bela](https://bela.io/) to parse OSC -> i2c, since Bela i2c pin requires 3.3V, for safety we might need [ logic level converter ](https://shopee.co.th/search?keyword=logic%20level%20converter%20%E0%B9%80%E0%B8%84%E0%B8%A3%E0%B8%B7%E0%B9%88%E0%B8%AD%E0%B8%87%E0%B8%A1%E0%B8%B7%E0%B8%AD%E0%B9%84%E0%B8%9F%E0%B8%9F%E0%B9%89%E0%B8%B2%E0%B9%81%E0%B8%A5%E0%B8%B0%E0%B9%80%E0%B8%84%E0%B8%A3%E0%B8%B7%E0%B9%88%E0%B8%AD%E0%B8%87%E0%B8%A1%E0%B8%B7%E0%B8%AD%E0%B8%8A%E0%B9%88%E0%B8%B2%E0%B8%87&showItems=true) to use with Eurorack level (5V). inspired by [hans](https://llllllll.co/t/hans/36455/14).


# Quick Start (re-write later)
```
# run bela-osc
./build.sh

# open another window
cargo run
```

# Usage (currently support only ER-301)

osc pattern = `module/module_number/command/output_port value` eg. `/er301/1/cv_slew/1 1000`
use same configuration as Teletype( value = unsigned 14bits integer ( 16,384 = 10v ))/


# [Optional] send data via WIFI ( eg. use TouchOSC (Mobile) -> Bela )

<img src="images/bela-setup-ip.png?sanitize=true">

- setup WIFI on Bela first, details [here](https://learn.bela.io/using-bela/bela-techniques/connecting-to-wifi/).
- make sure you're in same WIFI as Bela.
- on Bela
  - get wlan IP by `ip a` on Bela board.
- on TouchOSC:
  - set HOST to IP obtained by previous command (eg. `192.168.1.115` as shown above).
  - set Outgoing to `7562` (default Bela's listening port).

# Notes

# Disclaimer

this have been successfully tested on
Bela image: v0.3.2, (released 13 March 2018).
OSX: 10.13.6 (High Sierra)
MacBook Pro (Retina, 13-inch, Early 2013)

# Licence

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

# Resources

- [bela-sys](https://github.com/padenot/bela-sys)
- [hans](https://github.com/nordseele/hans_rust)
