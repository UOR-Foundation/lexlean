import unittest
from ci_arm_capacity import capacity_model


class CapacityTests(unittest.TestCase):
    def test_known_optimum(self):
        rows = capacity_model({'a': 8, 'b': 7, 'c': 6, 'd': 5, 'e': 4}, 2)
        self.assertEqual([r['ideal_host_completion_seconds'] for r in rows], [32, 17, 13, 11, 10])
        self.assertTrue(all(r['total_execution_seconds'] == 32 for r in rows))

    def test_historical_lane_sensitivity(self):
        lanes = {'producer': 694, 'moves': 2207, 'validate': 838, 'print': 1690, 'lower': 1986}
        base = capacity_model(lanes, 30)
        extra = capacity_model(lanes, 30, 300)
        self.assertEqual(base[0]['total_execution_seconds'], 7445)
        self.assertEqual(base[-1]['ideal_host_completion_seconds'], 2237)
        self.assertEqual(extra[0]['total_execution_seconds'], 8945)
        self.assertTrue(all(x['ideal_host_completion_seconds'] <= y['ideal_host_completion_seconds']
                            for x, y in zip(base, extra)))

    def test_invalid_inputs(self):
        for lanes, join, overhead in [({}, 0, 0), ({'a': True}, 0, 0), ({'a': 0}, 0, 0),
                                      ({'a': 1}, -1, 0), ({'a': 1}, 0, -1),
                                      ({str(i): 1 for i in range(7)}, 0, 0)]:
            with self.subTest(lanes=lanes, join=join, overhead=overhead), self.assertRaises(ValueError):
                capacity_model(lanes, join, overhead)


if __name__ == '__main__':
    unittest.main()
