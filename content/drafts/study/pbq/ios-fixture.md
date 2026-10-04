---
# FIXTURE, NOT A LESSON. Drafted by an agent to show the IOS scenario format
# and to give the matching tests a real file to load. It is not routed, listed,
# or searched. Jeff rewrites or replaces it before anything like it goes into
# content/study/pbq/. The keyword lists have not been checked against a device,
# and the citations have not been checked against what the page teaches.
title: "FIXTURE: bring up an access port"
situation: >
  FIXTURE. A switch port is administratively down. Find it, then bring it up.
dialect: ios
# Group name -> the keywords that compete there. The first token of a command
# is matched against the group named by the step's mode (exec when absent).
# Later tokens are matched against the group named by the accepted tokens
# before them, joined with a hyphen.
keywords:
  exec: [show, configure, copy, ping, traceroute, enable, disable, debug]
  show: [ip, interfaces, vlan, running-config, startup-config, version]
  show-ip: [interface, route, protocols]
  show-ip-interface: [brief]
  configure: [terminal, memory, network]
  config: [interface, hostname, ip, vlan, line, no]
  config-if: [shutdown, switchport, speed, duplex, description, no]
  "no": [shutdown, switchport, speed]
steps:
  - prompt: >
      FIXTURE. List every interface with its address and status on one line each.
    accept:
      - "show ip interface brief"
    explanation: >
      FIXTURE explanation, long enough to pass the length check and nothing more.
    learn_slug: "switching-technologies"
    learn_anchor: "interface-configuration"

  - prompt: >
      FIXTURE. Enter global configuration mode.
    accept:
      - "configure terminal"
    explanation: >
      FIXTURE explanation, long enough to pass the length check and nothing more.
    learn_slug: "switching-technologies"
    learn_anchor: "interface-configuration"

  - prompt: >
      FIXTURE. Select the first gigabit port.
    mode: config
    accept:
      - "interface GigabitEthernet0/1"
    explanation: >
      FIXTURE explanation, long enough to pass the length check and nothing more.
    learn_slug: "switching-technologies"
    learn_anchor: "interface-configuration"

  - prompt: >
      FIXTURE. Bring the port up.
    mode: config-if
    accept:
      - "no shutdown"
    explanation: >
      FIXTURE explanation, long enough to pass the length check and nothing more.
    learn_slug: "switching-technologies"
    learn_anchor: "interface-configuration"
---
