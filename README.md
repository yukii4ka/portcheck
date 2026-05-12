# portcheck

tired of typing `nc -zv host port` every time. this does that but nicer.

## usage

```
portcheck <host> <port> [port2] [port3] ...
```

examples:
```
portcheck google.com 80 443
portcheck 192.168.1.1 22 80 443 8080
```

## output

```
google.com:80     open    (14ms)
google.com:443    open    (11ms)
192.168.1.1:22   open    (3ms)
192.168.1.1:8080  closed
```

default timeout is 3s. change with `--timeout <ms>`:
```
portcheck some-host.local 80 --timeout 500
```

## build

```
cargo build --release
./target/release/portcheck
```

needs rust 1.70+

## why

i use this a lot when debugging vpn setups — need to quickly check which ports are actually reachable through a tunnel. nmap is overkill for that.
