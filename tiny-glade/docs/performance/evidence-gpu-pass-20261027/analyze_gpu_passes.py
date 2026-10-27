#!/usr/bin/env python3
"""Rank named GPU pass spans from a validated pass-span diagnostic session."""

import argparse
import collections
import json
import math
from pathlib import Path


def percentile(values, fraction):
    ordered = sorted(values)
    index = max(0, math.ceil(fraction * len(ordered)) - 1)
    return ordered[min(index, len(ordered) - 1)]


def stats(values):
    return {
        "samples": len(values),
        "mean_ms": sum(values) / len(values),
        "p50_ms": percentile(values, 0.50),
        "p95_ms": percentile(values, 0.95),
        "p99_ms": percentile(values, 0.99),
        "max_ms": max(values),
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("session", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    session = json.loads(args.session.read_text(encoding="utf-8"))
    gpu = session["gpu_diagnostic"]
    measured_frames = {
        index for index, frame in enumerate(gpu["frames"]) if frame["measured"]
    }
    # The capture includes the boundary row where the warmup becomes measured;
    # clean observations are the following `samples` rows.
    ordered_measured = sorted(measured_frames)
    if len(ordered_measured) == session["samples"] + 1:
        measured_frames = set(ordered_measured[1:])
    assert len(measured_frames) == session["samples"]
    samples = collections.defaultdict(list)
    totals = collections.defaultdict(lambda: collections.defaultdict(float))
    identities = collections.defaultdict(set)
    statistic_fields = (
        "input_vertices",
        "input_primitives",
        "vertex_invocations",
        "clipping_invocations",
        "clipping_primitives",
        "fragment_invocations",
        "compute_invocations",
    )
    statistic_totals = collections.defaultdict(lambda: collections.defaultdict(int))

    for sample, frame_index in zip(
        gpu["pass_samples"], gpu["pass_sample_frame_indices"]
    ):
        if frame_index not in measured_frames:
            continue
        name = sample["pass_name"]
        duration_ms = sample["duration_ns"] / 1_000_000.0
        samples[name].append(duration_ms)
        totals[name][frame_index] += duration_ms
        identities[name].add(sample["pass_index"])
        statistics = sample.get("statistics")
        if statistics:
            for field in statistic_fields:
                statistic_totals[name][field] += statistics[field]

    rows = []
    for name, values in samples.items():
        per_frame_total = [totals[name][frame] for frame in measured_frames]
        row = {
            "pass_name": name,
            "pass_indices": sorted(identities[name]),
            "frames_present": len(totals[name]),
            "mean_duration_ms": sum(values) / len(values),
            "mean_total_per_frame_ms": sum(per_frame_total) / len(measured_frames),
            **stats(values),
        }
        if statistics := statistic_totals.get(name):
            for field in statistic_fields:
                row["mean_" + field] = statistics[field] / len(values)
            row["fragment_invocations_per_pixel"] = (
                statistics["fragment_invocations"] / len(values) / (1280 * 720)
            )
        rows.append(row)

    rows.sort(key=lambda row: row["mean_total_per_frame_ms"], reverse=True)
    result = {
        "schema": "gpu-pass-ranking-v1",
        "source_session": str(args.session.resolve()),
        "scene": session["scenario"],
        "warmup": session["warmup"],
        "samples": session["samples"],
        "measured_frames": len(measured_frames),
        "measurement_class": session["measurement_class"],
        "scope": (
            "Named inclusive GPU timestamp spans. Parallel or nested pass spans may "
            "overlap; rows are rankings, not additive frame latency."
        ),
        "passes": rows,
        "ranking_by_mean_duration": [
            {
                "rank": rank,
                "pass_name": row["pass_name"],
                "mean_duration_ms": row["mean_duration_ms"],
                "p95_ms": row["p95_ms"],
                "mean_total_per_frame_ms": row["mean_total_per_frame_ms"],
            }
            for rank, row in enumerate(
                sorted(rows, key=lambda row: row["mean_duration_ms"], reverse=True)[
                    :20
                ],
                1,
            )
        ],
        "ranking_by_p95": [
            {
                "rank": rank,
                "pass_name": row["pass_name"],
                "mean_duration_ms": row["mean_duration_ms"],
                "p95_ms": row["p95_ms"],
                "mean_total_per_frame_ms": row["mean_total_per_frame_ms"],
            }
            for rank, row in enumerate(
                sorted(rows, key=lambda row: row["p95_ms"], reverse=True)[:20], 1
            )
        ],
        "ranking_by_per_frame_total": [
            {
                "rank": rank,
                "pass_name": row["pass_name"],
                "mean_total_per_frame_ms": row["mean_total_per_frame_ms"],
                "mean_duration_ms": row["mean_duration_ms"],
                "p95_ms": row["p95_ms"],
            }
            for rank, row in enumerate(rows[:20], 1)
        ],
    }
    args.output.write_text(
        json.dumps(result, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    print(json.dumps(result["ranking_by_mean_duration"], indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
