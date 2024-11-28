# ExtendedSQL

## What is it?

ExtendedSQL (ESQL) is a programming language designed to link simple backend requests directly to the database layer of the stack.

## How to run

### Prerequisites

1. [git](https://git-scm.com/downloads)
2. [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Running

Clone the repository into a folder with either

1. https: `git clone https://github.com/ThomasNotTom/ExtendedSQL.git`
2. SSH: `git clone git@github.com:ThomasNotTom/ExtendedSQL.git`

Go into the created folder

```Bash
cd ExtendedSQL
```

Build with

```Bash
cargo build
```

Run with

```Bash
cargo run {input_file}.esql
```

replacing `{input_file}.esql`, with the file you'd like to open

The complete installation process using SSH looks like this

```Bash
git clone git@github.com:ThomasNotTom/ExtendedSQL.git

cd ExtendedSQL

cargo build
cargo run ./examples/example1.esql
```

## Installing

To install you can either (a) add the binary in your `/usr/bin/` or (b) add it to your path.

### For both

First you need to build with the `--release` flag.

```Bash
cargo build --release
```

#### (a) Adding the binary

Then you need to move the binary to your `/usr/bin/` folder.

```Bash
sudo mv ./target/release/extended_sql /usr/bin/
```

### Adding to your path

Add the output binary to your path

1. For zsh:

```Bash
echo 'export PATH="$(pwd)/target/release/build/:$PATH"' >> ~/.zshrc

source ~/.zshrc
```

2. For bash:

```Bash
echo 'export PATH="$(pwd)/target/release/build/:$PATH"' >> ~/.bashrc

source ~/.bashrc
```

## How is it used?

(syntax subject to change)

```
CREATE SERVER server;
INSERT INTO SERVER server (
    6543, // The port number
)

CREATE TABLE users (
    user_id INT PRIMARY_KEY,
    first_name STRING,
    last_name STRING,
    age INT NOT_NULL,
);

Server ON GET TO "/get_user" WITH QUERY (
    user_id INT,
) RETURN (
    USER (SELECT * FROM users WHERE users.user_id=HEADER.user_id)
);
```

Given a user exists with `user_id` `1234` and a GET request to `/get_user?user_id=1234` on port `6543` would return

```JS
{
    user: {
        user_id: 1234,
        first_name: 'John',
        last_name: 'Doe',
        age: 40
    }
}
```

## What is the current syntax?

```
CREATE DATABASE MyDatabase;

CREATE TABLE MyDatabase.users;
CREATE TABLE MyDatabase.items;
CREATE TABLE MyDatabase.transactions;
```

## Why is this needed?

ESQL is not designed to be a replacement to a backend or database, however is intended to be used as a temporary placeholder for both. This is particularly useful for programmers who want to quickly spin up a backend/database, and have it be configured quickly so other functionality (mainly frontend) can be worked on.
