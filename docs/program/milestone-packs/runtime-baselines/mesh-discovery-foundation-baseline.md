# Mesh Discovery Foundation Baseline (MF-1)

## Objective

Validate governed discovery semantics before coordination/trust waves.

## Baseline checks

1. Confirm node advertisements exist with role + mesh + owner anchors.
2. Confirm owner discovery and peer discovery are inspectable as distinct views.
3. Confirm bootstrap descriptor/seed references are present.
4. Confirm discovery scope can restrict visibility by role/workspace/mesh.
5. Confirm no discovery surface implies enrollment or trust completion.

## Expected outcomes

- Node visibility is available for topology/bootstrap.
- Discovery state is explicit and inspectable.
- Enrollment/trust remain subsequent planes.
- Authority remains owner-side.

## Anti-drift assertions

- Discovered peer != trusted peer.
- Visible peer != enrolled peer.
- Discovery endpoint reachability != delegated permission.
