"""Tests of the host-resource summary (CI #603 DOD-7a evidence)."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import summarize_host_samples as summary  # noqa: E402

HEADER = "time\tcpu_busy_pct\tavail_kib\tswap_used_kib\ttop_rss_kib\tcores=4\n"


class Summarize(unittest.TestCase):
    def test_statistics(self):
        rows = [HEADER] + [f"{t}\t{c}\t{a}\t{s}\t{r}\n" for t, c, a, s, r in (
            (1, "", 8388608, 0, 1048576), (2, 100, 4194304, 1048576, 2097152),
            (3, 50, 2097152, 0, 3145728))]
        line = summary.summarize(rows)
        self.assertIn("cores 4", line)
        self.assertIn("samples 3", line)
        self.assertIn("CPU busy mean 75%", line)
        self.assertIn("min available 2.00 GiB", line)
        self.assertIn("peak swap 1.00 GiB", line)
        self.assertIn("peak largest RSS 3.00 GiB", line)

    def test_empty_and_missing_samples_never_fail(self):
        self.assertIn("samples 0", summary.summarize([HEADER]))
        self.assertEqual(summary.main(["x", "/nonexistent/samples.tsv"]), 0)


if __name__ == "__main__":
    unittest.main()
