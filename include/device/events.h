#pragma once

#ifndef INCLUDE_DEVICE_EVENTS_H
#define INCLUDE_DEVICE_EVENTS_H

#include <device/types.h>

enum yai_device_event_kind {
    YAI_DEVICE_EVENT_ADD = 0,
    YAI_DEVICE_EVENT_REMOVE,
    YAI_DEVICE_EVENT_BIND,
    YAI_DEVICE_EVENT_UNBIND,
    YAI_DEVICE_EVENT_FAULT
};

struct yai_device_event {
    yai_device_id_t device_id;
    enum yai_device_event_kind kind;
};

#endif
