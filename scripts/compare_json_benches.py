#!/usr/bin/env python3
"""
Parse `cargo bench -p benches bench_serde` output and compare JSON
performance across fhirbolt, helios main, and helios serde-context.

Usage:
    python scripts/compare_json_benches.py bench-output.txt

If no file is supplied (or "-" is given) the script reads from stdin, so you
can pipe bench output directly:

    RUST_MIN_STACK=33554432 cargo bench -p benches | ./scripts/compare_json_benches.py
"""

from __future__ import annotations

import argparse
import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Dict, Iterable, Optional, Tuple

TIME_LINE_PATTERN = re.compile(r"time:\s*\[([^\]]+)\]")
VALUE_PATTERN = re.compile(r"([\d,]*\.?\d+)\s*(ns|us|\u00b5s|\u03bcs|ms|s)")

RUNNER_LABELS = {
    "fhirbolt": "fhirbolt",
    "helios-main": "helios main",
    "helios-serde-context": "helios serde-context",
}
RUNNER_ORDER = ["fhirbolt", "helios-main", "helios-serde-context"]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Compare JSON bench results for fhirbolt vs helios crates.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("input", nargs="?", default="-", help="Bench output path or '-' for stdin")
    parser.add_argument(
        "--baseline",
        choices=RUNNER_ORDER,
        default="fhirbolt",
        help="Runner used as the comparison baseline (default: fhirbolt)",
    )
    return parser.parse_args()


def read_source(path: str) -> str:
    if path == "-":
        return sys.stdin.read()
    return Path(path).read_text()


def parse_time_block(block: str) -> Optional[float]:
    matches = VALUE_PATTERN.findall(block)
    if len(matches) < 2:
        return None
    # Criterion prints [lower estimate, median, upper estimate]
    median_value, median_unit = matches[1]
    return to_nanoseconds(median_value, median_unit)


def to_nanoseconds(value: str, unit: str) -> float:
    unit = unit.lower().replace("\u03bc", "\u00b5")
    value = float(value.replace(",", ""))
    factors = {
        "ns": 1.0,
        "us": 1_000.0,
        "\u00b5s": 1_000.0,
        "ms": 1_000_000.0,
        "s": 1_000_000_000.0,
    }
    if unit not in factors:
        raise ValueError(f"Unknown unit '{unit}' in bench output")
    return value * factors[unit]


def classify_benchmark(label: str) -> Optional[Tuple[str, str]]:
    if not label.startswith("json/"):
        return None

    name = label[len("json/") :].strip()
    name_lower = name.lower()

    if "element" in name_lower:
        return None

    runner = "fhirbolt"
    if "(helios main)" in name_lower:
        runner = "helios-main"
    elif "(helios serde-context)" in name_lower:
        runner = "helios-serde-context"

    operation = name.split("(")[0].strip()
    if not operation:
        return None

    return operation, runner


def collect_results(lines: Iterable[str]) -> Dict[str, Dict[str, float]]:
    results: Dict[str, Dict[str, float]] = defaultdict(dict)
    current_label: Optional[str] = None

    for line in lines:
        stripped = line.strip()
        if stripped.startswith("json/"):
            current_label = stripped
            continue

        if current_label is None:
            continue

        time_match = TIME_LINE_PATTERN.search(line)
        if not time_match:
            continue

        median_ns = parse_time_block(time_match.group(1))
        if median_ns is None:
            current_label = None
            continue

        classified = classify_benchmark(current_label)
        current_label = None
        if not classified:
            continue

        operation, runner = classified
        previous = results[operation].get(runner)
        if previous is None or median_ns < previous:
            results[operation][runner] = median_ns

    return results


def format_time(ns_value: float) -> str:
    if ns_value < 1_000:
        return f"{ns_value:7.2f} ns"
    if ns_value < 1_000_000:
        return f"{ns_value / 1_000:7.3f} us"
    if ns_value < 1_000_000_000:
        return f"{ns_value / 1_000_000:7.3f} ms"
    return f"{ns_value / 1_000_000_000:7.3f} s"


def print_report(results: Dict[str, Dict[str, float]], baseline_runner: str) -> None:
    if not results:
        print("No matching JSON benchmarks found in the provided output.", file=sys.stderr)
        sys.exit(1)

    baseline_label = RUNNER_LABELS[baseline_runner]
    header = f"{'Operation':<34} {'Runner':<22} {'Median':>12} {f'vs {baseline_label}':>13}"
    print("JSON benchmark comparison (median estimates)")
    print(header)
    print("-" * len(header))

    for operation in sorted(results.keys()):
        operation_results = results[operation]
        baseline_value = operation_results.get(baseline_runner)
        first_line = True

        for runner in RUNNER_ORDER:
            if runner not in operation_results:
                continue

            value = operation_results[runner]
            ratio = "-"
            if runner == baseline_runner:
                ratio = "1.00x"
            elif baseline_value:
                ratio = f"{value / baseline_value:>6.2f}x"

            op_label = operation if first_line else ""
            first_line = False
            print(
                f"{op_label:<34} "
                f"{RUNNER_LABELS[runner]:<22} "
                f"{format_time(value):>12} "
                f"{ratio:>13}"
            )

        print()


def main() -> None:
    args = parse_args()
    text = read_source(args.input)
    results = collect_results(text.splitlines())
    print_report(results, args.baseline)


if __name__ == "__main__":
    main()
