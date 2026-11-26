"""The compiler-semantics IDs relock-identity reads (#603, #624).

v0.5's relocker matched the ID as bare hex, so a `blake3:` ID read as none and
the forward merge's regeneration kept every lock it should have moved
("relocked 0, kept 275", run 45581677739). These tests run the real helper.
"""
from pathlib import Path
import shutil
import subprocess
import unittest

SCRIPTS = Path(__file__).resolve().parent
HEX = "70995069e14da8c7eac9c3594b1b4a2820da8d8b3b5adbdf9de982adb6c2b20a"


def ids(text, prefix=""):
    done = subprocess.run(["bash", "-c", f'source "{(SCRIPTS / "semantics-id.sh").as_posix()}"; semantics_ids "$1"', "ids", prefix],
                          input=text, capture_output=True, text=True, check=True)
    return done.stdout.splitlines()


@unittest.skipUnless(shutil.which("bash"), "needs bash")
class SemanticsIds(unittest.TestCase):
    def test_bare_and_blake3_ids_are_read(self):
        self.assertEqual(ids(f'compiler_semantics = "{HEX}"\n'), [HEX])
        self.assertEqual(ids(f'compiler_semantics = "blake3:{HEX}"\n'), ["blake3:" + HEX])

    def test_a_diff_side_is_read_by_its_prefix_only(self):
        diff = f'-compiler_semantics = "blake3:{HEX}"\n+compiler_semantics = "blake3:{"0" * 64}"\n'
        self.assertEqual(ids(diff, "-"), ["blake3:" + HEX])
        self.assertEqual(ids(diff, "+"), ["blake3:" + "0" * 64])
        self.assertEqual(ids(diff), [])

    def test_anything_else_is_no_id(self):
        for value in ("", HEX[:-1], HEX + "0", HEX.upper(), "sha512:" + HEX, "blake3:", "blake3:" + HEX[:-1], " " + HEX):
            with self.subTest(value=value):
                self.assertEqual(ids(f'compiler_semantics = "{value}"\n'), [])

    def test_the_relocker_reads_every_id_through_the_helper(self):
        text = (SCRIPTS / "relock-identity.sh").read_text()
        self.assertIn("source .github/scripts/semantics-id.sh", text)
        self.assertNotIn("compiler_semantics = \"\\([0-9a-f]*\\)\"", text, "a bare-hex reading of the ID")
        self.assertEqual(text.count("semantics_ids"), 3)
        self.assertIn("moves a compiler-semantics ID this step does not read", text)


if __name__ == "__main__":
    unittest.main()
