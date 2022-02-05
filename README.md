# Untree: Undoing tree for fun and profit

Untree inverts the action of [tree] by converting tree diagrams of directory
structures back into directory structures. Given a directory structure, [tree]
produces a tree diagram, and given a tree diagram, untree produces a directory
structure.

Let's say you have the following directory structure, created by running `tree`
in the root of this project:

<pre><font color="#75A1FF"><b>.</b></font>
├── Cargo.lock
├── Cargo.toml
├── <font color="#75A1FF"><b>inputs</b></font>
│   └── test1.tree
├── <font color="#75A1FF"><b>lib</b></font>
│   ├── either.rs
│   ├── errors.rs
│   ├── functions.rs
│   ├── mod.rs
│   ├── more_context.rs
│   ├── path_action.rs
│   └── types.rs
├── LICENSE.txt
├── <font color="#75A1FF"><b>media</b></font>
│   ├── <font color="#F580FF"><b>image1.png</b></font>
│   └── <font color="#F580FF"><b>image2.png</b></font>
├── README.md
└── <font color="#75A1FF"><b>src</b></font>
    └── main.rs
</pre>

untree can create a mirror that directory structure, just based on that input:

```bash
tree | untree --dir path/to/output/dir
```

Here, `test` is the destination directory where `untree` is supposed to create
files. Now, if we `tree` the newly created directory, we can see that it has the
same structure as the repository:

<pre><font color="#75A1FF"><b>path/to/output/dir</b></font>
├── Cargo.lock
├── Cargo.toml
├── <font color="#75A1FF"><b>inputs</b></font>
│   └── test1.tree
├── <font color="#75A1FF"><b>lib</b></font>
│   ├── either.rs
│   ├── errors.rs
│   ├── functions.rs
│   ├── mod.rs
│   ├── more_context.rs
│   ├── path_action.rs
│   └── types.rs
├── LICENSE.txt
├── <font color="#75A1FF"><b>media</b></font>
│   ├── <font color="#F580FF"><b>image1.png</b></font>
│   └── <font color="#F580FF"><b>image2.png</b></font>
├── README.md
└── <font color="#75A1FF"><b>src</b></font>
    └── main.rs

4 directories, 15 files</pre>

`untree` can also read in the tree from an input file, or you can paste it in
directly since it accepts input from standard input:

![Screenshot of untree running on input from stdin. The generated file was placed in path/to/output/dir][image1]

## Motivating untree

I've noticed that in the past I've had to recreate directory structures in order
to answer questions or run tests on the directory. For example, [this
question][stack-overflow-question] asks about ignoring certain kinds of files,
and it provides a directory structure as reference.

The files themselves aren't provided, nor do they need to be, but the directory
structure itself _is_ relevant to the question.

`untree` allows you to replicate the structure of a directory printed with tree,
making it easy to answer questions about programs that traverse the directory
tree. This means that untree is also good for quickly creating directory
structures for the purpose of mocking input to other programs.

## Using untree as a library

You can use untree as a library if you need that functionality included in your
program. In order to create a tree, invoke [`create_tree`] with the given
directory, `Lines` buffer, and options.

These options are very simple - there's [`UntreeOptions::verbose`], which will
tell [`create_tree`] and [`create_path`] to print out any directories or files
that were created when set, and [`UntreeOptions::dry_run`], which will print out
any directories or files without actually creating them (`dry_run` implies
`verbose`).

Below is an example usage:

```rust
use untree::*;
use std::io::{BufRead, BufReader, stdin, Lines};

let options = UntreeOptions::new()
    .dry_run(true)   // Set dry_run to true
    .verbose(true);  // Set verbose to true
let lines = BufReader::new(stdin()).lines();

create_tree("path/to/directory", lines, options)?;

# Ok::<(), Error>(())
```

Additional functions include

- [`create_path`], used to create a file or path with the given options,
- [`get_entry`], used to parse a line in a tree file,
- [`touch_directory`], used to create a directory,
- [`touch_file`], used to touch a file (does the same thing as unix touch)

The primary error type used by untree is [`Error`], which holds information
about a path and the action being done on it, in addition to the normal error
information provided by `io::Error`.

## User testimonials

When asked about _untree_, my friend said:

> I retroactively want that for my time trying to get Conan to work. It woulda
> made certain things just a little less painful.

— _some guy_ (He asked to be referred to as "some guy")

## Comments, feedback, or contributions are welcome!

I'm in the progress of learning rust, so any feedback you have is greatly
appreciated! Also, if `untree` is useful to you, please let me know!

[image1]: media/image1.png
[tree]: https://linux.die.net/man/1/tree
[stack-overflow-question]:
  https://stackoverflow.com/questions/70933172/how-to-write-gitignore-so-that-it-only-includes-yaml-files-and-some-specific-fi
