## About
A simple rust disposable email domain checker based on [https://github.com/disposable-email-domains/disposable-email-domains](https://github.com/disposable-email-domains/disposable-email-domains)


## Setup
```bash
# need to run only once, or run again if the list needs to be updated
# file will be saved to data/disposable_email_blocklist.conf
./download.sh
cargo build --release
```


## To run
```bash
cargo run --release 1-tm.com
# should print hit

cargo run --release gmail.com
# should print miss
```

