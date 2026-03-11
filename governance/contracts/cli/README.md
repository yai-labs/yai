# CLI Contracts

`contracts/cli/` defines CLI-facing contract notes and binding constraints.

The CLI surface remains stable even when internal runtime packaging changes.

## Alignment rule

- Public command surface stability is prioritized.
- Internal topology migration (`core/exec/brain`) must not force unnecessary CLI breakage.
- Legacy runtime names may appear as command topics during migration, but are not ontology-defining.

## Governance parsing surface

The deterministic governance ingestion pipeline is exposed through:

- `yai law gov source ...`
- `yai law gov parse ...`
- `yai law gov parsed ...`
- `yai law gov normalize ...`
- `yai law gov normalized ...`
- `yai law gov build ...`
- `yai law gov candidate ...`
- `yai law gov validate ...`
- `yai law gov diff ...`
- `yai law gov status ...`
