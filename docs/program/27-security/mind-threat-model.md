# Threat Model — YAI Mind

## Assets

- prompts/context (may contain sensitive info)
- embeddings and derived representations
- memory graph contents (nodes/edges)
- provider configurations
- logs and evidence artifacts

## Threats

1) Supply-chain compromise
- vulnerable dependencies
- malicious transitive crates

2) Provider-driven injection
- crafted outputs that poison memory or planning
- prompt/context injection via retrieved content

3) Data exfiltration
- secrets in logs
- accidental commit of datasets or DB files

4) Authority bypass
- Mind executing effects without explicit authority envelope
- implicit side-effects inside provider clients

## Mitigations

- CI lint/test gates
- strict “no secrets in repo” policy
- sanitize logs; minimal logging defaults
- enforce authority boundary at integration points
