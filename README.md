🍃 Leaf Tasks
=============

Leaf Tasks is a lightweight, self-hosted task tracking (todo) tool.

I created Leaf Tasks as replacement for my somewhat specific use of
Wunderlist. I curate, [Read Rust], a site that collects interesting
posts from the Rust community. My workflow for the site is mostly
powered by RSS and [Feedbin] but when I encounter a post outside of
Feedbin, typcally on my phone I use the iOS share sheet functionality
to add a link to Wunderlist. With Wunderlist being 

Features
--------

What's included:

* A simple task list that lets you add and complete tasks, that's all.
* Uncluttered design.
* Plain text (CSV) storage.
* Uses plain old HTML forms — works in almost any browser,
  including [Lynx].
* Single file, dependency-free binary.
* Super fast (typical response times ~20µs), memory efficient (~1.3Mb)
  server.

What's not included:

* JavaScript.
* User tracking of any kind.
* Multiple lists.
* Multiple users.
* Sharing (outside of sharing a login).
* Task editing.
* Task deletion.
* Viewing completed tasks in the UI (although they are stored in a file).

Download
--------

TODO

Using
-----

### Shortcuts Workflow for iOS

This workflow for the built-in Shortcuts app allows you to add new tasks using the
standard share sheet.

TODO: Finish

### Tips

* There's no need to click the Save button when adding a task. Just hit Enter
  and the default browser behaviour of submitting the form will take place.

### Font

To minimise page weight Leaf does not use and web fonts. However it was
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

Leaf uses environment variables for configuration. To run the server the
following environment variables need to be set.

#### `LEAF_PASSWORD_HASH`

This contains the password hash used to verify you when logging in. The value
can be generated with the `argon2` tool. This tool is installed by default on
Arch Linux. If you are using a different system you may need to install it, the
package is probably called `argon2`.

The shell snippet below will read your from stdin and then prints the hash.
Type your chosen password and press Enter, note that it will echo in the
terminal.  See below for an
[explanation of the snippet](#password-hash-shell-snippet-explanation).

    (read -r PASS; echo -n "$PASS" | argon2 $(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c 8) -e)

You should see something like the following, which is what `LEAF_PASSWORD_HASH`
should be set to.

    $argon2i$v=19$m=4096,t=3,p=1$eEVkYlJFZGY$N0p7VxqHDGBZ1ivgotGv2olZ/eXM9WPPCRf0wZuyyLo

**Note:** The hash contains `$` characters so be aware of shell quoting issues.
If setting the var in a shell use single quotes:

    export LEAF_PASSWORD_HASH='$argon2i$v=19$m=4096,t=3,p=1$eEVkYlJFZGY$N0p7VxqHDGBZ1ivgotGv2olZ/eXM9WPPCRf0wZuyyLo'
    
#### `LEAF_SECURE_COOKIE` (optional)

**Default:** `true`

Whether the login cookie sets [the secure flag][secure-cookie]. For local development
without https, set this to `false.`

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

    LEAF_SECURE_COOKIE=false watchexec -w src -s SIGINT -r 'cargo run'
    
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
  * `$()` runs a command and supplies substitutes it with the output from the
    command. 
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
