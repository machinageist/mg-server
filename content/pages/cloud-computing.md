---
title: "Cloud computing concepts for networking"
date: 2026-07-24
summary: "NFV, virtual private clouds, cloud-native traffic controls, connectivity options, deployment models, and how cloud resources scale."
tags: [education, networking, cloud, nfv, virtualization, network-plus]
---

## Overview

Cloud computing moves network functions from dedicated hardware onto shared,
virtualized infrastructure. The jobs described on [network
appliances](/learn/network-appliances) — routing, firewalling, load balancing,
and storage — still exist. In a cloud environment, virtual machines,
containers, and managed services do that work on a provider's physical
infrastructure instead of a chassis in your rack.

## Network function virtualization

**Network function virtualization (NFV)** replaces purpose-built network
hardware with software on general-purpose infrastructure. It depends on two
main building blocks:

- A **virtual machine (VM)** emulates a complete computer and runs its own
  operating system on a hypervisor. Several isolated VMs can share one physical
  host.
- A **container** packages an application and its dependencies but shares the
  host operating system's kernel. Containers are lighter and usually start
  faster than VMs, but the shared kernel means less isolation. Docker is the
  most common container runtime; its engine is open source, though Docker
  Desktop is not. Podman is an alternative that runs containers without a root
  daemon.

A **virtual appliance** packages a network function, such as a firewall,
router, or load balancer, to run in a VM or container rather than on dedicated
hardware.

NFV architecture has three parts:

- The **virtualized network function (VNF)** does the network job.
- The **NFV infrastructure (NFVi)** supplies its compute, storage, and
  networking resources.
- **Management and orchestration (MANO)** deploys, scales, and coordinates VNFs
  across that infrastructure.

## Virtual private clouds

A **virtual private cloud (VPC)** is a logically isolated part of a cloud
provider's network for one customer's resources. A VPC can contain several
subnets, usually separated by exposure:

- A **public subnet** has a route to an internet gateway and can host
  internet-facing services.
- A **private subnet** has no direct route to the internet gateway. Its
  resources are not publicly reachable by default. A NAT gateway, VPN, or
  bastion host can still provide controlled connectivity, so "private" describes
  the routing design rather than absolute isolation.

Databases and internal application servers often go in private subnets so they
are not directly addressable from outside the VPC.

## Traffic controls: security groups and access lists

Cloud platforms usually provide traffic policy at both the network-interface
and subnet levels. The names and behavior differ by provider:

- AWS **Security Groups**, Azure **Network Security Groups (NSGs)**, and Oracle
  Cloud Infrastructure **Network Security Groups (NSGs)** attach to network
  interfaces. They are stateful by default: if a request is allowed, its return
  traffic is allowed automatically.
- AWS **Network ACLs** and OCI **Security Lists** apply to subnets. AWS Network
  ACLs are stateless, so they need rules in both directions. OCI Security Lists
  can be stateful or stateless.

Both layers can filter on source and destination IP, port, and protocol. A web
server might allow inbound TCP 80 and 443 and deny other inbound traffic by
default. "Network Security List" is not standard terminology for a major
provider; OCI calls its construct a **Security List**.

## Cloud gateways

A **cloud gateway** connects cloud resources to the internet or another
network. Two common types are:

- An **internet gateway**, which routes traffic between a VPC's public subnets
  and the internet.
- A **NAT gateway**, which lets systems in a private subnet start outbound
  internet connections without accepting unsolicited inbound connections.

## Connecting to the cloud

There are two common ways to connect an on-premises network to cloud resources:

- **Direct or dedicated connections** are private links leased from a provider.
  They give more consistent latency and throughput than a path over the public
  internet, but cost more. They are **not encrypted by default**, so encryption
  must be added when needed.
- **Site-to-site VPNs** create an encrypted tunnel over the internet. They are
  cheaper and quicker to provision than a dedicated circuit, but their
  performance depends on the internet path.

## Cloud deployment models

- **Public cloud** — a cloud service provider (CSP) hosts many customers on
  shared infrastructure.
- **Private cloud** — one organization uses dedicated infrastructure, either
  owned or leased as single-tenant hardware. It costs more but can simplify
  isolation and compliance requirements.
- **Hybrid cloud** — an organization keeps some workloads on-premises and runs
  others with a CSP, with a network connection between them.
- **Community cloud** — organizations with shared requirements, such as an
  industry standard or regulatory framework, use infrastructure designed
  around those requirements.

## Cloud service models

- **Infrastructure as a Service (IaaS)** provides virtual compute, storage, and
  networking. The customer manages the operating system and everything above
  it.
- **Platform as a Service (PaaS)** adds the operating system, runtime, and
  supporting application services to what the provider manages, so the customer
  deploys application code rather than servers.
- **Software as a Service (SaaS)** provides a complete application, usually
  through a browser and subscription. Microsoft 365 is one example.

Moving from IaaS toward SaaS gives the provider more operational responsibility
and gives the customer less direct control. The customer is still responsible
for how its data is used in the service.

## Scalability, elasticity, and multitenancy

- **Scalability** is the ability to add or remove resources as demand changes.
  Capacity may be planned and provisioned in advance.
- **Elasticity** is automatic, near-real-time scaling in response to current
  demand. Resources scale out under load and back in when demand falls.
- **Multitenancy** means several customers share the same physical
  infrastructure while their workloads remain logically isolated.

## Suggested practice: compare a VM and a container locally

You can see the main VM/container difference without a cloud account:

1. Install a local hypervisor (VirtualBox or KVM/libvirt) and boot a small Linux
   VM.
2. Install Docker or Podman and run a container from a similar base image.
3. Compare startup time and idle resource use with `docker stats` or the host's
   process and memory tools.
4. Check which environment has its own kernel and which shares the host kernel.
   That difference explains much of what you measured.

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

Provider terminology and product behavior change. Check the current vendor
documentation when working with a specific platform.
