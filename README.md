🍃 Leaf Tasks
=============

Leaf is a lightweight, web based, self-hosted task tracking (todo) tool.

I created Leaf Tasks as replacement for my somewhat specific use of
Wunderlist. I curate, [Read Rust], a site that collects interesting
posts from the Rust community. My workflow for the site is mostly
powered by RSS and [Feedbin] but when I encounter a post outside of
Feedbin — typically on my phone I use the iOS share sheet functionality
to add a link to Wunderlist. With Wunderlist being shut down in May
2020 I built Leaf as a replacement.

<img src="https://github.com/wezm/leaf/raw/master/screenshot.png" width="698">

Features
--------

What's included:

* A simple task list that lets you add and complete tasks.
* Uncluttered design.
* Plain text (CSV) storage.
* Uses plain old HTML forms — works in almost any browser, including [Lynx]
  ([Screenshot][lynx-screenshot]).
* Single file, dependency-free binary.
* Super fast — Typical response times are ~160µs.
* Memory efficient — Uses ~1.4Mb RAM.

What's not included:

* JavaScript.
* User tracking.
* Multiple lists.
* Multiple users.
* Sharing (outside of sharing a login).
* Task editing.
* Task deletion.
* Viewing completed tasks in the UI (although they are stored in a file).

Download
--------

Pre-built binaries are available:

* [FreeBSD 12.1 amd64](https://releases.wezm.net/leaf/0.1.0-alpha.2/leaf-0.1.0-alpha.2-amd64-unknown-freebsd.tar.gz)
* [Linux x86\_64](https://releases.wezm.net/leaf/0.1.0-alpha.2/leaf-0.1.0-alpha.2-x86_64-unknown-linux-musl.tar.gz)
* [Mac OS](https://releases.wezm.net/leaf/0.1.0-alpha.2/leaf-0.1.0-alpha.2-x86_64-apple-darwin.tar.gz)
<!-- * [Windows](https://releases.wezm.net/leaf/0.1.0-alpha.2/leaf-0.1.0-alpha.2-x86_64-pc-windows-gnu.zip) -->

Using
-----

### Shortcuts Workflow for iOS

This workflow for the built-in Shortcuts app allows you to add new tasks using the
standard share sheet.

<https://www.icloud.com/shortcuts/b90e0304a40545ff8c53b8ed3c63d131>

You will need to customise two things:

1. In the Text block with "Bearer `your-api-token`", replace `your-api-token`
   with the token the Leaf instance is using (`LEAF_API_TOKEN` environment
   variable).
2. In the URL block, replace https://example.com/tasks with the URL of your
   Leaf instance.

### Tips

* There's no need to click the Save button when adding a task. Just hit Enter
  and the default browser behaviour of submitting the form will take place.

### Font

To minimise page weight Leaf does not use any web fonts. However it was
designed using the [Muli font][Muli] and this font is specified in the CSS.
Install the font if you would like Leaf use it. If you'd rather not install it,
that's fine — Leaf will use your browsers default sans-serif font.

FAQ
---

### Why no editing or deletion of tasks?

Just complete it and add a new one.

### Why no completed task list?

Leaf stores all completed tasks in a separate file for manual review if needed.
To avoid unnecessarily complicating the UI it is not exposed there though.

### What if I accidentally complete a task?

Add it again as a new task. If you're unsure of the content review the completed
task list file manually.

### What if I really want multiple lists?

You can run multiple instances of Leaf. Each server process is very small.

Running
-------

### Configuration

Leaf uses environment variables for configuration.

#### `LEAF_PASSWORD_HASH`

This contains the password hash used to verify you when logging in. The value
can be generated with the `argon2` tool. This tool is installed by default on
Arch Linux. If you are using a different system you may need to install it, the
package is probably called `argon2`.

The shell snippet below will read your password from stdin and then print the
hash. Type your chosen password and press Enter, note that it will echo in the
terminal. See below for an
[explanation of the snippet](#password-hash-shell-snippet-explanation).

    (read -r PASS; echo -n "$PASS" | argon2 $(cat /dev/urandom | LC_ALL=C tr -dc 'a-zA-Z0-9' | head -c 8) -e)

You should see something like the following, which is what `LEAF_PASSWORD_HASH`
should be set to.

    $argon2i$v=19$m=4096,t=3,p=1$eEVkYlJFZGY$N0p7VxqHDGBZ1ivgotGv2olZ/eXM9WPPCRf0wZuyyLo

**Note:** The hash contains `$` characters so be aware of shell quoting issues.
If setting the var in a shell use single quotes:

    export LEAF_PASSWORD_HASH='$argon2i$v=19$m=4096,t=3,p=1$eEVkYlJFZGY$N0p7VxqHDGBZ1ivgotGv2olZ/eXM9WPPCRf0wZuyyLo'

#### `LEAF_API_TOKEN`

The contents of this environment variable is used as a Bearer token (password)
for the add task route. I use it to add tasks on my phone with the iOS
Shortcuts workflow above. It must be at least 64 characters long. I used my
[password manager][gopass] to generate mine.

    export LEAF_API_TOKEN=Insert64orMoreRandomCharactersHere

#### `ROCKET_SECRET_KEY`

This is used to encrypt the cookie used for authentication. I can be generated with:

    openssl rand -base64 32

#### `LEAF_TASKS_PATH` (optional)

**Default:** `tasks.csv` in the working directory.

The path to the CSV file that will store tasks. If it does not exist it will be
created.

#### `LEAF_COMPLETED_PATH` (optional)

**Default:** `completed.csv` in the working directory.

The path to the CSV file that will store completed tasks. If it does not exist
it will be created.

#### `LEAF_SECURE_COOKIE` (optional)

**Default:** `true`

Whether the login cookie sets [the secure flag][secure-cookie]. For local development
without https, set this to `false.`

#### Rocket Configuration

The web framework Leaf uses ([Rocket]), also has some of it's own configuration
options: <https://rocket.rs/v0.4/guide/configuration/>.

File Format
-----------

TODO

API
---

TODO

Development
-----------

### Auto-reloading server

To run the server during development and have it rebuild and restart when
source files are changed I use [watchexec]:

    watchexec -w src -s SIGINT -r 'cargo run'

### Linking with lld

Using `lld` speeds up linking. I see 0.71s vs. 1.76s for an incremental build,
15 vs. 19s for clean build. Add the following to `.cargo/config`:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = [
    "-C", "link-arg=-fuse-ld=lld",
]
```

Licence
-------

This project is dual licenced under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/wezm/leaf/blob/master/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/wezm/leaf/blob/master/LICENSE-MIT))

at your option.

Appendix
--------

### Password Hash Shell Snippet Explanation

* The outer brackets `()` run the command in a sub-shell, this is to prevent
  the `PASS` environment variable remaining set in your shell.
* `read -r PASS` reads a line from stdin and sets the `PASS` environment
  variable with the value read, minus the terminalting new-line. `-r` disables
  `\` escape sequence support.
* `echo -n "$PASS"` prints the password on stdout,  `-n` disables the trailing
  new-line.
* `read` + `echo` are used to avoid having the password in your shell history,
  if it has one.
* `$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c 8)` is used to generate a
  random "salt" for the hash.
  * `$()` runs a command and substitutes it with the output from that command.
  * `cat /dev/urandom` reads from the `dev/urandom` pseudo random number
    generator device and writes to stdout.
  * `tr -dc 'a-zA-Z0-9'` reads from stdin and drops (`-d`) characters not in
    the set supplied to `-c`. This has the effect of filtering the binary
    `/dev/urandom` data and only outputting characters 'a-zA-Z0-9'.
  * `head -c 8` reads the first 8 characters (`-c`) from stdin and outputs them
    on stdout.
* `argon2` receives the random salt as an argument, it reads the password from
  stdin and prints just the encoded hash on stdout (`-e`).

[Read Rust]: https://readrust.net/
[Feedbin]: https://feedbin.com/
[watchexec]: https://github.com/watchexec/watchexec
[Lynx]: https://lynx.invisible-island.net/
[Muli]: https://www.fontsquirrel.com/fonts/muli
[secure-cookie]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#Secure
[gopass]: https://www.gopass.pw/
[Rocket]: https://rocket.rs/
[lynx-screenshot]: https://github.com/wezm/leaf/blob/master/screenshot-lynx.png
