// Author:      machinageist
// Date:        2026-08-14
// Description: Defines Lab, Phase, and LabStatus — the data model for the /labs
//              progress page. Entries are transcribed from the runbooks in
//              ~/mg-coreforge/runbooks/, which are the authority on what the
//              homelab work actually is and what order it has to happen in.
//              Hardcoded in all() as a static Vec for the same reason Project
//              is: structured, typed, changes rarely, compiler-validated.
//
// Notes:       This is a PROGRESS surface, not a portfolio. criteria.md 1C
//              permits work in progress to appear here and forbids it from
//              implying portfolio status, so nothing in this list may read as
//              finished. The invariant that enforces it: a lab is Done only
//              when it has a published writeup_url, and the tests below fail
//              if that is ever violated.
//
//              Scope is deliberately forward-looking: planned work only. The
//              recovery that precedes all of it is finished writing rather than
//              a queue item — it is told properly in the network-migration post,
//              and listing it here again would make the page about the past.
//
//              The dependency chain is the honest part. Everything is Blocked,
//              on a specific named thing rather than on enthusiasm, and the
//              first blocker is off-list: the recovery exit gate in
//              network-segmentation-runbook.md §5 freezes all segmentation work
//              until thirteen checks pass with fresh evidence.

// -----------------------------------------------------------------------
// Data types
// -----------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Lab {
    pub name: &'static str,
    // What the work actually consists of
    pub entails: &'static str,
    // Why it exists — the reason it is on the list at all, not a restatement
    // of the task. This is the field that makes the page worth reading.
    pub why: &'static str,
    pub phase: Phase,
    pub status: LabStatus,
    // What has to finish first. None only when a lab can be started today.
    pub blocked_by: Option<&'static str>,
    // The runbook this entry is transcribed from, relative to ~/mg-coreforge/
    pub runbook: &'static str,
    // None until Done — set only when a real writeup is published
    pub writeup_url: Option<&'static str>,
}

impl Lab {
    pub fn status_class(&self) -> &'static str {
        self.status.class_name()
    }
}

// The precondition every entry here waits on. It is not a lab on this list:
// closing out the recovery is current work, written up rather than queued.
pub const RECOVERY_GATE: &str = "the recovery exit gate";

// The two stages of planned work, in the order they must happen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    // Divide the flat network into VLANs, one change domain at a time
    Segmentation,
    // Put each service on its target VLAN and prove its config
    Services,
}

impl Phase {
    pub fn label(&self) -> &'static str {
        match self {
            Phase::Segmentation => "Segmentation",
            Phase::Services => "Services",
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

// InProgress and Done are unconstructed today because nothing has been started
// yet — which is the honest state, not an oversight. They are part of the model
// so that starting a lab is a one-word edit here rather than a refactor, and the
// tests below already assert what Done must satisfy.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabStatus {
    // Unblocked and startable today. Nothing is, until the gate passes.
    Next,
    // Waiting on something specific, named in blocked_by
    Blocked,
    InProgress,
    // Finished AND written up. There is no "done but undocumented" state on
    // purpose — that state is how a progress page becomes an overclaim.
    Done,
}

impl LabStatus {
    pub fn class_name(&self) -> &'static str {
        match self {
            LabStatus::Next => "next",
            LabStatus::Blocked => "blocked",
            LabStatus::InProgress => "in-progress",
            LabStatus::Done => "done",
        }
    }
}

// Allow {{ lab.status }} in Askama templates — renders the display string directly
impl std::fmt::Display for LabStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LabStatus::Next => write!(f, "next"),
            LabStatus::Blocked => write!(f, "blocked"),
            LabStatus::InProgress => write!(f, "in progress"),
            LabStatus::Done => write!(f, "done"),
        }
    }
}

// -----------------------------------------------------------------------
// The program — transcribed from ~/mg-coreforge/runbooks/. Flip status and
// set writeup_url only when a lab is finished AND published.
// -----------------------------------------------------------------------

// Return the canonical lab list in dependency order
pub fn all() -> Vec<Lab> {
    vec![
        // ---- Segmentation, one change domain at a time ---------------------
        Lab {
            name: "S0 — Document the physical topology",
            entails: "Record what is actually plugged in where: switch model, firmware, and exported \
                      config; port to device to NIC mapping; current PVID and VLAN membership per port; \
                      the firewall's parent interface and assignments; the real NIC names on each node, \
                      which differ; and every VM's ID, MAC, node, bridge, address, and target VLAN.",
            why: "No exported switch config, port map, or PVID table exists anywhere in the evidence. \
                  Configuring VLANs against a remembered topology is guessing, and the out-of-band \
                  path for each cutover has to be identified before the cutover, not during it.",
            phase: Phase::Segmentation,
            status: LabStatus::Blocked,
            blocked_by: Some(RECOVERY_GATE),
            runbook: "runbooks/network/10-segmentation-topology.md",
            writeup_url: None,
        },
        Lab {
            name: "S1 — Prepare without moving traffic",
            entails: "Make the VLAN-aware bridges, switch VLAN definitions, and firewall interfaces \
                      exist without moving a single host onto them. Nothing changes broadcast domain \
                      in this stage.",
            why: "Separating 'the plumbing exists' from 'traffic now uses it' halves the number of \
                  things that can be wrong when something breaks. If preparation and cutover happen in \
                  one window, a failure has twice as many candidate causes.",
            phase: Phase::Segmentation,
            status: LabStatus::Blocked,
            blocked_by: Some("S0 — Document the physical topology"),
            runbook: "runbooks/network/11-segmentation-prepare.md",
            writeup_url: None,
        },
        Lab {
            name: "S2 — Prove LAB (VLAN 50) first",
            entails: "Move the disposable lab VMs onto VLAN 50 and test the policy in both directions: \
                      that they can reach DNS, NTP, and updates, and that they cannot reach the \
                      management, trusted, server, or admin networks at all.",
            why: "LAB goes first because it is the zone whose failure costs nothing. Proving the \
                  mechanism — tagging, PVIDs, inter-VLAN rules, negative tests — on throwaway machines \
                  means the first real cutover is not also the first attempt.",
            phase: Phase::Segmentation,
            status: LabStatus::Blocked,
            blocked_by: Some("S1 — Prepare without moving traffic"),
            runbook: "runbooks/network/12-segmentation-lab.md",
            writeup_url: None,
        },
        Lab {
            name: "S3 — GUEST (60) and TRUSTED (20)",
            entails: "Stand up the untrusted client network with internet and approved DNS only, and \
                      the trusted client network with its narrower set of allowed internal \
                      destinations. Negative tests prove guest traffic cannot reach any internal range.",
            why: "These two zones are where the household actually lives, so they are the first \
                  segmentation anyone else in the building will notice. Getting them wrong is visible \
                  immediately, which is a good property this early.",
            phase: Phase::Segmentation,
            status: LabStatus::Blocked,
            blocked_by: Some("S2 — Prove LAB (VLAN 50) first"),
            runbook: "runbooks/network/13-segmentation-guest-trusted.md",
            writeup_url: None,
        },
        Lab {
            name: "S4 — ADMIN (40)",
            entails: "Move the bastion and the VPN endpoint onto the admin network, so that \
                      administrative access to everything else has one identified entry point rather \
                      than being reachable from wherever an admin happens to be sitting.",
            why: "The admin zone has to exist before management traffic can be restricted, because \
                  restricting management without a proven admin path is how you lock yourself out of \
                  your own cluster.",
            phase: Phase::Segmentation,
            status: LabStatus::Blocked,
            blocked_by: Some("S3 — GUEST (60) and TRUSTED (20)"),
            runbook: "runbooks/network/14-segmentation-admin.md",
            writeup_url: None,
        },
        Lab {
            name: "S5 — SERVERS (30), this site last",
            entails: "Move the service VMs onto VLAN 30 with egress limited to DNS, NTP, updates, and \
                      the outbound tunnel, and inbound from the internal networks denied. The VM \
                      serving machinageist.dev moves last, and the public request path is re-verified \
                      after it.",
            why: "This site is the one guest whose failure is publicly visible, so it moves only after \
                  the same change has been proven on something that does not have an audience.",
            phase: Phase::Segmentation,
            status: LabStatus::Blocked,
            blocked_by: Some("S4 — ADMIN (40)"),
            runbook: "runbooks/network/15-segmentation-servers.md",
            writeup_url: None,
        },
        Lab {
            name: "S6 — MGMT (10) explicitly tagged",
            entails: "Make the management network an explicitly tagged VLAN rather than the untagged \
                      default it is today, and confirm the cluster stays healthy across the change.",
            why: "Management goes last because it is the network the recovery is being performed \
                  over. Re-tagging it earlier would mean changing the road while driving on it, with \
                  no other road available.",
            phase: Phase::Segmentation,
            status: LabStatus::Blocked,
            blocked_by: Some("S5 — SERVERS (30), this site last"),
            runbook: "runbooks/network/16-segmentation-mgmt.md",
            writeup_url: None,
        },
        // ---- Services onto their target zones -------------------------------
        Lab {
            name: "Firewall and router configuration",
            entails: "Prove what is actually configured inside the OPNsense VM: which interfaces and \
                      VLANs it holds, whether it genuinely owns the gateway address, and an exported, \
                      restorable config. Then implement the inter-zone policy matrix as rules on the \
                      interface where traffic enters, using aliases rather than repeated literal \
                      addresses.",
            why: "The VM exists and the gateway address answers DNS and routes traffic, but no \
                  evidence proves which of those the firewall is responsible for. The policy matrix is \
                  the whole point of segmenting; without it, VLANs are just extra subnets.",
            phase: Phase::Services,
            status: LabStatus::Blocked,
            blocked_by: Some("S0 — Document the physical topology"),
            runbook: "runbooks/services/opnsense.md",
            writeup_url: None,
        },
        Lab {
            name: "Bastion host",
            entails: "Define and document what the bastion VM is for, harden it, and make it the \
                      single identified path to administrative interfaces.",
            why: "The VM exists and its role is undocumented, which means it is currently a machine \
                  with access rather than a control. A bastion nobody has written down the purpose of \
                  is an unaudited entry point.",
            phase: Phase::Services,
            status: LabStatus::Blocked,
            blocked_by: Some("S4 — ADMIN (40)"),
            runbook: "runbooks/services/bastion.md",
            writeup_url: None,
        },
        Lab {
            name: "Remote access — pick one primary",
            entails: "Decide between the self-hosted WireGuard endpoint and Tailscale as the primary \
                      remote-access path, then configure, prove, and document the one that wins.",
            why: "Tailscale is WireGuard with coordination and NAT traversal on top, so running both \
                  means two remote-access paths, two policy surfaces, and two ways to bypass the \
                  firewall matrix being built. The runbooks state the tradeoff deliberately and do not \
                  pick — this is an open decision, not an oversight.",
            phase: Phase::Services,
            status: LabStatus::Blocked,
            blocked_by: Some("S4 — ADMIN (40)"),
            runbook: "runbooks/services/wireguard.md",
            writeup_url: None,
        },
        Lab {
            name: "Container host and its workloads",
            entails: "Document what actually runs on the Docker host, put it on the servers network, \
                      and bring its egress under the same policy as every other service VM.",
            why: "The VM exists and its workloads are undocumented. An unknown set of containers on a \
                  network being locked down is the thing most likely to break in a way nobody can \
                  diagnose.",
            phase: Phase::Services,
            status: LabStatus::Blocked,
            blocked_by: Some("S5 — SERVERS (30), this site last"),
            runbook: "runbooks/services/docker-host.md",
            writeup_url: None,
        },
        Lab {
            name: "Media service, LAN-only by default",
            entails: "Install and run the media server on the container host, reachable from the local \
                      network only. Any public exposure is a separate decision with its own risk \
                      writeup.",
            why: "It does not exist yet — no VM, no install. Recording it as LAN-only by default \
                  matters because the tempting move is to reuse the tunnel that already fronts this \
                  site, and the runbook says explicitly not to infer that.",
            phase: Phase::Services,
            status: LabStatus::Blocked,
            blocked_by: Some("Container host and its workloads"),
            runbook: "runbooks/services/jellyfin.md",
            writeup_url: None,
        },
        Lab {
            name: "Isolated lab VMs",
            entails: "Keep the disposable lab machines on VLAN 50 with no route to any other internal \
                      zone, reachable over SSH only when the connection is initiated from the admin \
                      network. These are the machines S2 uses to prove the segmentation mechanism.",
            why: "The value here is the isolation, not the machines. A zone whose members are expected \
                  to be rebuilt, broken, or thrown away is the correct place to test whether the \
                  negative firewall rules actually hold.",
            phase: Phase::Services,
            status: LabStatus::Blocked,
            blocked_by: Some("S2 — Prove LAB (VLAN 50) first"),
            runbook: "runbooks/services/kali-lab.md",
            writeup_url: None,
        },
        Lab {
            name: "RHEL study box",
            entails: "Keep a Red Hat Enterprise Linux VM on the lab network as the practice machine \
                      for the Linux systems administration work — the environment where the commands \
                      documented in the education wiki get run against the distribution they are \
                      written for.",
            why: "The Linux pages on this site are checked against man pages and the Filesystem \
                  Hierarchy Standard, but this server runs Debian. A RHEL box is where a claim about \
                  RHEL tooling can be tested rather than assumed.",
            phase: Phase::Services,
            status: LabStatus::Blocked,
            blocked_by: Some("S2 — Prove LAB (VLAN 50) first"),
            runbook: "runbooks/services/rhel-lab.md",
            writeup_url: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_is_finished_and_nothing_pretends_to_be() {
        let labs = all();
        assert!(!labs.is_empty(), "the program should not be empty");
        assert!(
            labs.iter().all(|lab| lab.status != LabStatus::Done),
            "no lab is finished yet — flipping one to Done requires a published writeup"
        );
        assert!(
            labs.iter().all(|lab| lab.writeup_url.is_none()),
            "writeup_url must stay None until a lab is genuinely Done and published"
        );
    }

    // The honesty invariant this whole surface rests on. criteria.md 1C lets
    // work in progress appear on a progress page and forbids it implying
    // portfolio status; "done but undocumented" is exactly how that line gets
    // crossed, so the type system's Done is defined to include the writeup.
    #[test]
    fn a_lab_is_done_only_when_its_writeup_is_published() {
        for lab in all() {
            if lab.status == LabStatus::Done {
                assert!(
                    lab.writeup_url.is_some(),
                    "{} is marked Done with no writeup — either publish one or change the status",
                    lab.name
                );
            }
        }
    }

    // The dependency chain is the reason this page is worth showing. If entries
    // were blocked on nothing in particular, "blocked" would be an excuse.
    #[test]
    fn every_blocked_lab_says_what_is_blocking_it() {
        for lab in all() {
            match lab.status {
                LabStatus::Next => assert!(
                    lab.blocked_by.is_none(),
                    "{} is startable but names a blocker",
                    lab.name
                ),
                LabStatus::Blocked => assert!(
                    lab.blocked_by.is_some(),
                    "{} is blocked but does not say by what",
                    lab.name
                ),
                _ => {}
            }
        }
    }

    // A blocker has to resolve to something real, or the chain is decorative.
    // Exactly one blocker is allowed to be off-list — the recovery gate, which
    // is current work rather than planned work and is written up instead.
    #[test]
    fn every_blocker_resolves_to_a_lab_or_the_declared_precondition() {
        let labs = all();
        let names: Vec<&str> = labs.iter().map(|lab| lab.name).collect();
        for lab in &labs {
            if let Some(blocker) = lab.blocked_by {
                assert!(
                    names.contains(&blocker) || blocker == RECOVERY_GATE,
                    "{} is blocked by {blocker:?}, which is neither on the list nor the \
                     declared precondition",
                    lab.name
                );
            }
        }
    }

    // Scope guard. The recovery work is covered by the network-migration post;
    // re-listing it here would make a page about planned work into a rehash.
    #[test]
    fn the_list_carries_planned_work_only() {
        let labs = all();
        assert!(
            labs.iter()
                .all(|lab| matches!(lab.phase, Phase::Segmentation | Phase::Services)),
            "only segmentation and services are planned work"
        );
        for lab in &labs {
            assert!(
                !lab.name.starts_with("R0")
                    && !lab.name.starts_with("R1")
                    && !lab.name.starts_with("R2")
                    && !lab.name.starts_with("R3")
                    && !lab.name.starts_with("R4"),
                "{} is a recovery phase — that work is written up, not queued",
                lab.name
            );
        }
    }

    // Every entry traces to the runbook it came from, so the page can never
    // drift into aspirations nobody wrote an execution document for
    #[test]
    fn every_lab_cites_a_runbook() {
        for lab in all() {
            assert!(
                lab.runbook.starts_with("runbooks/") && lab.runbook.ends_with(".md"),
                "{} cites {:?}, which is not a runbook path",
                lab.name,
                lab.runbook
            );
            assert!(
                !lab.why.is_empty() && !lab.entails.is_empty(),
                "{} must say both what it involves and why it exists",
                lab.name
            );
        }
    }

    #[test]
    fn labs_never_claim_offensive_or_unearned_identity() {
        let labs = all();
        let combined = labs
            .iter()
            .map(|lab| format!("{} {} {}", lab.name, lab.entails, lab.why))
            .collect::<Vec<_>>()
            .join("\n");

        // Anti-overclaim guards. The lab network holds a disposable security VM,
        // and the defensible framing is the isolation around it — not an
        // offensive-security identity the publication gate in criteria.md 1C
        // does not permit.
        for banned in [
            "SOC analyst",
            "penetration test",
            "pentest",
            "red team",
            "red-team",
            "offensive security",
            "HackerOne",
            "bug bounty",
            "Hack The Box",
        ] {
            assert!(
                !combined.contains(banned),
                "lab copy claims {banned:?}, which the claim discipline forbids"
            );
        }
    }

    // The 2026-08-02 re-lock dropped Network+ and Server+ from the spine. The
    // previous version of this file not only carried "Anchored to Network+" and
    // "Anchored to Server+" in its copy, it asserted their presence — so the
    // guard meant to protect claim integrity was pinning a stale claim in place.
    #[test]
    fn lab_copy_names_no_certification() {
        let labs = all();
        let combined = labs
            .iter()
            .map(|lab| format!("{} {} {}", lab.name, lab.entails, lab.why))
            .collect::<Vec<_>>()
            .join("\n");

        for cert in [
            "Network+",
            "Security+",
            "Server+",
            "Linux+",
            "CompTIA",
            "RHCSA",
            "CCNA",
        ] {
            assert!(
                !combined.contains(cert),
                "lab copy names {cert:?} — no cert claim without a booked voucher \
                 (criteria.md auto-fail rule 1)"
            );
        }
    }
}
