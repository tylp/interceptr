# interceptr

Intercept packets on the network.

## Requirements

User must  set the right capabilities : `sudo setcap 'cap_net_admin=+ep' <path>`.

## Capabilities

- 

## Notes

Use nft to add tables, chains and rules.

A table contains:
- a name
- a type (ip, ip6, arp, bridge...)
- chains

A chain contains:
- a name
- rules
- type (filter, nat, route)
- hooks (input, output, forward, prerouting)

A rule contains:
- criterias (ip, port...)
- actions when criterias are met

Example : A rule that modifies the source port of packets from 127.0.0.1

- `sudo nft add table ip mangle`, where `mangle` is the table name and is associated to `ip` packets.
- `sudo nft add chain ip mangle prerouting { type filter hook prerouting priority -15; }`
- `sudo nft add rule ip mangle prerouting ip saddr 127.0.0.1 tcp sport set 12345`

Example : modify payload using nfqueue
We add a rule for the `mangle` table in the `prerouting` chain that will redirect all `ip` (ipv4) pakcets in the nfqueue `1`.

`sudo nft add rule ip mangle prerouting ip saddr 127.0.0.1 queue num 1`, 

We can then use the nfqueue crate to handle packets in this queue.

Where ``