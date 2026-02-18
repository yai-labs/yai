Milestone Pack: `MP-ROOT-HARDENING-0.1.1`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.1 — Root = Byte-Perfect Router`)
Owner: runtime

Objective:
- Make Root a deterministic validate-forward-relay boundary with indestructible append-only logging.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: deterministic response frame on every reject (no timeout-as-error).
- Logging: mandatory append-only `~/.yai/run/root/root.log` with required fields.

Repo Split:
- `yai`: Root transport validation, byte-perfect forwarding, deterministic reject and append-only logging.
- `yai-cli`: optional harness improvements for negative routing cases and log assertions.

Evidence Plan (minimum):
- Positive cases:
  - `yai root ping` succeeds with byte-stable request/response envelope.
  - Valid control command is forwarded and relayed without envelope mutation.
- Negative cases:
  - Wrong `magic` and wrong `version` return error frame then close.
  - `payload_len > max` returns deterministic reject and traceable log line.

Compatibility Classification:
- Type: A
- Rationale: routing semantics are hardened without requiring immediate client wire changes.
- Upgrade path: existing conformant clients continue to operate.

Definition of Done:
- [ ] Root performs validate + forward + relay without mutating envelope or payload.
- [ ] Every reject returns deterministic error frame; no silent close.
- [ ] `~/.yai/run/root/root.log` is created if missing and always append-only.
- [ ] Evidence and logs are attached in PR and CI.
