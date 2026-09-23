#!/usr/bin/env bash

# Script to convert the output from Divan (Rust benchmark library)
# to a CSV format.
#
# Usage: cargo bench --quiet --bench {name} | ./bench2csv.sh

set -euo pipefail

awk '
BEGIN {
  OFS = ","
  print "size,fastest,slowest,median,mean,samples,iters"
}

function trim(s) {
  sub(/^[[:space:]]+/, "", s)
  sub(/[[:space:]]+$/, "", s)
  return s
}

function to_ms(s,    value, unit) {
  if (match(s, /^([0-9]+(\.[0-9]+)?)[[:space:]]*(ns|µs|μs|us|ms)$/, m)) {
    value = m[1] + 0
    unit = m[3]

    if (unit == "ns") return value / 1000000
    if (unit == "µs" || unit == "μs" || unit == "us") return value / 1000
    return value
  }

  return -1
}

function fmt_ms(s,    v) {
  v = to_ms(s)
  if (v == int(v)) return sprintf("%.0f", v)
  return sprintf("%.6f", v)
}

{
  line = $0

  # skip the header line
  if (line ~ /^powersort_bench/) next

  # only process benchmark rows
  if (line !~ /[│]/) next

  n = split(line, parts, /│/)

  if (n < 6) next

  # left side: "100000     313.8 µs"
  left = trim(parts[1])
  sub(/^[[:space:]]*[-─├╰]+[[:space:]]*/, "", left)

  if (match(left, /^([0-9]+)[[:space:]]+(.+)$/, m)) {
    size = m[1]
    fastest = fmt_ms(trim(m[2]))
  } else {
    next
  }

  slowest = fmt_ms(trim(parts[2]))
  median  = fmt_ms(trim(parts[3]))
  mean    = fmt_ms(trim(parts[4]))
  samples = trim(parts[5])
  iters   = trim(parts[6])

  print size, fastest, slowest, median, mean, samples, iters
}
' "$@"