---
title: "mg-social"
date: 2026-05-18
summary: "GitHub org member enumeration and email pattern generation for a target org, with a LinkedIn dork printed for manual follow-up."
tags: [geistscope, cli, osint]
---

## Purpose

Enumerate GitHub org members and generate plausible email addresses from their
names and the target domain. Useful for building a contact list for phishing
simulations or password spray prep.

## Output

- `recon/social-<org>.json` — GitHub usernames, display names, generated email
  candidates per name pattern, and the LinkedIn dork URL.

## CLI

```bash
mg-social acme-bounty --org acme-corp
mg-social acme-bounty --org acme-corp --domain acme.example.com --token $GH_TOKEN
```

## Notes

- GitHub token is optional but recommended. Without it, the GitHub API rate
  limit is 60 requests/hr, which limits member list depth for large orgs.
- Email patterns generated per member: `first.last`, `flast`, `firstl`,
  `first`, combined with the target domain.
- The `--domain` flag overrides the domain used for email generation; defaults
  to the root domain from the engagement scope.
- LinkedIn dork (`site:linkedin.com/in "acme"`) is printed to stdout only, not
  stored, since LinkedIn blocks automated access.
- Pair output with [mg-breach](/wiki/mg-breach) to check generated emails
  against known breach data.
