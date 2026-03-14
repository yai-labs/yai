# Session-only workspace prompt hook for zsh (LEFT prompt only).
# Usage:
#   source /path/to/yai/tools/dev/yai-prompt.zsh
#   yai_prompt_enable
#   yai_prompt_disable

if [[ -n "${YAI_PROMPT_LOADED:-}" ]]; then
  return 0
fi
typeset -g YAI_PROMPT_LOADED=1

yai_prompt_token_cmd() {
  local script_dir
  script_dir="$(cd -- "$(dirname -- "${(%):-%N}")" && pwd)"
  local fallback="${script_dir}/../bin/yai-ws-token"
  if [[ -x "$fallback" ]]; then
    printf '%s\n' "$fallback"
    return 0
  fi
  local canonical="$HOME/Developer/YAI/yai/tools/bin/yai-ws-token"
  if [[ -x "$canonical" ]]; then
    printf '%s\n' "$canonical"
    return 0
  fi
  if command -v yai-ws-token >/dev/null 2>&1; then
    command -v yai-ws-token
    return 0
  fi
  return 1
}

yai_prompt_segment() {
  local cmd
  cmd="$(yai_prompt_token_cmd)" || return 0
  "$cmd" 2>/dev/null || true
}

prompt_yai_ws() {
  emulate -L zsh
  local tok
  tok="$(yai_prompt_segment)"
  [[ -n "$tok" ]] || return 0
  p10k segment -f 255 -b 35 -t "$tok"
}

yai_prompt_enable() {
  typeset -ga POWERLEVEL9K_LEFT_PROMPT_ELEMENTS
  typeset -ga POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS

  # Kill right-side workspace token unconditionally.
  POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(${POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS:#yai_ws})

  # Keep a single yai_ws on the left, right before vcs when available.
  POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(${POWERLEVEL9K_LEFT_PROMPT_ELEMENTS:#yai_ws})
  if (( ${POWERLEVEL9K_LEFT_PROMPT_ELEMENTS[(I)vcs]} <= ${#POWERLEVEL9K_LEFT_PROMPT_ELEMENTS} )); then
    integer _yai_i=${POWERLEVEL9K_LEFT_PROMPT_ELEMENTS[(I)vcs]}
    POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(
      ${POWERLEVEL9K_LEFT_PROMPT_ELEMENTS[1,$((_yai_i-1))]}
      yai_ws
      ${POWERLEVEL9K_LEFT_PROMPT_ELEMENTS[_yai_i,-1]}
    )
    unset _yai_i
  else
    POWERLEVEL9K_LEFT_PROMPT_ELEMENTS+=(yai_ws)
  fi
}

yai_prompt_disable() {
  typeset -ga POWERLEVEL9K_LEFT_PROMPT_ELEMENTS
  typeset -ga POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS
  POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(${POWERLEVEL9K_LEFT_PROMPT_ELEMENTS:#yai_ws})
  POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(${POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS:#yai_ws})
}
