#pragma once

#ifndef INCLUDE_SECURITY_AUDIT_H
#define INCLUDE_SECURITY_AUDIT_H

struct yai_security_audit_record {
    const char *action;
    const char *subject;
    int allowed;
};

#endif
