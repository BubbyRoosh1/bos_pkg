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
Most of this isn't done yet cause it's getting late and yeah.

* Add packages (I will say "Nice!" every time someone besides myself contributes.. not like this'll take off seeing as it's only a package manager but still).
* Add the ability to remove packages.
* Package category/name assumption.
* Search/Query (optional --regex)
* Other things idk
