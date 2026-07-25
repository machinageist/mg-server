---
title: "GeistScope: A Retrospective on an Over-Scoped AI-Assisted Project"
date: 2026-06-15
summary: "GeistScope was an early experiment in building a large Rust security toolkit with heavy AI assistance. It over-scoped badly. This is the honest account of what it was, what was real versus aspirational, what I learned, and why I pulled it back and pivoted."
tags: [rust, retrospective, ai-assisted, scope-control, lessons-learned]
---

This post replaces roughly two dozen GeistScope devlog entries that used to live
here. I took them down on purpose and folded the honest parts into this single
retrospective. The devlogs read like a product launch for something that was
never a product, and keeping them up would have been the opposite of the point.

## What GeistScope was

GeistScope was a Rust workspace of security-testing command-line tools —
recon, crawling, fingerprinting, a pile of web-vulnerability scanners, some
cloud and auth modules, a terminal dashboard, and an "AI harness" that let a
language model dispatch the tools through a JSON API. I built it fast, with a
lot of AI-assisted coding, over a few weeks in the spring of 2026.

The devlogs described it as a "bug bounty toolchain" and an "AI-native"
platform. That framing was aspirational marketing I wrote for a project that was
really a personal learning sandbox. That gap is the whole lesson.

## What was real versus aspirational

Being honest about the split matters more than the code did.

**Real:**

- A working Rust workspace that compiled and ran. Writing dozens of small async
  crates taught me a lot about Rust's ownership model, `tokio`, error handling,
  and workspace structure. That learning was genuine.
- A handful of genuinely passive, report-first pieces — engagement/scope
  management, HTTP fingerprinting, a passive header/CSP checker, artifact and
  Markdown/JSON reporting. These are the parts I can still explain and defend.
- The port scanner I wrote earlier (a separate blog post) got pulled in as one
  of the recon tools.

**Aspirational or over-claimed:**

- "AI-native exploitation," "exploit-chain engine," and "bug-bounty automation
  platform" were labels, not demonstrated capabilities. Much of the active,
  credentialed, cloud, and post-exploitation surface was scaffolding I could not
  operate safely, explain line by line, or prove against a real scoped target.
- Breadth was mistaken for depth. Sixty-plus crates is not sixty-plus working
  tools; it is a lot of surface area I could not stand behind.

## What went wrong

The failure mode was **scope, not syntax**. AI assistance made it very cheap to
generate another crate, another scanner, another "module." Cheap generation
removed the natural friction that normally forces you to ask whether you should
build the next thing. So I kept building outward instead of proving anything
inward. The result was a large repository that looked impressive in a file tree
and could not be defended in a five-minute conversation.

There was also a claims problem. The README and the devlogs described
professional red-team / offensive tooling. I am not a professional red-teamer,
and presenting owned-scope learning code as an offensive platform is exactly the
kind of overclaim that falls apart under one good interview question.

## What I did about it

I put GeistScope into an explicit pruning-and-ownership phase and pulled it off
my résumé:

- Wrote a mechanical inventory of every crate and harness endpoint and labeled
  each one: keep, keep-later, needs-ownership-walkthrough, archive, or unsafe.
- Rewrote the README down to an honest posture: authorized/local scope only,
  passive and report-first workflows first, active and destructive surfaces
  archived or lab-gated until they have a walkthrough and safe local proof.
- Froze public claims. GeistScope is now represented by this learning retrospective,
  not presented as security work I did professionally.

The safe core that survives is small: scope/workspace management, passive
fingerprint and header checks, and evidence reporting. That small core is worth
more to me than the sprawling version, because I can actually explain it.

## What I learned

- **Scope discipline is the skill.** With AI assistance, generating code is no
  longer the bottleneck; deciding what *not* to build is. A tool you can operate,
  explain, and defend beats ten you cannot.
- **Claims must trail evidence.** Say the smaller, true thing. "I wrote passive
  HTTP fingerprinting and reporting tools in Rust for local labs" holds up.
  "I built an AI-native offensive platform" does not.
- **Self-awareness is the deliverable.** The value of this project now is not the
  code — it is being able to look at my own over-scoped work and prune it
  honestly.

## Where my focus went instead

I redirected the effort into the work this site is actually about: infrastructure
work anchored in a Proxmox homelab, real networking and Linux operations evidence,
and a small defensive-security section built to grow — each piece tied to a homelab
project with captured evidence. That is a narrower story, and a true one.

The individual GeistScope tool pages are no longer published. Git preserves the full
experiment, while this retrospective preserves the public record without presenting
unfinished scaffolding as useful documentation.

A tool can earn a public page in the future, but the gate is deliberately higher: it
must fit and work within the complete GeistScope pipeline, be operable through both a
human interface and a stable machine interface for AI agents, and have sanitized
evidence from an actual authorized engagement. Until then, preserving an idea in the
repository is not the same as publishing it as a usable tool.
