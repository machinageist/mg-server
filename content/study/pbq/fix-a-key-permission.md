---
title: "An SSH key the client refuses to use"
situation: >
  You have copied a private key onto a new machine and SSH will not touch it.
  The error mentions permissions. Work out what is wrong and fix it, then make
  sure the same mistake does not repeat for files you create later.
steps:
  - prompt: >
      First, look at the key and its directory so you can see the modes rather
      than guess at them. What do you run?
    accept:
      - "ls -l ~/.ssh"
      - "ls -la ~/.ssh"
    explanation: >
      `ls -l` prints the mode as ten characters — one type character and three
      groups of three. You need to see the key itself and the directory holding
      it, because SSH checks both. Reading the modes before changing anything is
      the difference between a fix and a guess.
    learn_slug: "linux-permissions"
    learn_anchor: "reading-ls-l"

  - prompt: >
      The listing shows the key as `-rw-r--r--`. SSH refuses a private key that
      any other account can read. Set it so only its owner can read and write
      it.
    given: |
      -rw-r--r--  1 you you 411 Aug 14 09:12 id_ed25519
      -rw-r--r--  1 you you  98 Aug 14 09:12 id_ed25519.pub
    accept:
      - "chmod 600 ~/.ssh/id_ed25519"
      - "chmod u=rw,go= ~/.ssh/id_ed25519"
      - "chmod go-rwx ~/.ssh/id_ed25519"
    explanation: >
      `600` is read and write for the owner and nothing for anyone else. The
      public key can stay world-readable — that is what public means — so only
      the private key needs changing. Numeric notation sets all nine bits at
      once, which is what you want when you know the end state you are after.
    learn_slug: "linux-permissions"
    learn_anchor: "numeric-notation"

  - prompt: >
      The directory is `drwxr-xr-x`, which lets others list your key names.
      Restrict it to its owner.
    accept:
      - "chmod 700 ~/.ssh"
      - "chmod go-rwx ~/.ssh"
      - "chmod u=rwx,go= ~/.ssh"
    explanation: >
      On a directory the three bits mean something different: read lists the
      names inside, write creates and deletes entries, and execute lets you
      traverse into it. `700` gives the owner all three and everyone else
      nothing, so no other account can even enumerate what keys you have.
    learn_slug: "linux-permissions"
    learn_anchor: "permissions-on-directories"

  - prompt: >
      Finally, make new files private by default in this shell, so the next key
      you copy does not land world-readable in the first place.
    accept:
      - "umask 077"
    explanation: >
      The umask subtracts from a base of 666 for files and 777 for directories,
      so `077` yields 600 and 700 — exactly the modes you just set by hand. It
      applies to the shell that runs it, so making it stick means putting it in
      a startup file. Note that new files never get execute permission from the
      umask no matter what you set, which is a deliberate safety property.
    learn_slug: "linux-permissions"
    learn_anchor: "default-permissions-and-umask"
---

A performance-based scenario. Data lives in the frontmatter above; this body is
intentionally empty.
