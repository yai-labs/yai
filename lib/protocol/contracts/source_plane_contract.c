#include <stddef.h>

#include <yai/protocol/contracts/source_plane_contract.h>

static const yai_source_contract_shape_t SHAPES[] = {
    {YAI_SOURCE_CONTRACT_ENROLL, YAI_SOURCE_CONTRACT_TYPE_ENROLL_CALL, YAI_SOURCE_CONTRACT_TYPE_ENROLL_REPLY},
    {YAI_SOURCE_CONTRACT_ATTACH, YAI_SOURCE_CONTRACT_TYPE_ATTACH_CALL, YAI_SOURCE_CONTRACT_TYPE_ATTACH_REPLY},
    {YAI_SOURCE_CONTRACT_EMIT, YAI_SOURCE_CONTRACT_TYPE_EMIT_CALL, YAI_SOURCE_CONTRACT_TYPE_EMIT_REPLY},
    {YAI_SOURCE_CONTRACT_STATUS, YAI_SOURCE_CONTRACT_TYPE_STATUS_CALL, YAI_SOURCE_CONTRACT_TYPE_STATUS_REPLY},
};

const yai_source_contract_shape_t *yai_source_contract_shape(yai_source_contract_operation_t op)
{
  size_t i = 0;
  for (i = 0; i < (sizeof(SHAPES) / sizeof(SHAPES[0])); ++i)
  {
    if (SHAPES[i].op == op)
    {
      return &SHAPES[i];
    }
  }
  return NULL;
}
