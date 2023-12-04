# knock-rs

Simple port knocking client written in Rust.
This program works fine with knockd service.

## Usage

```
$ ./knock --help
Usage: knock [OPTIONS] --remote <REMOTE> --ports <PORTS>...

Options:
  -r, --remote <REMOTE>
  -p, --ports <PORTS>...
  -d, --delay <DELAY>     [default: 0]
  -h, --help              Print help
  -V, --version           Print version
```

Here's a small example sending the sequence `1000/tcp 2000/udp 3000/tcp` to `localhost` :

```
$ ./knock -r localhost -p 1000/tcp 2000/udp 3000/tcp
```

Specifying protocol is optional. `3000` will default to `3000/tcp`.
