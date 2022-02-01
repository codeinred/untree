# Untree: Undoing tree for fun and profit

Untree converts tree diagrams produced by
[tree](https://linux.die.net/man/1/tree) back into directory file structures.

Let's say you have the following directory structure, created by running `tree`
in the root of this project:

```
.
├── Cargo.lock
├── Cargo.toml
├── inputs
│   └── test1.tree
├── README.md
└── src
    ├── macros.rs
    ├── main.rs
    ├── traits.rs
    └── types.rs

2 directories, 8 files
```

untree can mirror that directory structure, just based on that input:

```bash
tree | untree --dir test
```

Here, `test` is the destination directory where `untree` is supposed to create
files. Now, if we `tree` the newly created directory, we can see that it has the
same structure as the repository:

```
test
├── Cargo.lock
├── Cargo.toml
├── inputs
│   └── test1.tree
├── README.md
└── src
    ├── macros.rs
    ├── main.rs
    ├── traits.rs
    └── types.rs

2 directories, 8 files
```

`untree` can also read in the tree from an input file, or you can paste it in
directly since it accepts input from standard input:

![](media/image1.png)

## Motivating untree

I've noticed that in the past I've had to recreate directory structures in order
to answer questions or run tests on the directory. For example, [this
question][stack-overflow-question] asks about ignoring certain kinds of files,
and it provides a directory structure as reference.

The files themselves aren't provided, nor do they need to be, but the directory
structure itself _is_ relevant to the question.

`untree` allows you to exactly replicate the directory structure, just by
pasting in the given directory tree:

![](media/image2.png)

Aside from allowing you to answer questions about directory structures, untree
is good for quickly creating directory structures for the purpose of mocking
input to other programs.

[stack-overflow-question]:
  https://stackoverflow.com/questions/70933172/how-to-write-gitignore-so-that-it-only-includes-yaml-files-and-some-specific-fi

## Comments, feedback, or contributions are welcome!

I'm in the progress of learning rust, so any feedback you have is greatly
appreciated! Also, if `untree` is useful to you, please let me know!
