#!/usr/bin/env python3
import os
import sys
import subprocess

MESON = "meson"

def meson_build() -> bool:
    cmd = [MESON, "setup", "build", "--reconfigure"],
    result = subprocess.run(
        [MESON, "setup", "build", "--reconfigure"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True
    )
    if result:
        print(f"  \x1b[1;32mCMD SUCCESS\x1b[1;0m    {cmd}")
    else:
        print(f"  \x1b[1;31mCMD FAIL\x1b[1;0m    {cmd}")
    return result

def setup_gtest() -> bool:
    gtest_path = "./subprojects/gtest.wrap"
    if os.path.isfile(gtest_path):
        print(f"  \x1b[1;33m{gtest_path}\x1b[1;0m file exists")
        cmd        = [MESON, "wrap", "update", "gtest"]
    else:
        print(f"  \x1b[1;34m{gtest_path}\x1b[1;0m file does not exists")
        cmd        = [MESON, "wrap", "install", "gtest"]
    result = subprocess.run(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True
    )
    if result:
        print(f"  \x1b[1;32mCMD SUCCESS\x1b[1;0m    {cmd}")
    else:
        print(f"  \x1b[1;31mCMD FAIL\x1b[1;0m       {cmd}")
    return result

def main():
    if not setup_gtest():
        exit(1)
    if not meson_build():
        exit(1)

if __name__ == "__main__":
    main()
