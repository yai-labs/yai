# Runbook C — Digital Governed Flow

## Goal

Demonstrate governed retrieve/egress/publication/distribution with sink-sensitive control.

## Steps

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
./dist/bin/yai up
./dist/bin/yai ws create ws_digital_demo
./dist/bin/yai ws set ws_digital_demo
./dist/bin/yai ws domain set --family digital --specialization remote-publication

# deny: missing authority contract
./dist/bin/yai ws run digital.publish sink=external_untrusted artifact=bundle-v1

# quarantine: authority but untrusted sink
./dist/bin/yai ws run digital.publish sink=external_untrusted contract=approved artifact=bundle-v1

# review_required/allow: trusted sink + contract
./dist/bin/yai ws run digital.publish sink=internal_trusted contract=approved artifact=bundle-v1 destination=ops_portal

./dist/bin/yai ws domain set --family digital --specialization remote-retrieval
./dist/bin/yai ws run digital.retrieve source=trusted_repo sink=internal

./dist/bin/yai ws domain set --family digital --specialization artifact-distribution
./dist/bin/yai ws run digital.distribute artifact=bundle-v1

./dist/bin/yai ws policy effective
./dist/bin/yai ws debug resolution
./dist/bin/yai ws inspect

./dist/bin/yai ws unset
./dist/bin/yai ws destroy ws_digital_demo
```

## Acceptance

- digital specialization distinctions are visible
- sink-sensitive deny/quarantine/review behavior is visible
- inspect/policy/debug expose digital summaries and evidence traces
