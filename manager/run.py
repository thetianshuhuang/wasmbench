"""test script."""

import subprocess
import os
import sys


BASE_DIR = "../benchmarks/target/wasm32-wasi/debug"
RUNNER_DIR = "../runtime/target/debug/runtime"


subprocess.run([RUNNER_DIR, "Singlepass", "Universal"] + [os.path.join(BASE_DIR, s + ".wasm") for s in sys.argv[1:]])
