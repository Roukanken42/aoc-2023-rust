<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2023

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

|            Day            | Part 1 |  Part 2   |
|:-------------------------:| :---: |:---------:|
| [Day 1](./src/bin/01.rs)  | `548.3µs` | `557.0µs` |
| [Day 2](./src/bin/02.rs)  | `150.6µs` | `158.1µs` |
| [Day 3](./src/bin/03.rs)  | `508.0µs` | `243.1µs` |
| [Day 4](./src/bin/04.rs)  | `318.0µs` | `361.8µs` |
| [Day 5](./src/bin/05.rs)  | `34.8µs` | `71.9µs`  |
| [Day 6](./src/bin/06.rs)  | `349.0ns` | `687.0ns` |
| [Day 7](./src/bin/07.rs)  | `5.0ms` |  `6.8ms`  |
| [Day 8](./src/bin/08.rs)  | `424.5µs` | `15.0ms`  |
| [Day 9](./src/bin/09.rs)  | `300.7µs` | `301.5µs` |
| [Day 10](./src/bin/10.rs) | `292.1µs` |  `6.5ms`  |
| [Day 11](./src/bin/11.rs) | `229.0µs` | `220.6µs` |
| [Day 12](./src/bin/12.rs) | `819.5µs` | `20.1ms`  |
| [Day 13](./src/bin/13.rs) | `543.6µs` | `545.8µs` |
| [Day 14](./src/bin/14.rs) | `275.0µs` | `99.4ms`  |
| [Day 15](./src/bin/15.rs) | `84.9µs` | `589.8µs` |
| [Day 16](./src/bin/16.rs) | `1.1ms` | `23.9ms`  |
| [Day 17](./src/bin/17.rs) | `58.1ms` | `282.2ms` |
| [Day 18](./src/bin/18.rs) | `71.3µs` | `72.0µs`  |
| [Day 19](./src/bin/19.rs) | `856.6µs` | `672.2µs` |
| [Day 20](./src/bin/20.rs) | `2.8ms` |    `-`    |
| [Day 21](./src/bin/21.rs) | `271.2µs` | `21.0ms`  |
| [Day 22](./src/bin/22.rs) | `653.5µs` |  `8.0ms`  |
| [Day 23](./src/bin/23.rs) | `871.2µs` | `669.3ms` |
| [Day 24](./src/bin/24.rs) | `545.7µs` |  `2.8ms`  |
| [Day 25](./src/bin/25.rs) | `556.2s` |    `-`    |

**Total: 1233.59ms + 556.2s = 557.4s**
<!--- benchmarking table --->

---

## Template setup

This template supports all major OS (macOS, Linux, Windows).

### Create your repository 📝

1.  Open [the template repository](https://github.com/fspoettel/advent-of-code-rust) on Github.
2.  Click [Use this template](https://github.com/fspoettel/advent-of-code-rust/generate) and create your repository.
3.  Clone your repository to your computer.
4.  If you are solving a previous year's advent of code, change the `AOC_YEAR` variable in `.cargo/config.toml` to reflect the year you are solving.

### Setup rust 💻

1.  Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2.  (recommended) Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) extension for your code editor.
3.  (optional) Install a native debugger. If you are using VS Code, [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a good option.

---

✨ You can start solving puzzles now! Head to the [Usage section](#usage) to see how to use this template. If you like, you can configure [some optional features](#optional-template-features).

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. Use these unit tests to develop and debug your solutions against the example input.

Tip: when editing a solution, `rust-analyzer` will display buttons for running / debugging unit tests above the unit test blocks.

### Download input & description for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

By default, `solve` executes your code once and shows the execution time. If you append the `--time` flag to the command, the runner will run your code between `10` and `10.000` times (depending on execution time of first execution) and print the average execution time.

For example, running a benchmarked, optimized execution of day 1 would look like `cargo solve 1 --release --time`. Displayed _timings_ show the raw execution time of your solution without overhead like file reads.

#### Submitting solutions

> **Note**  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

In order to submit part of a solution for checking, append the `--submit <part>` option to the `solve` command.

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, `--release` controls whether real inputs will be used.

#### Update readme benchmarks

The template can output a table with solution times to your readme. Please note that these are not "scientific" benchmarks, understand them as a fun approximation. 😉

In order to generate a benchmarking table, run `cargo all --release --time`. If everything goes well, the command will output "_Successfully updated README with benchmarks._" after the execution finishes.

### Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

### Read puzzle description in terminal

> **Note**  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

## Optional template features

### Configure aoc-cli integration

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli --version 0.12.0`
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the _Application_ or _Storage_ tab, and copy out the `session` cookie value. [^1]

Once installed, you can use the [download command](#download-input--description-for-a-day) and automatically submit solutions via the [`--submit` flag](#submitting-solutions).

### Automatically track ⭐️ progress in the readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`.
-   `AOC_YEAR`: the year you want to track. Example: `2021`.
-   `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

Go to the _Variables_ tab in your repository settings and create the following variable:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker.

✨ You can now run this action manually via the _Run workflow_ button on the workflow page. If you want the workflow to run automatically, uncomment the `schedule` section in the `readme-stars.yml` workflow file or add a `push` trigger.

### Check code formatting / clippy lints in CI

Uncomment the respective sections in the `ci.yml` workflow.

### Use VS Code to debug your code

1.  Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
2.  Set breakpoints in your code. [^3]
3.  Click _Debug_ next to the unit test or the _main_ function. [^4]
4.  The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessred.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Common pitfalls

-   **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
[^3]:
    <img src="https://user-images.githubusercontent.com/1682504/198838369-453dc22c-c645-4803-afe0-fc50d5a3f00c.png" alt="Set a breakpoint" width="450" />

[^4]:
    <img alt="Run debugger" src="https://user-images.githubusercontent.com/1682504/198838372-c89369f6-0d05-462e-a4c7-8cd97b0912e6.png" width="450" />

[^5]:
    <img alt="Inspect debugger state" src="https://user-images.githubusercontent.com/1682504/198838373-36df6996-23bf-4757-9335-0bc4c1db0276.png" width="450" />
