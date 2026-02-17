#!/usr/bin/env python3
import os
import runpy
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TARGET = os.path.join(ROOT, "scripts", "dev", "test_handshake.py")
runpy.run_path(TARGET, run_name="__main__")
