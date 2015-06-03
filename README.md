Cref
====

Cref is a commit message search tool for non-native English speakers.

## Description

There is a situation that I don't have proper expression to describe changes. So I created a tool to search commits across repositories using interactive shell.

## Usage

### Search

Search imported commits are filtered as you type.

```sh
$ cref
```

![](https://raw.githubusercontent.com/rejasupotaro/cref/master/images/search.gif)

### Import

Import repositories into `~/.cref/cref.db`. I import some Android libraries so I'm a Android developer.

```sh
$ cref import <repo>...
```

### List

List imported repositories.

```sh
$ cref list
```

### Update

Update imported repositories. `repo` is optional argument. If you don't specify `repo`, all imported repositories are updated.


```sh
$ cref update [<repo>...]
```

### Delete

Delete imported repository.

```sh
cref delete <repo>
```

## Install

This tool is under development. Commands would be changed. I don't recommend to use it for now.

## Licence

[MIT](https://raw.githubusercontent.com/rejasupotaro/cref/master/LICENSE)

## Author

[rejasupotaro](https://github.com/rejasupotaro)
