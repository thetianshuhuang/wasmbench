"""test script."""

import subprocess
import os
import json
import sys

import cpuinfo


class Benchmarks:

    def __init__(
            self, targets=[], compiler="Cranelift", engine="Universal",
            base_dir="../benchmarks/target/wasm32-wasi/debug",
            runtime="../runtime/target/debug/runtime"):
        self.targets = {
            t[0]: "{}:{}:{}".format(
                os.path.join(base_dir, t[0] + ".wasm"), t[1], t[2])
            for t in targets
        }
        self.compiler = compiler
        self.engine = engine
        self.runtime = runtime

    def evaluate(self, save=None):
        stdout = subprocess.run(
            [self.runtime, self.compiler, self.engine] 
                + list(self.targets.values()),
            capture_output=True).stdout
        d = json.loads(stdout)
        result = {k: d[v] for k, v in self.targets.items()}

        if save:
            result["cpuinfo"] = cpuinfo.get_cpu_info()
            with open(save, 'w') as f:
                json.dump(result, f)

        return result


if __name__ == '__main__':

    Benchmarks([
        ("bogo", 25, 1000),
        ("blake2", 25, 1000),
        ("life", 25, 10)
    ]).evaluate(save="../results/{}.json".format(sys.argv[1]))
