// Author:      machinageist
// Date:        2026-08-21
// Description: Public metadata for reusable lab exercises. The entries describe
//              educational patterns, not the state or topology of a live network.

#[derive(Debug, Clone)]
pub struct Lab {
    pub slug: &'static str,
    pub name: &'static str,
    pub why: &'static str,
    pub phase: Phase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    NetworkDesign,
    ServiceDesign,
}

impl Phase {
    pub fn label(&self) -> &'static str {
        match self {
            Phase::NetworkDesign => "Network design",
            Phase::ServiceDesign => "Service design",
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

pub fn all() -> Vec<Lab> {
    vec![
        Lab {
            slug: "segmentation-topology",
            name: "Document the physical topology",
            why: "A verified port map and independent recovery path turn a risky network change into a reviewable one.",
            phase: Phase::NetworkDesign,
        },
        Lab {
            slug: "segmentation-prepare",
            name: "Prepare without moving traffic",
            why: "Separating preparation from cutover reduces the number of variables that can fail at once.",
            phase: Phase::NetworkDesign,
        },
        Lab {
            slug: "segmentation-lab",
            name: "Prove a low-risk test zone first",
            why: "Tagging, policy, and negative tests should be proven on disposable systems before a real cutover.",
            phase: Phase::NetworkDesign,
        },
        Lab {
            slug: "segmentation-guest-trusted",
            name: "Separate user and untrusted clients",
            why: "Client groups with different trust requirements should not share an unrestricted broadcast domain.",
            phase: Phase::NetworkDesign,
        },
        Lab {
            slug: "segmentation-admin",
            name: "Create a narrow administration path",
            why: "Management restrictions are safe only after an independent administrative path has been tested.",
            phase: Phase::NetworkDesign,
        },
        Lab {
            slug: "segmentation-servers",
            name: "Move services after lower-risk zones",
            why: "Public and shared services should move only after the same mechanism has succeeded on less visible systems.",
            phase: Phase::NetworkDesign,
        },
        Lab {
            slug: "segmentation-mgmt",
            name: "Move management traffic last",
            why: "Changing the network used for recovery should be the final cutover, not the first experiment.",
            phase: Phase::NetworkDesign,
        },
        Lab {
            slug: "firewall-policy",
            name: "Firewall and router policy",
            why: "Segmentation matters only when routing ownership and the allow-and-deny policy are explicit and tested.",
            phase: Phase::ServiceDesign,
        },
        Lab {
            slug: "bastion-host",
            name: "Bastion host design",
            why: "A controlled entry point needs a narrow role, auditable access, and a documented failure boundary.",
            phase: Phase::ServiceDesign,
        },
        Lab {
            slug: "remote-access",
            name: "Choose one primary remote-access path",
            why: "Overlapping remote-access systems create duplicate policy surfaces and make effective reachability harder to audit.",
            phase: Phase::ServiceDesign,
        },
        Lab {
            slug: "container-host",
            name: "Container host boundaries",
            why: "Workload inventory, persistence, and network policy should be known before a container host crosses trust zones.",
            phase: Phase::ServiceDesign,
        },
        Lab {
            slug: "media-service",
            name: "Keep a media service local by default",
            why: "A service should not become internet-reachable merely because a convenient publishing path already exists.",
            phase: Phase::ServiceDesign,
        },
        Lab {
            slug: "lab-network",
            name: "Isolated test machines",
            why: "Disposable systems are useful only when their isolation and negative firewall rules are verified.",
            phase: Phase::ServiceDesign,
        },
        Lab {
            slug: "rhel-study-box",
            name: "Platform-specific study machine",
            why: "Distribution-specific claims should be tested on the platform they describe rather than inferred from another system.",
            phase: Phase::ServiceDesign,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, path::Path};

    #[test]
    fn public_lab_metadata_is_complete_and_unique() {
        let labs = all();
        let mut slugs = HashSet::new();

        assert!(!labs.is_empty());
        for lab in labs {
            assert!(slugs.insert(lab.slug), "duplicate lab slug: {}", lab.slug);
            assert!(!lab.name.is_empty());
            assert!(!lab.why.is_empty());
            assert!(
                Path::new("content/labs")
                    .join(format!("{}.md", lab.slug))
                    .is_file(),
                "{} has no matching educational page",
                lab.slug
            );
        }
    }

    #[test]
    fn public_metadata_does_not_describe_live_operations() {
        let copy = all()
            .iter()
            .map(|lab| format!("{} {}", lab.name, lab.why))
            .collect::<Vec<_>>()
            .join("\n");

        for disclosure in [
            "VLAN 10",
            "VLAN 20",
            "VLAN 30",
            "VLAN 40",
            "VLAN 50",
            "VLAN 60",
            "Tailscale",
            "WireGuard",
            "Proxmox",
            "Corosync",
            "runbooks/",
            "recovery exit gate",
            "the VM exists",
            "does not exist yet",
        ] {
            assert!(
                !copy.contains(disclosure),
                "public lab metadata contains operational detail: {disclosure}"
            );
        }
    }
}
