# Fake log

Just a toy project to learn more about Rust

It generate fake apache logs 

For the moment it only supports Common Log Format (CLF)

## build

```
cargo build --release
```

## usage

```
./target/release/fake_log --help
fake_log 0.1.0
Generate fake apache logs (CLF Format)

USAGE:
    fake_log [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -n, --number <NUMBER>    [default: 10]
    -V, --version            Print version information

./target/release/fake_log 
141.235.45.148 - - [20/May/2022:16:02:18] "DELETE +0000 /update/635 HTTP/1.0" 500 8772
123.5.11.141 - - [20/May/2022:16:02:18] "DELETE +0000 /create/37 HTTP/1.0" 301 13344
227.187.81.83 - - [20/May/2022:16:02:18] "POST +0000 /delete/194 HTTP/1.0" 301 16633
75.237.173.158 - - [20/May/2022:16:02:18] "PUT +0000 /delete/785 HTTP/1.0" 301 4944
174.56.238.182 - - [20/May/2022:16:02:18] "DELETE +0000 /search/518 HTTP/1.0" 301 11910
93.12.93.252 - - [20/May/2022:16:02:18] "GET +0000 /update/38 HTTP/1.0" 500 3683
103.186.170.224 - - [20/May/2022:16:02:18] "POST +0000 /create/945 HTTP/1.0" 500 11326
88.49.139.77 - - [20/May/2022:16:02:18] "GET +0000 /create/903 HTTP/1.0" 500 13544
221.159.25.29 - - [20/May/2022:16:02:18] "POST +0000 /update/464 HTTP/1.0" 404 1011
113.187.161.109 - - [20/May/2022:16:02:18] "GET +0000 /search/124 HTTP/1.0" 500 15510

```

Create a file with 1 000 000 entries

```
./target/release/fake_log -n 1000000 > logs.txt
```

### TODO

- Add more formats
