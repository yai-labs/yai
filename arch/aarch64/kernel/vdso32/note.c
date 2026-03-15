// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2012-2018 ARM Limited
 *
 * This supplies .note.* sections to go into the PT_NOTE inside the vDSO text.
 * Here we can supply some information useful to userland.
 */

#include <yai/uts.h>
#include <yai/version.h>
#include <yai/elfnote.h>
#include <yai/build-salt.h>

ELFNOTE32("Linux", 0, LINUX_VERSION_CODE);
BUILD_SALT;
