---
title: "Building a Port Scanner in Rust"
date: 2026-04-03
summary: "Rewriting a Python port scanner in Rust — ownership, async I/O, and why it's an order of magnitude faster."
tags: [rust, networking, tools, security]
---

## Why Rewrite It

The Python version works. It's readable and easy to modify.
But on large port ranges it slows down — Python's GIL limits true concurrency,
and socket operations block the thread that initiated them.
Against a single host across 65,535 ports, the wall time is significant.

The Rust version scans the same range roughly ten times faster, uses less memory,
and opens a clean path to the features the Python version couldn't support:
banner grabbing, source port binding, randomised scan order.

## The Rust Approach

Rust's async model with Tokio lets you fire thousands of connection attempts
concurrently without thousands of threads. Each attempt is a lightweight task
scheduled by the async runtime. You get the concurrency of threads with the
overhead of coroutines.

The core function looks like this:

```rust
async fn probe_port(ip: IpAddr, port: u16, timeout: Duration) -> PortResult {
    let target = SocketAddr::new(ip, port);
    match timeout_duration(timeout, TcpStream::connect(target)).await {
        Ok(Ok(mut stream)) => {
            let banner = grab_banner(&mut stream, timeout).await;
            PortResult { port, state: PortState::Open, service: service_name(port), banner }
        }
        Ok(Err(_)) => PortResult { port, state: PortState::Closed, .. },
        Err(_)     => PortResult { port, state: PortState::Filtered, .. },
    }
}
```

Three outcomes: the connection succeeds (open), it's refused with RST (closed),
or it times out (filtered — a firewall is dropping packets).
The distinction between closed and filtered matters for reconnaissance: a closed port
tells you the host is up and reachable on that address. A filtered port tells you
there's something between you and the host that's silently dropping packets.

## Ownership in Practice

The trickiest part of writing async Rust for the first time is understanding what
the ownership rules require across await points.

A `Future` in Rust might be paused at any await point and resumed on a different
thread. That means any data the Future holds across an await point must be `Send` —
safe to transfer between threads. The compiler enforces this. If you try to hold
a reference across an await point that can't be sent, it's a compile error.

In the port scanner, the target IP address is shared across all the scanning tasks.
In Python you'd just close over it — the reference counting handles the lifetime.
In Rust, you have to be explicit: the tasks each get their own copy of the IP
(`IpAddr` is `Copy`, so it's cloned cheaply). The explicit clone makes the
data-sharing visible in the code.

This is not just boilerplate. It forces you to think about what data is being
shared and how. In a security context, that explicitness matters — you know exactly
what each task owns.

## Concurrency Without Semaphores

The concurrency cap is implemented with `JoinSet`, a Tokio primitive that tracks
a group of spawned tasks. When the set reaches the configured limit, the code
drains one completed result before spawning the next task:

```rust
while set.len() >= cfg.concurrency {
    if let Some(Ok(r)) = set.join_next().await {
        results.push(r);
    }
}
set.spawn(async move { probe_port(ip, port, timeout, source_port).await });
```

This replaces the common pattern of an `Arc<Semaphore>` with a permit acquired
before each task. The `JoinSet` approach has no per-task allocations — no `Arc`
clone, no permit struct — and the back-pressure is inherent: you can't spawn until
one finishes.

## Evasion Features

The scanner includes three features borrowed from Nmap's technique catalog,
implemented for the purpose of understanding how they work:

**Randomised scan order.** Sequential port scanning (1, 2, 3, 4, ...) is a well-known
IDS signature. Shuffling the port list before scanning breaks the sequential pattern.
The Rust implementation is one line: `ports.shuffle(&mut rng)`.

**Delay and jitter.** A configurable sleep between probes, with a random jitter window,
lets you control the scan rate precisely. `--delay-ms 100 --jitter-ms 50` means
each probe waits between 100ms and 150ms. This puts the scan rate well below
thresholds that trigger rate-based alerts.

**Source port binding.** Sending from a well-known source port — 53 (DNS), 80 (HTTP) —
can bypass naive firewall rules that permit traffic from those ports without inspecting
the full connection. The implementation requires `SO_REUSEPORT` on Unix systems so
multiple concurrent tasks can bind to the same source port, using the full 4-tuple
(src_ip, src_port, dst_ip, dst_port) to disambiguate each connection.

These features exist in the scanner because understanding them requires implementing them.
You understand network-layer firewall evasion differently when you've written the code
than when you've read about it.

## Banner Grabbing

After a successful TCP connection, the scanner reads the first 1024 bytes from the
stream with a timeout:

```rust
async fn grab_banner(stream: &mut TcpStream, timeout: Duration) -> Option<String> {
    let mut buf = [0u8; 1024];
    match timeout(timeout, stream.read(&mut buf)).await {
        Ok(Ok(n)) if n > 0 => {
            let s: String = String::from_utf8_lossy(&buf[..n])
                .chars()
                .map(|c| if c.is_control() { ' ' } else { c })
                .collect();
            Some(s.trim().to_string()).filter(|s| !s.is_empty())
        }
        _ => None,
    }
}
```

The control character filter is there because banners sometimes contain escape codes
or binary data. Printing raw binary to the terminal causes display corruption.
Replacing control characters with spaces keeps the output readable.

SSH servers send their version string immediately on connection: `SSH-2.0-OpenSSH_8.9p1`.
SMTP servers say `220 mail.example.com ESMTP`. A banner that includes a version string
is your first cross-reference point against known CVEs for that service.

## Where This Lives Now

The port scanner is now `mg-scan` in GeistScope, the bug bounty toolchain.
It accepts `--engagement` to scope-check targets and write output to the engagement
directory automatically. The standalone binary works as before; the engagement mode
adds the pipeline integration.

The port scanner writeup from the mg-server blog post that introduced this project
was the starting point. `mg-scan` is where that exploration landed.
