#!/usr/bin/env bash
# Evidence for #603 DOD-7a, never a gate: one row every five seconds of the
# host's CPU busy share, available memory, swap in use and the largest
# resident set, written to $1 until killed. Linux reads /proc; macOS reads
# top, vm_stat and sysctl. A failed probe writes an empty field; it never
# stops the sampling or affects the tests it watches.
out=${1:?the samples file}
cores=$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 0)
printf 'time\tcpu_busy_pct\tavail_kib\tswap_used_kib\ttop_rss_kib\tcores=%s\n' "$cores" > "$out"
previous=
while :; do
  now=$(date +%s)
  if [ -r /proc/stat ]; then
    read -r _ user nice system idle iowait irq softirq steal _ < /proc/stat
    total=$((user + nice + system + idle + iowait + irq + softirq + steal))
    busy=$((total - idle - iowait))
    cpu=
    if [ -n "$previous" ]; then
      set -- $previous
      dt=$((total - $1)); db=$((busy - $2))
      [ "$dt" -gt 0 ] && cpu=$((100 * db / dt))
    fi
    previous="$total $busy"
    avail=$(awk '/^MemAvailable:/ {print $2}' /proc/meminfo)
    swap=$(awk '/^SwapTotal:/ {t=$2} /^SwapFree:/ {f=$2} END {print t-f}' /proc/meminfo)
  else
    idle=$(top -l 2 -n 0 -s 1 2>/dev/null | awk -F'[ ,%]+' '/CPU usage/ {for (i = 1; i <= NF; i++) if ($i == "idle") v = $(i - 1)} END {print v}')
    cpu=; [ -n "$idle" ] && cpu=$(awk -v i="$idle" 'BEGIN {printf "%d", 100 - i}')
    page=$(sysctl -n hw.pagesize 2>/dev/null || echo 4096)
    avail=$(vm_stat 2>/dev/null | awk -v p="$page" '/Pages free|Pages inactive|Pages speculative/ {gsub("\.", "", $NF); s += $NF} END {printf "%d", s * p / 1024}')
    swap=$(sysctl -n vm.swapusage 2>/dev/null | awk '{for (i = 1; i <= NF; i++) if ($i == "used") {v = $(i + 2); sub("M", "", v); printf "%d", v * 1024}}')
  fi
  top=$( (ps -A -o rss= 2>/dev/null || true) | sort -n | tail -1 | tr -d ' ')
  printf '%s\t%s\t%s\t%s\t%s\n' "$now" "${cpu:-}" "${avail:-}" "${swap:-}" "${top:-}" >> "$out"
  sleep 5
done
