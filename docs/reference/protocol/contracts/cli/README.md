# CLI Contracts

`contracts/cli/` defines CLI-facing contract notes and binding constraints.

The CLI surface remains stable even when internal runtime packaging changes.

## Alignment rule

- Public command surface stability is prioritized.
- Internal topology migration (`core/exec/brain`) must not force unnecessary CLI breakage.
- Legacy runtime names may appear as command topics during migration, but are not ontology-defining.

## Governance parsing surface

The deterministic governance ingestion pipeline is exposed through:

- `yai govern gov source ...`
- `yai govern gov parse ...`
- `yai govern gov parsed ...`
- `yai govern gov normalize ...`
- `yai govern gov normalized ...`
- `yai govern gov build ...`
- `yai govern gov candidate ...`
- `yai govern gov validate ...`
- `yai govern gov diff ...`
- `yai govern gov status ...`
