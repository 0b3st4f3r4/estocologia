#!/usr/bin/env bash
# Assembles the book site into a directory (default _site): the Docsify page, the chapter files as written in
# book/, and a sidebar generated from their part, chapter and appendix headings, so it cannot drift from the text.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
out="${1:-$root/_site}"
rm -rf "$out"
mkdir -p "$out"
cp "$root/site/index.html" "$out/"
cp "$root/site/CNAME" "$out/"
touch "$out/.nojekyll"
files=("00-opening.md")
for n in $(seq -w 1 21); do files+=("ch$n.md"); done
files+=("appendices.md")
sidebar="$out/_sidebar.md"
: > "$sidebar"
for f in "${files[@]}"; do
  cp "$root/book/$f" "$out/$f"
  if [ "$f" = "00-opening.md" ]; then
    echo "- [Preface](/)" >> "$sidebar"
    continue
  fi
  part="$(sed -n '1s/^# //p' "$root/book/$f")"
  if [ -n "$part" ] && [ "$f" != "appendices.md" ]; then
    echo "- **${part}**" >> "$sidebar"
  fi
  if [ "$f" = "appendices.md" ]; then
    echo "- [**Appendices**](appendices.md)" >> "$sidebar"
  else
    title="$(grep -m1 '^## Chapter' "$root/book/$f" | sed 's/^## //')"
    echo "  - [${title}](${f%.md})" >> "$sidebar"
  fi
done
echo "site assembled in $out: ${#files[@]} pages"
