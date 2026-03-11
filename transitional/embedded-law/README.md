# Embedded Law Transitional Marker

`embedded/law` is legacy/transitional.

During convergence:

- keep compatibility only when strictly required
- migrate governance payloads to `governance/`
- cut runtime loader paths toward canonical governance roots

Exit condition:

- no active runtime dependency on `embedded/law`
- manifests/registry/schema/overlays resolved from `governance/`

Archive note:

- deprecated `embedded-export.manifest.json` is preserved only under
  `transitional/embedded-law/manifests/` for historical traceability.
