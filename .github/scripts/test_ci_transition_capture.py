import copy
import tempfile
import unittest
from pathlib import Path
from ci_transition_capture import capture, write_new


class FakeReader:
    def __init__(self, count=40):
        self.header = dict(id=12, head_sha='a'*40, run_attempt=1,
                           status='completed', conclusion='success',
                           created_at='2026-11-24T00:00:00Z', updated_at='2026-11-24T00:10:00Z')
        self.rows = [dict(id=i+1, run_id=12, head_sha='a'*40,
                          run_attempt=1, status='completed') for i in range(count)]
        self.calls = []
        self.change_header = False
        self.change_count = False
        self.short_page = False

    def get(self, endpoint, **params):
        self.calls.append((endpoint, params))
        if endpoint.endswith('/jobs'):
            assert endpoint.endswith('/attempts/1/jobs')
            assert params['per_page'] == 35
            page = params['page']
            rows = self.rows[(page-1)*35:page*35]
            if self.short_page and page == 1:
                rows = rows[:-1]
            return dict(total_count=len(self.rows)+(self.change_count and page > 1),
                        jobs=copy.deepcopy(rows))
        header = copy.deepcopy(self.header)
        if self.change_header and len(self.calls) > 1:
            header['updated_at'] = '2026-11-24T00:11:00Z'
        return header


class CaptureTests(unittest.TestCase):
    def test_exact_pages_and_stable_header(self):
        for count in (0, 35, 40, 70):
            with self.subTest(count=count):
                reader = FakeReader(count)
                out = capture(reader, 12)
                self.assertEqual(out['total_count'], count)
                self.assertEqual(len(out['jobs']), count)
                self.assertEqual(len(reader.calls), 2+max(1,(count+34)//35))

    def test_active_requires_explicit_opt_in(self):
        reader = FakeReader()
        reader.header.update(status='in_progress', conclusion=None)
        reader.rows[-1]['status'] = 'queued'
        with self.assertRaises(ValueError):
            capture(reader, 12)
        self.assertEqual(capture(reader, 12, allow_active=True)['total_count'], 40)

    def test_changed_header_or_page_refuses(self):
        for attribute in ('change_header', 'change_count', 'short_page'):
            reader = FakeReader()
            setattr(reader, attribute, True)
            with self.subTest(attribute=attribute), self.assertRaises(ValueError):
                capture(reader, 12)

    def test_identity_and_terminal_consistency(self):
        mutations = [lambda r: r.header.update(run_attempt=2),
                     lambda r: r.rows[-1].update(id=r.rows[0]['id']),
                     lambda r: r.rows[-1].update(run_id=99),
                     lambda r: r.rows[-1].update(head_sha='b'*40),
                     lambda r: r.rows[-1].update(status='queued'),
                     lambda r: r.rows[-1].update(run_attempt=2)]
        for mutate in mutations:
            reader = FakeReader()
            mutate(reader)
            with self.assertRaises(ValueError):
                capture(reader, 12)

    def test_immutable_write(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'capture.json'
            write_new(path, {'first': 1})
            before = path.read_bytes()
            with self.assertRaises(FileExistsError):
                write_new(path, {'second': 2})
            self.assertEqual(path.read_bytes(), before)
            self.assertEqual(list(Path(directory).iterdir()), [path])


if __name__ == '__main__':
    unittest.main()
