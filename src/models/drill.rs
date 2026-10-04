// Author:      machinageist
// Date:        2026-10
// Description: Generated practice problems for /study/drills. A set is built
//              from a seed, so the same URL always shows the same problems and
//              the server keeps nothing between showing a set and grading it.
//              Holds the subnetting problems: the generator, the solver, the
//              answer matching, and the working shown with each result.
//
// Notes:       The generator is SplitMix64. It is not for security. It only has
//              to give the same numbers on every build, because a shared link
//              is a promise about what the reader will see. Changing it, or the
//              order values are drawn in, changes every shared set, and a test
//              pins the output for that reason.
//
//              Addresses come from RFC 1918 and RFC 5737 space only, so no
//              problem names a real network.
//
//              Every problem kind cites the /learn heading that teaches it, and
//              a test resolves each citation the way the question bank does.

use std::net::Ipv4Addr;

// -----------------------------------------------------------------------
// Limits
// -----------------------------------------------------------------------

// Problems in one set
pub const SET_SIZE: usize = 10;

// Prefix range for network and host-count problems. /31 and /32 are left out
// because their host rules are special cases the subnetting page does not teach
const MIN_PREFIX: u8 = 8;
const MAX_PREFIX: u8 = 30;

// Shortest prefix a sizing problem asks for, which keeps host counts readable
const SIZING_MIN_PREFIX: u8 = 20;

// Bits in an IPv4 address
const ADDRESS_BITS: u8 = 32;

// Page every subnetting problem cites
const LEARN_SLUG: &str = "subnetting";

// Address blocks problems are drawn from, as (base, prefix)
const POOLS: &[(u32, u8)] = &[
    (u32::from_be_bytes([10, 0, 0, 0]), 8),
    (u32::from_be_bytes([172, 16, 0, 0]), 12),
    (u32::from_be_bytes([192, 168, 0, 0]), 16),
    (u32::from_be_bytes([192, 0, 2, 0]), 24),
    (u32::from_be_bytes([198, 51, 100, 0]), 24),
    (u32::from_be_bytes([203, 0, 113, 0]), 24),
];

// Words for an octet's position, used in the working
const OCTET_ORDINALS: [&str; 4] = ["first", "second", "third", "fourth"];

// -----------------------------------------------------------------------
// Seeded generator
// -----------------------------------------------------------------------

// SplitMix64 state
struct Rng(u64);

impl Rng {
    // Advance the state and return the next value
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    // Draw a value in 0..limit
    fn below(&mut self, limit: u64) -> u64 {
        self.next() % limit
    }

    // Draw a prefix length in low..=high
    fn prefix(&mut self, low: u8, high: u8) -> u8 {
        low + self.below(u64::from(high - low) + 1) as u8
    }
}

// -----------------------------------------------------------------------
// Subnet arithmetic
// -----------------------------------------------------------------------

// Build the mask for a prefix length
fn mask(prefix: u8) -> u32 {
    if prefix == 0 {
        0
    } else {
        u32::MAX << (ADDRESS_BITS - prefix)
    }
}

// Find the network address a host address belongs to
fn network_of(address: u32, prefix: u8) -> u32 {
    address & mask(prefix)
}

// Find the broadcast address of a host address's subnet
fn broadcast_of(address: u32, prefix: u8) -> u32 {
    address | !mask(prefix)
}

// Count the usable host addresses in a subnet of this prefix
fn usable_hosts(prefix: u8) -> u64 {
    (1u64 << (ADDRESS_BITS - prefix)) - 2
}

// Find the longest prefix whose subnet holds this many hosts
fn prefix_for_hosts(hosts: u64) -> u8 {
    (MIN_PREFIX..=MAX_PREFIX)
        .rev()
        .find(|prefix| usable_hosts(*prefix) >= hosts)
        .unwrap_or(MIN_PREFIX)
}

// Write an address in dotted decimal
fn dotted(address: u32) -> String {
    Ipv4Addr::from(address).to_string()
}

// -----------------------------------------------------------------------
// Problems
// -----------------------------------------------------------------------

// The kinds of subnetting problem, and the `type` value that selects each
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProblemKind {
    Network,
    Hosts,
    Sizing,
}

impl ProblemKind {
    pub const ALL: [ProblemKind; 3] = [
        ProblemKind::Network,
        ProblemKind::Hosts,
        ProblemKind::Sizing,
    ];

    // The value used in the URL
    pub fn key(self) -> &'static str {
        match self {
            ProblemKind::Network => "network",
            ProblemKind::Hosts => "hosts",
            ProblemKind::Sizing => "vlsm",
        }
    }

    // The name shown on the page
    pub fn label(self) -> &'static str {
        match self {
            ProblemKind::Network => "Network and host range",
            ProblemKind::Hosts => "Hosts per prefix",
            ProblemKind::Sizing => "Prefix for a host count",
        }
    }

    // Look up a kind by its URL value
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.key() == key)
    }

    // The heading on the subnetting page that teaches this kind
    fn learn_anchor(self) -> &'static str {
        match self {
            ProblemKind::Network => "subnet-masks",
            ProblemKind::Hosts => "counting-hosts-and-networks",
            ProblemKind::Sizing => "variable-length-subnet-masking",
        }
    }

    // The deep link a reader follows to see the method explained
    pub fn learn_href(self) -> String {
        format!("/learn/{LEARN_SLUG}#{}", self.learn_anchor())
    }
}

// One generated problem
#[derive(Debug, Clone, PartialEq)]
pub enum Problem {
    // A host address and prefix. Find its network, broadcast, and host range
    Network { address: u32, prefix: u8 },
    // A prefix. Count its usable hosts
    Hosts { prefix: u8 },
    // A host count. Find the longest prefix that holds it
    Sizing { hosts: u64 },
}

// What a correct answer to one field is
#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    Address(u32),
    Number(u64),
    Prefix(u8),
}

impl Answer {
    // Write the answer the way the result page shows it
    pub fn display(&self) -> String {
        match self {
            Answer::Address(address) => dotted(*address),
            Answer::Number(number) => number.to_string(),
            Answer::Prefix(prefix) => format!("/{prefix}"),
        }
    }

    // Decide whether a typed answer is this answer
    //
    // Compared as values, not text. Padded octets, a prefix with or without
    // its slash, and thousands separators in a count all still match. Anything
    // that does not parse is wrong, not an error.
    pub fn accepts(&self, typed: &str) -> bool {
        let typed = typed.trim();
        match self {
            Answer::Address(address) => parse_address(typed) == Some(*address),
            Answer::Number(number) => typed.replace(',', "").parse::<u64>().ok() == Some(*number),
            Answer::Prefix(prefix) => {
                typed.trim_start_matches('/').parse::<u8>().ok() == Some(*prefix)
            }
        }
    }
}

// Read a dotted-decimal address, allowing padded octets such as 010
fn parse_address(typed: &str) -> Option<u32> {
    let octets: Vec<u8> = typed
        .split('.')
        .map(|octet| octet.trim().parse::<u8>().ok())
        .collect::<Option<_>>()?;
    let octets: [u8; 4] = octets.try_into().ok()?;
    Some(u32::from_be_bytes(octets))
}

// One input a problem asks for
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    // Stable name, used in the form field name
    pub key: &'static str,
    pub label: &'static str,
    pub answer: Answer,
}

impl Problem {
    pub fn kind(&self) -> ProblemKind {
        match self {
            Problem::Network { .. } => ProblemKind::Network,
            Problem::Hosts { .. } => ProblemKind::Hosts,
            Problem::Sizing { .. } => ProblemKind::Sizing,
        }
    }

    // Word the problem for the reader
    pub fn prompt(&self) -> String {
        match self {
            Problem::Network { address, prefix } => format!(
                "A host has the address {}/{prefix}. Find its network address, its broadcast \
                 address, and the first and last usable host addresses.",
                dotted(*address)
            ),
            Problem::Hosts { prefix } => {
                format!("How many usable host addresses does a /{prefix} subnet have?")
            }
            Problem::Sizing { hosts } => format!(
                "A subnet needs room for {hosts} hosts. What is the longest prefix that fits them?"
            ),
        }
    }

    // List the inputs this problem asks for, with their answers
    pub fn fields(&self) -> Vec<Field> {
        match self {
            Problem::Network { address, prefix } => {
                let network = network_of(*address, *prefix);
                let broadcast = broadcast_of(*address, *prefix);
                vec![
                    Field {
                        key: "network",
                        label: "Network",
                        answer: Answer::Address(network),
                    },
                    Field {
                        key: "broadcast",
                        label: "Broadcast",
                        answer: Answer::Address(broadcast),
                    },
                    Field {
                        key: "first",
                        label: "First host",
                        answer: Answer::Address(network + 1),
                    },
                    Field {
                        key: "last",
                        label: "Last host",
                        answer: Answer::Address(broadcast - 1),
                    },
                ]
            }
            Problem::Hosts { prefix } => vec![Field {
                key: "hosts",
                label: "Usable hosts",
                answer: Answer::Number(usable_hosts(*prefix)),
            }],
            Problem::Sizing { hosts } => vec![Field {
                key: "prefix",
                label: "Prefix",
                answer: Answer::Prefix(prefix_for_hosts(*hosts)),
            }],
        }
    }

    // Show the working for this problem, with its own numbers filled in
    pub fn working(&self) -> String {
        match self {
            Problem::Network { address, prefix } => network_working(*address, *prefix),
            Problem::Hosts { prefix } => {
                let host_bits = ADDRESS_BITS - prefix;
                format!(
                    "A /{prefix} leaves 32 - {prefix} = {host_bits} host bits, which is \
                     2^{host_bits} = {} addresses. Take away the network address and the \
                     broadcast address and {} are usable.",
                    1u64 << host_bits,
                    usable_hosts(*prefix)
                )
            }
            Problem::Sizing { hosts } => {
                let prefix = prefix_for_hosts(*hosts);
                let host_bits = ADDRESS_BITS - prefix;
                format!(
                    "{hosts} hosts need {} addresses once the network address and the \
                     broadcast address are counted. The smallest power of two that holds \
                     that many is 2^{host_bits} = {}, so the subnet needs {host_bits} host \
                     bits. 32 - {host_bits} = {prefix}, and a /{prefix} holds {} hosts.",
                    hosts + 2,
                    1u64 << host_bits,
                    usable_hosts(prefix)
                )
            }
        }
    }
}

// Show the block-size working for a network problem
fn network_working(address: u32, prefix: u8) -> String {
    let network = network_of(address, prefix);
    let broadcast = broadcast_of(address, prefix);
    let result = format!(
        "Network {}, broadcast {}, hosts {} to {}.",
        dotted(network),
        dotted(broadcast),
        dotted(network + 1),
        dotted(broadcast - 1)
    );
    let mask_text = dotted(mask(prefix));

    // A mask that ends on an octet boundary needs no block arithmetic
    if prefix.is_multiple_of(8) {
        let whole = match prefix / 8 {
            1 => "The first octet is the network",
            2 => "The first two octets are the network",
            _ => "The first three octets are the network",
        };
        return format!(
            "A /{prefix} mask is {mask_text}, which ends on an octet boundary. {whole} and \
             the rest are host bits. {result}"
        );
    }

    let index = usize::from(prefix / 8);
    let ordinal = OCTET_ORDINALS[index];
    let mask_octet = u32::from(mask(prefix).to_be_bytes()[index]);
    let block = 256 - mask_octet;
    let value = u32::from(address.to_be_bytes()[index]);
    let start = value - value % block;
    format!(
        "A /{prefix} mask is {mask_text}, so the boundary is in the {ordinal} octet. Block \
         size is 256 - {mask_octet} = {block}. The {ordinal} octet of the address is {value}, \
         which is in the block that starts at {start} and ends before {}. {result}",
        start + block
    )
}

// -----------------------------------------------------------------------
// Sets
// -----------------------------------------------------------------------

// Build one problem of a given kind
fn generate(kind: ProblemKind, rng: &mut Rng) -> Problem {
    match kind {
        ProblemKind::Network => {
            let prefix = rng.prefix(MIN_PREFIX, MAX_PREFIX);
            // Only pools at least as large as the subnet, so it stays inside one
            let eligible: Vec<&(u32, u8)> = POOLS
                .iter()
                .filter(|(_, pool_prefix)| *pool_prefix <= prefix)
                .collect();
            let (base, pool_prefix) = *eligible[rng.below(eligible.len() as u64) as usize];
            let pool_size = 1u64 << (ADDRESS_BITS - pool_prefix);
            let network = network_of(base + rng.below(pool_size) as u32, prefix);
            // A usable host, never the network or broadcast address itself
            let host = 1 + rng.below(usable_hosts(prefix)) as u32;
            Problem::Network {
                address: network + host,
                prefix,
            }
        }
        ProblemKind::Hosts => Problem::Hosts {
            prefix: rng.prefix(MIN_PREFIX, MAX_PREFIX),
        },
        ProblemKind::Sizing => {
            // Pick the answer first, then a host count only that prefix fits
            let prefix = rng.prefix(SIZING_MIN_PREFIX, MAX_PREFIX);
            let low = if prefix == MAX_PREFIX {
                1
            } else {
                usable_hosts(prefix + 1) + 1
            };
            let span = usable_hosts(prefix) - low + 1;
            Problem::Sizing {
                hosts: low + rng.below(span),
            }
        }
    }
}

// Build the set a seed stands for. With no kind given, the kinds take turns
pub fn subnetting_set(seed: u64, kind: Option<ProblemKind>) -> Vec<Problem> {
    let mut rng = Rng(seed);
    (0..SET_SIZE)
        .map(|index| {
            let kind = kind.unwrap_or(ProblemKind::ALL[index % ProblemKind::ALL.len()]);
            generate(kind, &mut rng)
        })
        .collect()
}

// -----------------------------------------------------------------------
// Grading
// -----------------------------------------------------------------------

// One field of a problem, with what the reader typed into it
pub struct GradedField {
    pub field: Field,
    pub typed: String,
}

impl GradedField {
    pub fn correct(&self) -> bool {
        self.field.answer.accepts(&self.typed)
    }

    pub fn unanswered(&self) -> bool {
        self.typed.trim().is_empty()
    }
}

// One graded problem. It is correct only when every field is, and skipped
// only when every field was left blank
pub struct GradedProblem {
    pub problem: Problem,
    pub fields: Vec<GradedField>,
}

impl GradedProblem {
    // Pair a problem's fields with typed answers, looked up by field key
    pub fn new(problem: Problem, typed: impl Fn(&str) -> String) -> Self {
        let fields = problem
            .fields()
            .into_iter()
            .map(|field| GradedField {
                typed: typed(field.key),
                field,
            })
            .collect();
        GradedProblem { problem, fields }
    }

    pub fn correct(&self) -> bool {
        self.fields.iter().all(GradedField::correct)
    }

    pub fn unanswered(&self) -> bool {
        self.fields.iter().all(GradedField::unanswered)
    }

    pub fn verdict(&self) -> &'static str {
        if self.unanswered() {
            "skipped"
        } else if self.correct() {
            "correct"
        } else {
            "incorrect"
        }
    }

    pub fn verdict_class(&self) -> &'static str {
        if self.unanswered() {
            "is-skipped"
        } else if self.correct() {
            "is-correct"
        } else {
            "is-wrong"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Whether an address sits inside one of the allowed pools
    fn in_a_pool(address: u32) -> bool {
        POOLS
            .iter()
            .any(|(base, prefix)| network_of(address, *prefix) == *base)
    }

    #[test]
    fn a_seed_always_gives_the_same_set() {
        assert_eq!(subnetting_set(7, None), subnetting_set(7, None));
        assert_ne!(subnetting_set(7, None), subnetting_set(8, None));
        assert_eq!(subnetting_set(7, None).len(), SET_SIZE);
    }

    // A shared link is a promise about what the reader sees. If this fails, the
    // generator changed and every shared set changed with it
    #[test]
    fn the_generator_output_is_pinned() {
        let mut rng = Rng(1);
        assert_eq!(rng.next(), 0x910A_2DEC_8902_5CC1);
        assert_eq!(rng.next(), 0xBEEB_8DA1_658E_EC67);
    }

    #[test]
    fn a_kind_filter_gives_only_that_kind() {
        for kind in ProblemKind::ALL {
            assert_eq!(ProblemKind::from_key(kind.key()), Some(kind));
            for problem in subnetting_set(3, Some(kind)) {
                assert_eq!(problem.kind(), kind);
            }
        }
        assert_eq!(ProblemKind::from_key("wildcard"), None);
    }

    #[test]
    fn subnet_arithmetic_agrees_with_a_direct_calculation() {
        let address = u32::from_be_bytes([172, 16, 37, 201]);
        for prefix in MIN_PREFIX..=MAX_PREFIX {
            let size = 1u64 << (ADDRESS_BITS - prefix);
            let network = network_of(address, prefix);
            let broadcast = broadcast_of(address, prefix);
            assert_eq!(u64::from(network) % size, 0, "/{prefix} network alignment");
            assert_eq!(u64::from(broadcast - network) + 1, size, "/{prefix} size");
            assert!(network <= address && address <= broadcast);
            assert_eq!(usable_hosts(prefix), size - 2);
        }
        assert_eq!(dotted(network_of(address, 20)), "172.16.32.0");
        assert_eq!(dotted(broadcast_of(address, 20)), "172.16.47.255");
        assert_eq!(usable_hosts(26), 62);
        assert_eq!(usable_hosts(30), 2);
    }

    #[test]
    fn the_prefix_for_a_host_count_is_the_longest_that_fits() {
        for hosts in [1, 2, 3, 6, 7, 14, 15, 62, 63, 254, 255, 500, 510, 511, 4094] {
            let prefix = prefix_for_hosts(hosts);
            assert!(usable_hosts(prefix) >= hosts, "{hosts} hosts in /{prefix}");
            if prefix < MAX_PREFIX {
                assert!(
                    usable_hosts(prefix + 1) < hosts,
                    "{hosts} hosts would also fit /{}",
                    prefix + 1
                );
            }
        }
        assert_eq!(prefix_for_hosts(500), 23);
        assert_eq!(prefix_for_hosts(2), 30);
    }

    #[test]
    fn generated_problems_stay_in_range_and_solve_cleanly() {
        for seed in 0..200 {
            for problem in subnetting_set(seed, None) {
                match &problem {
                    Problem::Network { address, prefix } => {
                        assert!((MIN_PREFIX..=MAX_PREFIX).contains(prefix));
                        let network = network_of(*address, *prefix);
                        let broadcast = broadcast_of(*address, *prefix);
                        assert!(in_a_pool(network) && in_a_pool(broadcast));
                        assert!(
                            network < *address && *address < broadcast,
                            "the given address should be a usable host"
                        );
                    }
                    Problem::Hosts { prefix } => {
                        assert!((MIN_PREFIX..=MAX_PREFIX).contains(prefix));
                    }
                    Problem::Sizing { hosts } => {
                        assert!(*hosts >= 1);
                        assert!(prefix_for_hosts(*hosts) >= SIZING_MIN_PREFIX);
                    }
                }
                // Every field's own displayed answer is accepted
                for field in problem.fields() {
                    assert!(field.answer.accepts(&field.answer.display()));
                }
                assert!(!problem.working().is_empty());
            }
        }
    }

    #[test]
    fn answers_are_compared_as_values() {
        let address = Answer::Address(u32::from_be_bytes([10, 0, 0, 1]));
        assert!(address.accepts("10.0.0.1"));
        assert!(address.accepts(" 010.000.0.01 "));
        assert!(!address.accepts("10.0.0.2"));
        assert!(!address.accepts("10.0.0"));
        assert!(!address.accepts("10.0.0.1.5"));
        assert!(!address.accepts("10.0.0.256"));
        assert!(!address.accepts(""));

        let prefix = Answer::Prefix(26);
        assert!(prefix.accepts("/26"));
        assert!(prefix.accepts("26"));
        assert!(!prefix.accepts("/27"));
        assert!(!prefix.accepts("255.255.255.192"));

        let number = Answer::Number(16_777_214);
        assert!(number.accepts("16777214"));
        assert!(number.accepts("16,777,214"));
        assert!(!number.accepts("16777216"));
        assert!(!number.accepts("lots"));
    }

    #[test]
    fn the_working_shows_the_block_size_method() {
        let off_boundary = Problem::Network {
            address: u32::from_be_bytes([172, 16, 37, 201]),
            prefix: 20,
        }
        .working();
        assert!(off_boundary.contains("255.255.240.0"));
        assert!(off_boundary.contains("third octet"));
        assert!(off_boundary.contains("256 - 240 = 16"));
        assert!(off_boundary.contains("starts at 32 and ends before 48"));
        assert!(off_boundary.contains("Network 172.16.32.0, broadcast 172.16.47.255"));

        let on_boundary = Problem::Network {
            address: u32::from_be_bytes([192, 168, 5, 77]),
            prefix: 24,
        }
        .working();
        assert!(on_boundary.contains("ends on an octet boundary"));
        assert!(on_boundary.contains("hosts 192.168.5.1 to 192.168.5.254"));

        assert!(
            Problem::Hosts { prefix: 26 }
                .working()
                .contains("2^6 = 64 addresses")
        );
        assert!(
            Problem::Sizing { hosts: 500 }
                .working()
                .contains("32 - 9 = 23")
        );
    }

    #[test]
    fn a_problem_is_graded_field_by_field() {
        let problem = Problem::Network {
            address: u32::from_be_bytes([172, 16, 37, 201]),
            prefix: 20,
        };
        let all_right = GradedProblem::new(problem.clone(), |key| {
            match key {
                "network" => "172.16.32.0",
                "broadcast" => "172.16.47.255",
                "first" => "172.16.32.1",
                _ => "172.16.47.254",
            }
            .to_string()
        });
        assert_eq!(all_right.verdict(), "correct");

        let one_wrong = GradedProblem::new(problem.clone(), |key| {
            if key == "network" {
                "172.16.32.0".to_string()
            } else {
                "172.16.0.0".to_string()
            }
        });
        assert_eq!(one_wrong.verdict(), "incorrect");
        assert!(one_wrong.fields[0].correct());
        assert!(!one_wrong.fields[1].correct());

        // Skipped is not wrong
        let blank = GradedProblem::new(problem, |_| "  ".to_string());
        assert_eq!(blank.verdict(), "skipped");
        assert_eq!(blank.verdict_class(), "is-skipped");
    }

    // Each kind links to the heading that teaches it. Resolved against the ids
    // the renderer generates, the same way the question bank's citations are
    #[test]
    fn every_problem_kind_cites_a_real_published_heading() {
        let pages = std::path::PathBuf::from(crate::handlers::wiki::PAGES_DIR);
        let page = crate::models::page::Page::find(&pages, LEARN_SLUG)
            .expect("the subnetting page exists");
        assert!(!page.draft, "drills cite a draft page");
        let ids: Vec<&str> = page.outline.iter().map(|h| h.id.as_str()).collect();
        for kind in ProblemKind::ALL {
            assert!(
                ids.contains(&kind.learn_anchor()),
                "{:?} cites #{}, which has no such heading. Available: {ids:?}",
                kind,
                kind.learn_anchor()
            );
        }
    }
}
