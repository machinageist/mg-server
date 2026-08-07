// Author:      machinageist
// Date:        2026-07-26
// Description: Defines Lab and LabStatus — the data model for the /labs page.
//              Labs are curated from the bootcamp study repo's per-subject
//              backlogs (~mg-coreforge/bootcamp/subjects/*/labs/) and hardcoded
//              in all() as a static Vec, same reasoning as Project: structured,
//              typed, changes rarely, compiler-validated at build time.
//              LabStatus implements Display so templates can render
//              {{ lab.status }} directly without a helper method.
//
// Notes:       Distinct status vocabulary from ProjectStatus on purpose —
//              "queued" (not started) is a different claim than "active" (a
//              running project), so the two enums stay separate rather than
//              sharing variants. writeup_url is None until a lab is Completed;
//              flipping status and setting writeup_url is a manual edit here,
//              same workflow as adding a url to a Project once it ships.

// -----------------------------------------------------------------------
// Data types
// -----------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Lab {
    pub name: &'static str,
    pub description: &'static str,
    // One of the site's five pillars, or "General" if it spans more than one
    pub pillar: &'static str,
    // Observe / Tend / Repair / Share — lifecycle framing carried forward
    // from the solarpunk-portfolio-makeover draft
    pub lifecycle: &'static str,
    pub tags: &'static [&'static str],
    pub status: LabStatus,
    // None until Completed — set once a real writeup is published
    pub writeup_url: Option<&'static str>,
}

impl Lab {
    pub fn status_class(&self) -> &'static str {
        self.status.class_name()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LabStatus {
    Queued,
    InProgress,
    Completed,
}

impl LabStatus {
    pub fn class_name(&self) -> &'static str {
        match self {
            LabStatus::Queued => "queued",
            LabStatus::InProgress => "in-progress",
            LabStatus::Completed => "completed",
        }
    }
}

// Allow {{ lab.status }} in Askama templates — renders the display string directly
impl std::fmt::Display for LabStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LabStatus::Queued => write!(f, "queued"),
            LabStatus::InProgress => write!(f, "in progress"),
            LabStatus::Completed => write!(f, "completed"),
        }
    }
}

// -----------------------------------------------------------------------
// Lab list — curated from the bootcamp subject backlogs. New entries land
// here once they're actually queued; flip status + set writeup_url when done.
// -----------------------------------------------------------------------

// Return canonical lab list — called by labs handler on each request
pub fn all() -> Vec<Lab> {
    vec![
        Lab {
            name: "Homelab Network & DNS Foundation",
            description: "Baseline topology mapping, a dedicated Pi DNS resolver, annotated packet \
                          captures of DHCP/DNS/ARP/TLS traffic, a throughput and QoS baseline, and one \
                          controlled network or DNS incident worked end to end. Anchored to Network+.",
            pillar: "Networking",
            lifecycle: "Observe",
            tags: &["homelab", "networking", "dns", "network+"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "HA Cluster — Foundation & Membership",
            description: "Three-node Proxmox cluster quorum, a manual VM migration between nodes, and a \
                          planned-maintenance drill — the first three rungs of a five-stage high-availability \
                          maturity ladder, verified before any failure is deliberately introduced.",
            pillar: "Homelab & Proxmox",
            lifecycle: "Observe",
            tags: &["homelab", "proxmox", "ha-cluster", "quorum"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "HA Cluster — Failure & Recovery Testing",
            description: "A deliberately induced abrupt node failure, a network-isolation drill, and node \
                          reintegration — the top two rungs of the maturity ladder, run only once cluster \
                          membership and planned failover are already proven.",
            pillar: "Homelab & Proxmox",
            lifecycle: "Repair",
            tags: &["homelab", "proxmox", "ha-cluster", "failure-testing"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Linux Fleet Configuration & Ansible Automation",
            description: "A role-based configuration baseline applied across every node in the cluster via \
                          Ansible, with idempotency proven by running the same playbook twice and diffing \
                          the result.",
            pillar: "Linux / SysAdmin",
            lifecycle: "Tend",
            tags: &["linux", "ansible", "automation", "configuration-management"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Backup, Restore & Storage Architecture",
            description: "A Proxmox Backup Server policy, a verified VM and file-level restore with measured \
                          RPO/RTO, and a storage-architecture decision record — because a green backup job \
                          is not a restore test. Anchored to Server+.",
            pillar: "Homelab & Proxmox",
            lifecycle: "Repair",
            tags: &["proxmox", "backup", "restore", "server+"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Centralized Identity & Access Management",
            description: "FreeIPA/OpenLDAP integrated with Proxmox and Linux logins, role-based access \
                          control, and an SSH key management policy replacing ad hoc local accounts.",
            pillar: "Linux / SysAdmin",
            lifecycle: "Tend",
            tags: &["linux", "identity", "ldap", "rbac"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Isolated Management Networking & Secure Remote Access",
            description: "VLAN segmentation separating production from management traffic behind OPNsense, \
                          with WireGuard plus MFA as the only path into the management plane.",
            pillar: "Networking",
            lifecycle: "Tend",
            tags: &["networking", "vlan", "wireguard", "opnsense"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Homelab Observability Scripts",
            description: "A small suite of cron-driven checks — DNS/certificate expiry, a daily log digest, \
                          a per-node health reporter, and a backup-freshness verifier — each alerting on its \
                          own failure condition.",
            pillar: "Linux / SysAdmin",
            lifecycle: "Observe",
            tags: &["linux", "automation", "monitoring", "scripting"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Rate Limiting & Custom Monitoring Dashboard",
            description: "A per-IP rate limiter added to a self-hosted service and load-tested by \
                          deliberately flooding it, plus a small dashboard built directly against the \
                          hypervisor API as a from-scratch companion to the standard Grafana stack.",
            pillar: "Linux / SysAdmin",
            lifecycle: "Tend",
            tags: &["rust", "monitoring", "rate-limiting", "dashboard"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Hardening & Security+ Baseline",
            description: "A hardening pass across the lab VMs and this site — key-only SSH, host firewalls, \
                          non-root service accounts, unattended updates — plus a before/after security-headers \
                          audit. Anchored to Security+.",
            pillar: "Security",
            lifecycle: "Tend",
            tags: &["security", "linux", "hardening", "security+"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Monitoring, SIEM & NOC Alert Workflow",
            description: "A metrics stack (Zabbix or Prometheus/Grafana/Alertmanager) paired with a \
                          single-node Wazuh SIEM and at least two custom detection rules, feeding one shared \
                          triage and alert workflow instead of two disconnected tools.",
            pillar: "Security",
            lifecycle: "Tend",
            tags: &["security", "siem", "wazuh", "noc"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
        Lab {
            name: "Help Desk Ticketing Workflow",
            description: "osTicket running on Docker, an intake-and-prioritization pipeline, a documentation \
                          template, and a small batch of dummy tickets worked through to close.",
            pillar: "Linux / SysAdmin",
            lifecycle: "Share",
            tags: &["linux", "docker", "itsm", "help-desk"],
            status: LabStatus::Queued,
            writeup_url: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labs_list_has_twelve_curated_entries_all_queued() {
        let labs = all();
        assert_eq!(labs.len(), 12, "curated list should have exactly 12 labs");
        assert!(
            labs.iter().all(|lab| lab.status == LabStatus::Queued),
            "no lab has actually been started yet — all should be Queued"
        );
        assert!(
            labs.iter().all(|lab| lab.writeup_url.is_none()),
            "no lab has a writeup yet — writeup_url must stay None until Completed"
        );
    }

    #[test]
    fn labs_cover_expected_pillars_and_leading_entries() {
        let labs = all();
        assert!(labs[0].name.contains("Homelab Network"));
        assert!(labs[1].name.contains("HA Cluster — Foundation"));

        let combined = labs
            .iter()
            .map(|lab| format!("{} {} {}", lab.name, lab.description, lab.pillar))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(combined.contains("Networking"));
        assert!(combined.contains("Homelab & Proxmox"));
        assert!(combined.contains("Linux / SysAdmin"));
        assert!(combined.contains("Security"));
        assert!(combined.contains("Network+"));
        assert!(combined.contains("Security+"));
    }

    #[test]
    fn labs_never_claim_offensive_or_unearned_identity() {
        let labs = all();
        let combined = labs
            .iter()
            .map(|lab| format!("{} {}", lab.name, lab.description))
            .collect::<Vec<_>>()
            .join("\n");

        // Anti-overclaim guards — security-offensive material and unearned
        // titles were deliberately excluded from the curated list; this test
        // stops them from creeping back in without a conscious decision.
        assert!(!combined.contains("SOC analyst"));
        assert!(!combined.contains("penetration test"));
        assert!(!combined.contains("red team"));
        assert!(!combined.contains("red-team"));
        assert!(!combined.contains("offensive security"));
        assert!(!combined.contains("HackerOne"));
        assert!(!combined.contains("bug bounty"));
        assert!(!combined.contains("Hack The Box"));
        assert!(!combined.contains("HTB"));
    }
}
