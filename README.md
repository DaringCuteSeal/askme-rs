# AskMe-rs (askme-memorize)
Rewrite of [AskMe](https://github.com/DaringCuteSeal/askme) in Rust.

AskMe is a simple utility to help with memorizing terms, definitions, etc. AskMe files are written in [yaml](https://yaml.org/).


![Demo](demo.png)

# How it works

AskMe has many modes for you to pick from, currently, only `askme-memorize` is properly implemented, and `askme-multichoice` is currently being implemented. This program will have multiple binaries when installed.

# Usage

run `askme-* --help` where * is the askme mode you would like to run, or run it without any arguments to get a usage screen

## Writing AskMe Files
Here's an example question file:

```yaml
# Questionnaire configuration
title: My question title              # Question title (string)
subtitle: My question subtitle        # Question subtitle (string)

# List of questions
questions:
  - title: S                           # Question title (string)
    answers:                           # List of possible answers
      - Sulfur                         # Answer 1 (string)
      - Sulphur                        # Answer 2 (string)
  
  - title: W                           # Question title (string)
    answers:                           # List of possible answers
      - Wolfram                        # Answer 1 (string)

  - title: How many valence electrons does sodium have? # Question title (string)
    answers:                                            # List of possible answers
      - "1"                                             # Note: all answers must be strings!
```


# Building & Installing
```sh
git clone 'https://github.com/DaringCuteSeal/askme-rs.git' askme-rs
cd askme-rs
make
```

# Contributing & To-Do's
I appreciate contributions! Feel free to fork this repo and make PRs.

To-do's:
- [ ] Handle INT signal properly (print correct answers on termination)
- [ ] Don't force all fields to be declared