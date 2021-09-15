# Ignorance

Another client tool for retrieving `.gitignore` templates for language.

This tools fetches the templates from [here](https://github.com/github/gitignore) and caches them in
your filesystem.

## Install

```
cargo install ignorance
```

## Usage
```
ignorance [-options] [language]
```

Run `ignorance --help` to see what options are available.

### Examples

Get python .gitignore template (casing don't matter)

`ignorance python`

Refetch template by calling API again

`ignorance -u python`


Overwrites existing .gitignore immediately without prompt

`ignorance -f python`

Refetch template from API and overwrite

`ignorance -fu python`
