"""test script."""

import subprocess
import os
import json
import sys
from tqdm import tqdm

import cpuinfo


class Benchmarks:

    def __init__(
            self, targets=[], compiler="Cranelift", engine="Universal",
            base_dir="../benchmarks/target/wasm32-wasi/release",
            runtime="../runtime/target/debug/runtime"):
        self.targets = {
            t[0]: "{}:{}:{}".format(
                os.path.join(base_dir, t[0] + ".wasm"), t[1], t[2])
            for t in targets
        }
        self.compiler = compiler
        self.engine = engine
        self.runtime = runtime

    def evaluate_once(self):
        args = (
            [self.runtime, self.compiler, self.engine] 
            + list(self.targets.values()))
        stdout = subprocess.run(args, capture_output=True).stdout
        d = json.loads(stdout)
        return {k: d[v] for k, v in self.targets.items()}

    def evaluate(self, save=None, repeat=1):
        evaluations = [self.evaluate_once() for _ in tqdm(range(repeat))]

        result = {
            k1: {
                k2: [r[k1][k2] for r in evaluations]
                for k2 in evaluations[0][k1]
            } for k1 in evaluations[0]
        }

        if save:
            result["cpuinfo"] = cpuinfo.get_cpu_info()
            with open(save, 'w') as f:
                json.dump(result, f)

        return result


if __name__ == '__main__':

    Benchmarks([
        ("bogo", 25, 100000),
        ("blake2", 25, 1000),
        ("life", 25, 20),
        ("exponential", 25, 10)
    ]).evaluate(
        save="../results/{}.json".format(sys.argv[1]),
        repeat=int(sys.argv[2]))
