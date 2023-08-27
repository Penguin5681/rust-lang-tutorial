# Hello Cargo! <br>

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs  _dependencies_.) <br>

The simplest Rust programs, like the one we’ve written so far, don’t have any dependencies. If we had built the “Hello, world!” project with Cargo, it would only use the part of Cargo that handles building your code. As you write more complex Rust programs, you’ll add dependencies, and if you start a project using Cargo, adding dependencies will be much easier to do. <br>

check whether Cargo is installed by entering the following in your terminal:

    $ cargo --version

## Creating a project with cargo
Let’s create a new project using Cargo and look at how it differs from our original “Hello, world!” project. Navigate to the directory where you would store your Rust codes and run the following:

    $ cargo new hello_cargo  
    $ cd hello_cargo
Go into the _hello_cargo_ directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a _Cargo.toml_ file and a _src_ directory with a _main.rs_ file inside. <br>
"Cargo.toml" is a Cargo Configuration File in toml format (*Tom’s Obvious, Minimal Language*)