"""Summarize `sample-host.sh` rows (CI #603 DOD-7a evidence, never a gate).

Prints one line to the log and, when GITHUB_STEP_SUMMARY is set, appends it
there: cores, samples, CPU busy share (mean, p90, share of samples at or above
95%), minimum available memory, peak swap in use and peak largest resident set.
"""
import os
import sys


def summarize(lines):
    header, rows = lines[0], [line.rstrip("\n").split("\t") for line in lines[1:] if line.strip()]
    cores = header.rsplit("cores=", 1)[-1].strip() if "cores=" in header else "?"

    def column(index):
        out = []
        for row in rows:
            try:
                out.append(float(row[index]))
            except (IndexError, ValueError):
                pass
        return out
    cpu, avail, swap, top = column(1), column(2), column(3), column(4)
    cpu.sort()
    parts = [f"cores {cores}", f"samples {len(rows)}"]
    if cpu:
        parts.append(f"CPU busy mean {sum(cpu) / len(cpu):.0f}% p90 {cpu[int(0.9 * (len(cpu) - 1))]:.0f}% "
                     f">=95% in {100 * sum(v >= 95 for v in cpu) / len(cpu):.0f}% of samples")
    if avail:
        parts.append(f"min available {min(avail) / 1048576:.2f} GiB")
    if swap:
        parts.append(f"peak swap {max(swap) / 1048576:.2f} GiB")
    if top:
        parts.append(f"peak largest RSS {max(top) / 1048576:.2f} GiB")
    return "host resources (#603 DOD-7a evidence): " + ", ".join(parts)


def main(argv):
    try:
        with open(argv[1], encoding="utf-8") as handle:
            line = summarize(handle.readlines())
    except (OSError, IndexError) as error:
        print(f"host resources: no samples ({error})")
        return 0
    print(line)
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a", encoding="utf-8") as handle:
            handle.write(line + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
