# bos_pkg

## W h a t
bos_pkg is (going to be) for the bos (pronounced like "boss") linux distribution.

## W h y
So I can say I'm using linux "like a boss" like from 2008 internet.

## H o w
Pretty simple.

1. Clone this repo: `git clone https://github.com/BubbyRoosh1/bos_pkg`
2. Build bos_pkg: `cargo build --release`
3. Move to PATH: `(sudo/doas) mv target/release/bos_pkg /usr/local/bin`
4. Move the other stuff lol: `(sudo/doas) mv var/db/bos_pkg /var/db`
5. Use the thing lol.

## TODO
* /etc/bos_pkg/Config.yaml
* Makeopts (-j4, -march=native, etc)
* Better package removal
* Add packages (pls :))
* Other things idk
