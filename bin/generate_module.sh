#!/usr/bin/env bash
set -euo pipefail

TEMPLATE="stubs/game_module.stub"
BASE_DIR="src/modules"

usage() {
  echo "Usage: $0 <module_path>"
  echo "Example: $0 world::map::render"
  exit 1
}

[[ $# -eq 1 ]] || usage
[[ -f "$TEMPLATE" ]] || { echo "Template not found at: $TEMPLATE" >&2; exit 2; }

MODULE_PATH_INPUT="$1"

MODULE_PARTS_STR="${MODULE_PATH_INPUT//::/ }"
read -r -a MODULE_PARTS <<< "$MODULE_PARTS_STR"

to_title_case() {
  local s="$1" out="" seg first rest
  IFS='_' read -r -a segs <<< "$s"
  for seg in "${segs[@]}"; do
    [[ -z "$seg" ]] && continue
    first="${seg:0:1}"
    rest="${seg:1}"
    out+="${first^^}${rest,,}"
  done
  printf "%s" "$out"
}

escape_for_sed_repl() {
  local s="$1"
  s="${s//\\/\\\\}"  
  s="${s//&/\\&}"    
  s="${s//\//\\/}"   
  printf "%s" "$s"
}

current_path="$BASE_DIR"
created=()

for part in "${MODULE_PARTS[@]}"; do
  folder="${part,,}"
  current_path="$current_path/$folder"
  mkdir -p "$current_path"

  mod_file="$current_path/mod.rs"
  if [[ ! -f "$mod_file" ]]; then
    name_title="$(to_title_case "$part")"
    name_title_escaped="$(escape_for_sed_repl "$name_title")"
    sed "s/{{name}}/${name_title_escaped}/g" "$TEMPLATE" > "$mod_file"
    created+=("$mod_file")
  fi
done

if ((${#created[@]} == 0)); then
  echo "All modules already existed. Nothing to do."
else
  echo "Created the following files:"
  for f in "${created[@]}"; do
    echo "  - $f"
  done
fi
