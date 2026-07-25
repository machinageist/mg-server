# Writing ideas pulled from digital-liberty-website

Staged for the long-form interview process, not drafted. Each idea below is
lab-independent — it doesn't need Jeff to have completed a specific hands-on
build first, unlike the lab-linked writeups tracked in
`docs/plans/2026-07-12-consulting-to-portfolio-labs.md` (those stay gated on
their lab actually happening). These are judgment/explainer pieces, sourced
from real copy already written for the Private Home Systems consulting site
at `~/digital-liberty-website`, reframed for a personal-education voice
instead of a sales voice.

## The router is not the whole network

- Source: `src/pages/work.astro`, entry 1 ("The router is not the whole
  network").
- Angle: most home networks are a pile of half-decisions (ISP router in a bad
  spot, mesh kit bolted on later, IoT and guest devices with no plan). The
  fix isn't more hardware, it's legibility — what's trusted, what should be
  isolated, what needs replacing. Pairs naturally with the wiki's existing
  OSI/appliances pages but from a "your actual house" angle instead of an
  exam-objective angle.

## Private by default: remote access without publishing your house

- Source: `src/pages/work.astro`, entry 2 ("Remote access without publishing
  your house").
- Angle: a lot of home-tech advice still drifts toward exposing services to
  the internet because it's convenient in the moment. Make the case for
  boring private access (Tailscale/WireGuard, careful device approval, clear
  recovery notes) over open port forwards.

## "Will this make me anonymous online?" — no, and here's what DNS filtering actually does

- Source: `src/pages/services.astro`, FAQ entry.
- Angle: an honest myth-busting piece. Ad/tracker DNS filtering (Pi-hole/
  AdGuard) reduces noise, it doesn't grant anonymity — worth explaining what
  it actually blocks, what it doesn't, and why "practical risk reduction" and
  "anonymity" are different goals. Same anti-overclaim register as the rest
  of this site.

## Less clever, more durable

- Source: `src/pages/work.astro`, entry 3 ("Local media, backups, and
  household notes that survive real life").
- Angle: a system is only good if people keep using it after the novelty
  wears off — restore tests, plain folder names, notes written for tired
  future-you, over home-lab complexity for its own sake. A documentation-
  first philosophy piece, not a how-to.
