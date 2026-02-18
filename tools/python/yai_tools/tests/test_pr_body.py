import unittest
import os
import sys

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

from yai_tools._core.text import normalize_issue


class TestNormalizeIssue(unittest.TestCase):
    def test_issue_number(self):
        self.assertEqual(normalize_issue("123"), "#123")
        self.assertEqual(normalize_issue("#123"), "#123")

    def test_na(self):
        self.assertEqual(normalize_issue("N/A"), "N/A")
        self.assertEqual(normalize_issue("na"), "N/A")

    def test_invalid(self):
        with self.assertRaises(ValueError):
            normalize_issue("abc")


if __name__ == "__main__":
    unittest.main()
