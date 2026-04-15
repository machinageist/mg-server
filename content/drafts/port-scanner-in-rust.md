---
title: "Building a Port Scanner in Rust"
date: 2026-04-03
summary: "Rewriting a Pyhton port scanner in Rust - ownership, async I/O, and why it 10x faster."
tags: [rust, networking, tools, security]
---
## Why Rewrite It?

The Python version works. It's readable and easy to modify.
But on large port ranges it's slow - Python GIL limits true concurrency, and the socket operations block.

## The Rust Approach

Rust's async model with Tokio lets us fire thousands of connection 
attempts concurrently without thousands of threads. 
Each attempt is a lightweight task scheduled by the runtime.

## Ownership in Practice

The trickiest part of the rewrite was understanding how to share the target
hostname across async tasks. In Python you'd just close over it. 
In rust, the ownership rules require you t clone it explicitly - which forces you to think about what data is being shared and why.
