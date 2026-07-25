---
title: "Cloud computing concepts for networking"
date: 2026-07-24
summary: "NFV, virtual private clouds, cloud-native traffic controls, connectivity options, deployment models, and how cloud resources scale."
tags: [education, networking, cloud, nfv, virtualization, network-plus]
---

## Overview

Cloud computing moves networking functions that used to require dedicated
hardware onto shared, virtualized infrastructure. The functional roles
described on [network appliances](/learn/network-appliances) — routing,
firewalling, load balancing, storage — do not disappear in the cloud; they
are implemented as virtual machines, containers, and managed services running
on someone else's physical infrastructure instead of a chassis on a rack.

## Network function virtualization

**Network function virtualization (NFV)** replaces purpose-built network
hardware with software running on general-purpose infrastructure. Two
building blocks make this possible:

- A **virtual machine (VM)** emulates a complete computer, including its own
  operating system, running on top of a hypervisor. Several VMs can run on
  one physical host, each isolated from the others.
- A **container** packages an application with its dependencies but shares
  the host operating system's kernel rather than emulating a full machine.
  Containers are lighter weight than VMs and start faster, at the cost of
  weaker isolation from the host and from each other. Docker is the most
  common container runtime; Podman is a FOSS alternative that does not
  require a root daemon.

A **virtual appliance** is a network function — a firewall, router, or load
balancer — packaged to run inside a VM or container instead of on dedicated
hardware.

NFV architecture is commonly described in three parts: the **virtualized
network function (VNF)** itself, the **NFV infrastructure (NFVi)** — the
compute, storage, and networking resources the VNF runs on — and
**management and orchestration (MANO)**, which deploys, scales, and
coordinates VNFs across that infrastructure.

## Virtual private clouds

A **virtual private cloud (VPC)** is a logically isolated section of a cloud
provider's network reserved for one customer's resources. A VPC can be
divided into subnets, and those subnets are commonly split by exposure:

- A **public subnet** has a route to an internet gateway and can hold
  internet-facing services.
- A **private subnet** has no direct route to the internet gateway. Its
  resources are not reachable from the public internet by default — but
  "isolated" is a matter of routing, not absolute: a NAT gateway, VPN, or
  bastion host can still give a private subnet controlled reachability in one
  direction or the other.

Databases and internal application servers are commonly placed in private
subnets so they are not directly addressable from outside the VPC.

## Traffic controls: security groups and access lists

Cloud providers commonly offer two layers of traffic policy that are easy to
conflate, and naming varies by vendor:

- A rule set attached to specific network interfaces — AWS's **Security
  Groups**, Azure's **Network Security Groups (NSGs)**, and Oracle Cloud
  Infrastructure's **Network Security Groups (NSGs)** all work this way, and
  are stateful by default: allowing a request automatically allows its return
  traffic.
- A rule set attached to a subnet — AWS's **Network ACLs** and OCI's
  **Security Lists** apply at this level. AWS's Network ACLs are stateless,
  requiring explicit rules in both directions; OCI's Security Lists can be
  configured as either stateful or stateless.

Both layers filter by criteria such as source/destination IP, port, and
protocol — for example, a web server's rule set might allow inbound TCP 80
and 443 and deny everything else by default. ("Network Security List" is not
standard terminology for any major provider; the OCI construct is a
**Security List**.)

## Cloud gateways

A **cloud gateway** connects cloud resources to the internet or to other
networks. Two common types:

- An **internet gateway** provides a route between a VPC's public subnets
  and the internet.
- A **NAT gateway** lets resources in a private subnet initiate outbound
  connections to the internet while remaining unreachable from unsolicited
  inbound connections.

## Connecting to the cloud

Two general approaches link on-premises networks to cloud resources:

- **Direct/dedicated connections** are private links leased from a provider.
  They offer more consistent latency and throughput than a connection routed
  over the public internet, at higher cost, and are **not encrypted by
  default** — encryption has to be layered on top if it's required.
- **Site-to-site VPNs** build an encrypted tunnel over the ordinary internet.
  They are cheaper and faster to provision than a dedicated circuit, at the
  cost of depending on internet-path performance.

## Cloud deployment models

- **Public cloud** — a cloud service provider (CSP) hosts many customers on
  shared infrastructure.
- **Private cloud** — infrastructure is dedicated to one organization,
  whether owned outright or leased from a CSP as single-tenant hardware. This
  trades cost for a stronger isolation and compliance story.
- **Hybrid cloud** — an organization runs some workloads on-premises and
  others with a CSP, usually connected by one of the methods above.
- **Community cloud** — a group of organizations with shared requirements (a
  regulatory framework or industry standard, for example) jointly use
  infrastructure built around those requirements.

## Cloud service models

- **Infrastructure as a Service (IaaS)** provides virtualized compute,
  storage, and networking; the customer manages everything from the
  operating system up.
- **Platform as a Service (PaaS)** adds a managed runtime and supporting
  services for building applications, removing operating-system and
  infrastructure management from the customer's responsibility.
- **Software as a Service (SaaS)** delivers a complete application, typically
  through a browser and a subscription — Microsoft 365 is a common example.

Moving from IaaS toward SaaS trades customer control for provider-managed
convenience; it does not change who is accountable for how the customer's own
data is used within the service.

## Scalability, elasticity, and multitenancy

- **Scalability** is the capacity to add or remove resources to meet demand,
  often planned and provisioned ahead of time.
- **Elasticity** is automatic, near-real-time adjustment of resources in
  response to current demand — scaling out under load and back in as demand
  drops, usually without manual intervention.
- **Multitenancy** means multiple customers' workloads share the same
  underlying physical infrastructure, isolated from each other logically
  rather than physically.

## Suggested practice: compare a VM and a container locally

No cloud account is required to observe the core VM-versus-container
distinction:

1. Install a local hypervisor (VirtualBox or KVM/libvirt) and boot a small
   Linux VM.
2. Install Docker or Podman and run a container from an equivalent base
   image.
3. Compare startup time and idle resource use (`docker stats`, or the host's
   process/memory view) between the VM and the container running similar
   workloads.
4. Note which one has its own kernel and which one shares the host's — the
   architectural difference behind most of what you just measured.

## Related pages

- [Network appliances](/learn/network-appliances) — the physical roles
  (routing, firewalling, load balancing) that NFV reimplements in software.
- [Network functions](/learn/network-functions) — VPNs and IPsec, relevant to
  the connectivity options described above.
- [Network protocols and ports](/learn/network-protocols) — the port-level
  detail behind a security group or access-list rule.

## Sources and further reading

This page was edited from my networking reading notes and checked against:

- [NIST SP 800-145: The NIST Definition of Cloud Computing](https://csrc.nist.gov/pubs/sp/800/145/final)
  — the standard reference definitions for deployment and service models.
- [NIST SP 800-125: Guide to Security for Full Virtualization Technologies](https://csrc.nist.gov/pubs/sp/800/125/final)
  — security considerations for hypervisors and virtual machines.

Provider-specific terminology for security groups, gateways, and connectivity
products changes over time; vendor documentation is authoritative for a
specific platform's current behavior.
