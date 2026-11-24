"""Conditional scheduling model; not a qualification or a prediction of capacity."""
from itertools import product


def capacity_model(lanes, join_seconds, overhead_seconds=0):
    """Exact best static assignment for 1..N identical, always available runners.

    All lanes ready at time zero; nonpreemptive, independent, no competing jobs.
    Measured durations include existing setup. Additional overhead is per lane.
    The join runs on a separate runner, immediately after all lanes complete.
    """
    if (not isinstance(lanes, dict) or not 1 <= len(lanes) <= 6
            or any(not isinstance(k, str) or not k or type(v) is not int or v <= 0
                   for k, v in lanes.items())
            or type(join_seconds) is not int or join_seconds < 0
            or type(overhead_seconds) is not int or overhead_seconds < 0):
        raise ValueError('invalid bounded scheduling inputs')
    tasks = sorted((name, duration + overhead_seconds) for name, duration in lanes.items())
    rows = []
    for capacity in range(1, len(tasks) + 1):
        best = None
        for assignment in product(range(capacity), repeat=len(tasks)):
            loads = [0] * capacity
            for (_, duration), runner in zip(tasks, assignment):
                loads[runner] += duration
            candidate = (max(loads), assignment)
            if best is None or candidate < best:
                best = candidate
        rows.append({'native_capacity': capacity,
                     'ideal_host_completion_seconds': best[0] + join_seconds,
                     'total_execution_seconds': sum(v for _, v in tasks) + join_seconds})
    return rows
