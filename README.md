# foxtail

Claude Code statusline for Firefox developers. Shows model, cwd, time, context,
git status, session stats, cost, cache, etc.

![screenshot of the CLI](screenshot.png)
![screenshot of the CLI](screenshot-no-emojis.png)

## Usage

- Reads JSON on stdin and prints a status line.
- Flags:
  - `--print-config` print resolved config and a complete example
  - `--config <FILE>` use a specific config path
  - `--background {auto|light|dark}` set background mode (auto uses config, defaults to dark)
  - `--color {auto|always|never}` control ANSI color (respects `NO_COLOR`, `CLICOLOR`, `CLICOLOR_FORCE` when `auto`)
  - `--no-git` disable Git integration regardless of config
  - `--log-cost` append one `{"t":<receive time>,"c":<cost.total_cost_usd>}`
    line per received status payload to `<session id>.cost.jsonl` next to the
    session transcript

## Install

Install the binary:

```bash
cargo install foxtail
```

or using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall foxtail
```

or from a local copy:

```bash
cargo install --path .
```

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "foxtail",
    "padding": 0
  }
}
```

`./install.sh` automates that if you have a local copy.

## Config

Create `~/.claude/statusline.toml`, `~/.mozbuild/foxtail.toml`, or
`~/.config/foxtail/config.toml`, and run `foxtail --print-config` to see how to
configure it.

### Usage limits

`{limit5h}` and `{limit7d}` show how much of the Claude Code 5-hour and 7-day
usage windows is used, and optionally when each resets, e.g. `5h:23% ↻16:30`
or, with `use_emojis`, `🕔 23% ↻16:30` and `📅 60%`. The color goes from green
to red as the window is used up. Add them to `format` (they are not in the
default one):

```toml
format = "{head} | {model} | {ctx} | {limit5h} | {limit7d} | {tail}"
```

Claude Code only reports `rate_limits` for claude.ai Pro and Max subscribers
(or behind a Claude apps gateway that sets a spend limit; foxtail ignores the
gateway's `spend_limit`), and only after the first API response. Each window can
be missing on its own, and Claude Code drops a window once its `resets_at` time
passes. A missing window, or one whose reset time has passed, renders nothing.

The reset time is set per window with `limit5h_reset` and `limit7d_reset`
(defaults: `"clock"` and `"none"`; an unknown value falls back to the default):

| Style       | Output                                                     |
|-------------|------------------------------------------------------------|
| `clock`     | local reset time, `↻16:30`, or `↻Tue 16:30` on another day |
| `weekday`   | abbreviated day, `↻Tue`, or `↻16:30` if the reset is today |
| `countdown` | time left, `(2h10m)`                                       |
| `none`      | nothing                                                    |

```toml
limit5h_reset = "countdown"
limit7d_reset = "weekday"
```

Override the prefixes with the `limit5h` and `limit7d` keys of `[symbols]`, and
the `↻` marker with `limit_reset`.

## Development

Various scripts in this repo can help iterating on changes, don't reinvent the
wheel.

## License

Apache 2.0 or MIT

