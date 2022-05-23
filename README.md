# YamSH

A shell that runs on Yamini VM.

# Steps to run

1. Build a release version of YamSH:

```bash
user@programmer~:$ cargo build --release
```

2.1. If you are running macOS, then copy to:

```bash
user@programmer~:$ cp target/release/yamsh /usr/local/bin/
```

2.2. If you are running Linux, then copy to:

```bash
user@programmer~:$ cp target/release/yamsh /usr/bin/
```

2.3. If you are running windows, then I am sorry my friend, but this shell is not for you :)

3. Make this your default shell, here are some links to do that:

    a. macOS: https://support.apple.com/en-gb/guide/terminal/trml113/mac
    b. Linux: Depends on your distribution, please check your distribution's documentation