# Site voice

How public copy on machinageist.dev should read. Taken from Jeff's own edits, mainly
`618398f` ("revise networking posts for a more direct voice"), which rewrote seven
learn pages and the network migration post by hand. When a sentence could go either
way, compare it with that diff.

Applies to everything a visitor can read: `content/`, `templates/`, the portfolio
entries in `src/models/project.rs`, and `README.md`.

## What it sounds like

- Plain words. "Jobs", not "functional roles". "Start", not "initiate". "Picks", not
  "selects". "A provider's", not "someone else's".
- Short declarative sentences, one idea each. If a sentence needs a dash or a
  semicolon to hold together, it is two sentences.
- Concrete nouns: the tool, the command, the file, the port number.
- Few contractions in body copy. The sentences Jeff wrote in 618398f use "did not"
  and "is not", even though his notes contract freely. Short UI labels ("What's
  here") are fine as they are.
- First person for Jeff's own work ("I moved", "I checked", "I am still learning").
  "You" in practice sections. Teaching prose is otherwise impersonal.
- Limits stated flatly, once, where they apply. "I have not tested this on 6 GHz."
  Not a paragraph about honesty.

## What it avoids

These are the habits the 618398f edit removed, and the ones most likely to creep
back in with AI-assisted drafts.

- Em dashes in running prose. An em dash is fine as a list separator
  (`[Page](/learn/x) — why you would follow it`, `**Term** — definition`) and
  nowhere else.
- Semicolons in prose, mostly. Jeff kept one in the migration post for a tight
  two-part contrast ("The status output showed what the service believed; the
  capture showed whether packets were actually crossing the network."). One like
  that is fine. A list or paragraph held together by them is not. Code is exempt.
- Bold for emphasis or to introduce a term mid-sentence. Bold is fine as the label
  at the start of a definition-style list item.
- Stock intensifiers and filler: actually, genuinely, honestly, exactly, quietly,
  deliberately, simply, worth noting, worth knowing, it is important to, the real
  question, the whole point.
- "Rather than" as a reflex. Use it when two options are being compared. Otherwise
  "instead of" or a second sentence usually reads better, and often the clause can
  go.
- "Not X, but Y" reframes and "X is not Y. It is Z." setups used for effect. Keep them
  only when they correct a common mistake.
- A closing line that restates the paragraph as an aphorism ("the generation is a
  ceiling, not a promise"). End on the last fact.
- Throat-clearing: "I want to be honest about", "Here is the part", "It turns out",
  "This is where".
- Dramatic framing: "not a victory lap", "wearing two hats", "I flailed", "knocked
  over".
- Treating a page, table, or tool as if it had intentions ("the page argues", "the
  design wants").
- Curly quotes. Use straight quotes.

## What stays fixed

Voice edits must not change these. Tests hold most of them.

- Heading text on learn pages. Glossary entries and study questions link to heading
  anchors, and a renamed heading breaks them.
- The source line on learn pages ("This page was edited from my own study notes...").
- The project document sections, especially the disclosure heading and its three
  registers.
- The claim rules in `IMPROVEMENT_PLAN.md` and `docs/agent-context/README.md` §4. A
  plainer sentence still has to be a true one.
