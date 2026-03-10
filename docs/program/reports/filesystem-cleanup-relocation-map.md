# Filesystem Cleanup Relocation Map (DP-14)

| old_path | new_path | category | reason | data-plane replacement |
|---|---|---|---|---|
| `law/transitional/domain-family-seed/**` | `../archive_tmp/data-plane-filesystem/law/transitional/domain-family-seed/**` | FS-C4 -> FS-C6 | transitional residue removed from operational path | DB-first runtime record domains (`events`, `governance`, `authority`, `artifacts`, `enforcement`, `graph`) |

## Notes
- Archive is non-operational and non-authoritative.
- Runtime/SDK/CLI do not use archive paths as primary read source.
