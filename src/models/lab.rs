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
//              The dependency chain is the honest part. Almost everything here
//              is Blocked, and it is blocked on a specific named thing rather
//              than on enthusiasm. The recovery exit gate in
//              network-segmentation-runbook.md §5 freezes all segmentation
//              work until thirteen checks pass with fresh evidence; that freeze
//              is why a list of a dozen network projects has exactly one entry
//              anyone can start today.

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

// The three stages of the homelab program, in the order they must happen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    // Get the flat 10.0.10.0/24 baseline coherent and evidenced
    Recovery,
    // Divide it into VLANs, one change domain at a time
    Segmentation,
    // Put each service on its target VLAN and prove its config
    Services,
}

impl Phase {
    pub fn label(&self) -> &'static str {
        match self {
            Phase::Recovery => "Recovery",
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
    // Unblocked and startable today. The runbooks name exactly one of these.
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
        // ---- Phase 1: recovery on the flat network -------------------------
        Lab {
            name: "R0 — Fresh read-only snapshot",
            entails: "Run the same read-only checks on all three Proxmox nodes and save timestamped \
                      output outside /etc/pve: addressing, routes, resolver, service state, corosync \
                      config hashes, quorum, VM inventory, firewall status, and the guest-agent view \
                      from the VM serving this site.",
            why: "Every other item on this list depends on knowing the real current state. The last \
                  evidence on file is a historical checkpoint from an incident, not a description of \
                  the machines as they are now. Acting on a stale snapshot is how the outage happened \
                  the first time.",
            phase: Phase::Recovery,
            status: LabStatus::Next,
            blocked_by: None,
            runbook: "runbooks/network/00-recovery-snapshot.md",
            writeup_url: None,
        },
        Lab {
            name: "R1 — One coherent three-node cluster",
            entails: "Bring all three nodes into agreement at every layer, not just at Corosync: the \
                      same config version and ring addresses on each node, both peers connected, and \
                      the Proxmox cluster filesystem actually attached — node 2's corosync.conf row \
                      was missing from its database and node 3 has no successful recovery log.",
            why: "Corosync reporting quorum is not the same as the cluster filesystem being healthy, \
                  and the difference is exactly what was misread during the incident. Nothing built on \
                  top of a half-attached cluster can be trusted.",
            phase: Phase::Recovery,
            status: LabStatus::Blocked,
            blocked_by: Some("R0 — Fresh read-only snapshot"),
            runbook: "runbooks/network/01-recovery-cluster.md",
            writeup_url: None,
        },
        Lab {
            name: "R2 — Firewall policy on the recovery subnet",
            entails: "Reconcile host firewall policy across all three nodes. Two still allow the old \
                      pre-migration subnet as a management source and then drop management ports from \
                      anywhere else; one has no host firewall file at all, so its intended policy is \
                      undefined.",
            why: "On the new subnet those stale allow-rules can lock SSH and the web UI out of the \
                  cluster, depending on rule evaluation order. A firewall that was correct for the old \
                  network is a lockout risk on the new one.",
            phase: Phase::Recovery,
            status: LabStatus::Blocked,
            blocked_by: Some("R1 — One coherent three-node cluster"),
            runbook: "runbooks/network/02-recovery-firewall.md",
            writeup_url: None,
        },
        Lab {
            name: "R3 — Names, addresses, and DNS ownership",
            entails: "Build one authoritative host/IP/DNS table and remove stale pre-migration \
                      references from active automation. The preserved /etc/hosts files disagree with \
                      each other about this server's address across three different values, while the \
                      guest agent proves a fourth.",
            why: "Four sources of truth for one address is not a documentation problem, it is the \
                  thing that breaks a VLAN cutover. Inventory has to be settled before anything moves.",
            phase: Phase::Recovery,
            status: LabStatus::Blocked,
            blocked_by: Some("R2 — Firewall policy on the recovery subnet"),
            runbook: "runbooks/network/03-recovery-inventory-dns.md",
            writeup_url: None,
        },
        Lab {
            name: "R4 — Prove and soak the flat baseline",
            entails: "Run the positive and negative flow tests against the flat network and leave it \
                      alone long enough to see whether it stays healthy — cluster membership, the \
                      public request path, and DNS all still working after a period of no changes.",
            why: "A baseline that works once is not a baseline. Soaking it is what separates \
                  'recovered' from 'not currently broken', and it is the last chance to find a fault \
                  while the network is still simple enough to reason about.",
            phase: Phase::Recovery,
            status: LabStatus::Blocked,
            blocked_by: Some("R3 — Names, addresses, and DNS ownership"),
            runbook: "runbooks/network/04-recovery-soak.md",
            writeup_url: None,
        },
        Lab {
            name: "The recovery exit gate",
            entails: "Thirteen checks, each needing fresh evidence: three quorate nodes, matching \
                      config hashes, documented firewall policy on every node, a verified management \
                      path, a completed host/IP/DNS table, captured VM inventory, a verified public \
                      request path, exported and recoverable firewall and switch configs, and at least \
                      one tested rollback path for each of host networking, firewall, OPNsense, and \
                      the switch.",
            why: "This is the freeze. Until every box is checked, no VLAN filtering, no switch PVID \
                  change, no VM tag, no OPNsense interface change. The gate exists because the last \
                  attempt at segmentation started from an unproven baseline and took the cluster, \
                  remote access, and this site down for two days.",
            phase: Phase::Recovery,
            status: LabStatus::Blocked,
            blocked_by: Some("R4 — Prove and soak the flat baseline"),
            runbook: "runbooks/network/05-recovery-exit-gate.md",
            writeup_url: None,
        },
        // ---- Phase 2: segmentation, one change domain at a time ------------
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
            blocked_by: Some("The recovery exit gate"),
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
        // ---- Phase 3: services onto their target zones ---------------------
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

    // The dependency chain is the reason this page is worth showing. If every
    // entry were startable, "blocked" would be an excuse; because the runbooks
    // name one next action and a hard gate, it is a plan.
    #[test]
    fn exactly_one_lab_is_startable_and_the_rest_name_their_blocker() {
        let labs = all();
        let startable: Vec<&str> = labs
            .iter()
            .filter(|lab| lab.status == LabStatus::Next)
            .map(|lab| lab.name)
            .collect();
        assert_eq!(
            startable.len(),
            1,
            "the runbooks name exactly one immediate next action, found {startable:?}"
        );

        for lab in &labs {
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

    // A blocker has to be a lab on this list, or the chain is decorative
    #[test]
    fn every_blocker_names_a_lab_that_exists() {
        let labs = all();
        let names: Vec<&str> = labs.iter().map(|lab| lab.name).collect();
        for lab in &labs {
            if let Some(blocker) = lab.blocked_by {
                assert!(
                    names.contains(&blocker),
                    "{} is blocked by {blocker:?}, which is not on the list",
                    lab.name
                );
            }
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
