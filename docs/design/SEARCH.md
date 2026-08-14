# Search

**Route:** `GET /search?q=`
**Shipped:** 2026-08-14
**Spec:** `gauntlet-output/specs/C1-search.md`

Server-rendered search over the site's published writing and education wiki. No
JavaScript, no index file, no search dependency, no database.

## What is searchable

The corpus is **routable content only**, built fresh on each request:

| Source | Corpus rule | Servable at |
|---|---|---|
| `content/posts/*.md` | every file | `/blog/:slug` |
| `content/pages/*.md` | only slugs in `SIDEBAR` | `/learn/:slug` |

Two consequences follow from defining the corpus this way rather than by
globbing a directory:

- **A result can never 404.** The post corpus is exactly what `/blog/:slug`
  serves; the page corpus is exactly the allowlist `/learn/:slug` checks.
- **A draft can never leak.** `content/drafts/` is read by no route and by no
  branch of `SearchIndex::build`. Pinned by
  `search::tests::the_real_corpus_is_searchable_and_excludes_drafts`.

Each document contributes four searchable fields: title, summary, tags, and the
**full body text**. Body search is what separates this from a menu of page
titles — `trilateration` appears in one paragraph of one page and is findable.

## How matching works

Case-insensitive substring, **AND across terms**: every term must appear
somewhere in a document for it to be a candidate, so a second word narrows the
result set. No stemming, no fuzzy matching, no ranking library. At a corpus of
this size that is the right altitude — predictable, dependency-free, and
testable by hand.

Scoring takes the best field weight each term hits, plus a capped bonus for
repeated body occurrences:

| Field | Weight |
|---|---:|
| Title | 6 |
| Tags | 4 |
| Summary | 3 |
| Body | 1 (+1 per extra occurrence, capped at +5) |

The cap exists so a long page mentioning a term forty times cannot outrank a
short page that is actually *about* it. Ties break by date descending, then URL
ascending — the second key only so ordering is stable between requests.

## The snippet escaping contract

**This is the security-critical part.** `snippet_html` is the one value the
templates render with `|safe`, so `highlight_snippet` must guarantee that no
byte of content or query can become live markup.

The invariant: **escape every run first, then introduce `<mark>`.**

1. Choose a whole-word window around the first match.
2. Split the window into alternating plain and matched runs, on the *original*
   bytes.
3. HTML-escape each run independently.
4. Join with `<mark>` … `</mark>` around the escaped matched runs.

Both other orderings are wrong: escaping after marking would double-escape the
marks, and matching against already-escaped text would miscount offsets. Pinned
by `search::tests::snippets_escape_markup_before_they_highlight`, which searches
for `script` inside a body containing a real `<script>` tag and asserts the
output carries `&lt;` and no live tag.

## Decisions worth not relitigating

**Per-request corpus, not a startup index.** The engine re-reads and re-parses
the corpus on every `/search` request. Building it once into `AppState` would
save a few dozen small file reads, but it reintroduces staleness — the index
would lag disk until a restart. On a site whose value is currency, fresh-from-
disk is the right default, and it matches how every other route already reads
content. Revisit only if the corpus reaches the low hundreds of documents.

**No query logging, no analytics.** `q` is never written to a log, a counter, or
a cookie. Knowing which searches return nothing would genuinely help decide what
to write next, and it is still not worth building a record of what readers look
for. If it is ever wanted, aggregate counts only — never raw queries.

**No pagination.** `MAX_RESULTS` caps the list at 20, which is comfortably more
than the corpus. Revisit with the startup index.

**No JavaScript.** `criteria.md` auto-fail rule 3. The form is a plain GET with a
real `<input>` and `<button>`; results are HTML; every result is a real URL that
can be bookmarked and shared. Pinned by
`handlers::search::tests::search_needs_no_javascript`, which strips every
`<script>` from the response and asserts the form and results survive.

## Accessibility notes

- The form is a `search` landmark (`role="search"`), and the input has a
  visually hidden `<label>` — never placeholder-as-label.
- Results are an `<ol>` because rank is meaningful. Result titles are links
  inside list items, **not** headings: a results list is a list, and headings
  there would compete with the page outline.
- Result kind is the word "Writing" or "Learn", never a colour swatch.
- `<mark>` carries weight and an accent underline in addition to its tint, so a
  match stays visible for a reader who overrides background colours.
- No `aria-live` anywhere — there is no dynamic update to announce. That is the
  no-JS design paying off rather than an omission.

## Where the pieces live

| Path | Role |
|---|---|
| `src/search.rs` | Corpus, ranking, snippet and escaping |
| `src/handlers/search.rs` | `SearchQuery`, `SearchTemplate`, `search_view`, handler |
| `templates/search.html` | Form, count line, results, both empty states |
| `static/css/style.css` | `.vh`, `.search-*`, `mark` |
| `src/router.rs` | Route registration |

`search_view()` is split from the handler so tests can render the template
directly — a handler returning `impl IntoResponse` hides it behind an opaque
type.

## If you change this

- Adding a searchable field means updating the weight table above **and**
  `score()`. There is no config; the weights are named constants.
- Adding a content directory means deciding, explicitly, whether it is routable.
  If it is not, it must not be in the corpus.
- Touching `highlight_snippet` means re-reading the escaping contract above
  first. The test will catch a live tag, but the reasoning is what keeps the
  function correct for cases the test does not enumerate.
