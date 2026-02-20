import os
import sys
import unittest

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

from yai_tools.cli import _label_color


class TestLabelPalette(unittest.TestCase):
    def test_prefix_palette(self):
        self.assertEqual(_label_color("phase:0.1.2"), "1d76db")
        self.assertEqual(_label_color("track:contract-baseline-lock"), "0e8a16")
        self.assertEqual(_label_color("class:A"), "fbca04")
        self.assertEqual(_label_color("work-type:docs"), "c5def5")
        self.assertEqual(_label_color("type:ci"), "bfd4f2")
        self.assertEqual(_label_color("area:engine"), "0052cc")

    def test_exact_and_fallback_palette(self):
        self.assertEqual(_label_color("runbook"), "1d76db")
        self.assertEqual(_label_color("governance"), "5319e7")
        self.assertEqual(_label_color("mp-closure"), "8250df")
        self.assertEqual(_label_color("label-non-mappata"), "d4a72c")


if __name__ == "__main__":
    unittest.main()
