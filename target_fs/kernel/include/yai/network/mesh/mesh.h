#pragma once

#include <stdint.h>

#include <yai/net/discovery.h>
#include <yai/net/enrollment.h>
#include <yai/net/conflict.h>
#include <yai/net/coordination.h>
#include <yai/net/replay.h>
#include <yai/net/client.h>
#include <yai/net/overlay.h>

typedef struct {
  int initialized;
  yai_mesh_peer_registry_t registry;
  yai_mesh_enrollment_state_t enrollment;
  yai_mesh_conflict_state_t conflict;
  yai_mesh_coordination_state_t coordination;
  yai_mesh_replay_state_t replay;
  yai_mesh_transport_state_t transport;
} yai_mesh_state_t;
