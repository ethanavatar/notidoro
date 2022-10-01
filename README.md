# notidoro

A command line Pomodoro timer that sends desktop notifications on each interval.

## Contents
- [TODO List](#todo-list)
- [Installation](#installation)
- [Usage](#usage)

## TODO List

- [ ] Add a noise to the notifications
- [ ] Properly format time in logs and notifications
    - [ ] Use a logging library
    - [ ] [Maybe] switch to `chrono` for time
- [ ] Add a config file

## Installation

```bash
$ git clone https://github.com/ethanavatar/notidoro.git
$ cd notidoro
$ cargo install --path .
```

## Usage

```bash
$ notidoro --help
Usage: notidoro.exe <TOTAL_HOURS> <WORK_MINS> <BREAK_MINS>

Arguments:
  <TOTAL_HOURS>
  <WORK_MINS>
  <BREAK_MINS>

Options:
  -h, --help  Print help information
```