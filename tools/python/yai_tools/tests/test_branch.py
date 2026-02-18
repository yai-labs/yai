import unittest
import os
import sys

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

from yai_tools.workflow.branch import make_branch_name


class TestBranch(unittest.TestCase):
    def test_standard(self):
        b = make_branch_name("feat", "123", "", "root", "hardening-forward")
        self.assertEqual(b, "feat/123-root-hardening-forward")

    def test_meta(self):
        b = make_branch_name("docs", "N/A", "bootstrap", "governance", "tooling-v1")
        self.assertEqual(b, "meta/governance-tooling-v1")

    def test_meta_requires_reason(self):
        with self.assertRaises(ValueError):
            make_branch_name("docs", "N/A", "", "governance", "tooling-v1")

    def test_hotfix_na(self):
        b = make_branch_name("hotfix", "N/A", "incident-123", "root", "rollback-safe")
        self.assertEqual(b, "hotfix/root-rollback-safe")


if __name__ == "__main__":
    unittest.main()
