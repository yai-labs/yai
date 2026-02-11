# Engine Cortex v1

R5 defines a deterministic engine-side baseline scaler in C.

## Inputs
- `queue_depth` (required)

## Outputs
- `engine_scale_up`
- `engine_scale_down`

## Runtime behavior
- EWMA + hysteresis + hold timers + cooldown.
- No ML, no random source, no wall-clock driven decisions.
- Tick-driven decisions using fixed `tick_ms`.

## Event payload
Engine emits structured JSON markers in `engine.log`:
- Prefix: `[YAI_CORTEX_EVENT] `
- Fields: `type`, `ws`, `actor`, `reason`, `metrics`, `recommendation`.

Daemon tails `engine.log`, converts markers to event bus emissions, and appends to `events.log`.

## Config
Environment overrides are supported via `YAI_ENGINE_CORTEX_*`:
- `TICK_MS`, `EWMA_ALPHA`, `UP_THRESHOLD`, `DOWN_THRESHOLD`, `PEAK_DELTA`
- `UP_HOLD_MS`, `DOWN_HOLD_MS`, `COOLDOWN_UP_MS`, `COOLDOWN_DOWN_MS`
- `MIN_TARGET`, `MAX_TARGET`, `STEP_UP`, `STEP_DOWN`, `INITIAL_TARGET`

Invalid config fails fast at engine start.
