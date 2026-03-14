#pragma once

#ifndef INCLUDE_SYS_BUILD_H
#define INCLUDE_SYS_BUILD_H

struct yai_build_info {
    const char *version;
    const char *git_commit;
    const char *build_time;
};

#endif
