import subprocess
import tempfile
import unittest
from pathlib import Path
from ci_input_closure import compare, git_tree, Hold


class ClosureTests(unittest.TestCase):
    def setUp(self):
        self.old = {p: ('100644', 'a' * 40) for p in
                    ('api/a', 'client/b', 'docs/c', 'toolchain')}
        self.units = {'api': {'roots': ['api'], 'needs': []},
                      'client': {'roots': ['client'], 'needs': ['api']},
                      'docs': {'roots': ['docs'], 'needs': []}}

    def audit(self, new):
        return compare(self.old, new, self.units, ['toolchain'])

    def test_selective_transitive_invalidation(self):
        new = dict(self.old, **{'api/a': ('100644', 'b' * 40)})
        r = self.audit(new)
        self.assertEqual([r['units'][n]['action'] for n in ('api', 'client', 'docs')],
                         ['RERUN', 'RERUN', 'CANDIDATE_REUSE'])
        self.assertEqual(r['units']['docs']['old'], r['units']['docs']['new'])
        self.assertFalse(r['release_gate_authorized'])

    def test_added_deleted_renamed_and_mode_change(self):
        for change in ('add', 'delete', 'rename', 'mode'):
            with self.subTest(change=change):
                new = dict(self.old)
                if change in ('delete', 'rename'):
                    del new['api/a']
                if change in ('add', 'rename'):
                    new['api/new'] = ('100644', 'b' * 40)
                if change == 'mode':
                    new['api/a'] = ('100755', 'a' * 40)
                self.assertEqual(self.audit(new)['units']['client']['action'], 'RERUN')

    def test_unknown_and_global_changes_invalidate_all(self):
        for path in ('new/unowned', 'toolchain'):
            r = self.audit(dict(self.old, **{path: ('100644', 'b' * 40)}))
            self.assertTrue(all(u['action'] == 'RERUN' for u in r['units'].values()))

    def test_unsupported_input_even_unchanged(self):
        self.old['api/a'] = ('120000', 'a' * 40)
        self.assertIn('unsupported_input_kind', self.audit(self.old)['units']['client']['reasons'])

    def test_cycle_unknown_dependency_and_missing_root_hold(self):
        self.units['api']['needs'] = ['client']
        with self.assertRaises(Hold):
            self.audit(self.old)
        self.units['api']['needs'] = ['missing']
        with self.assertRaises(Hold):
            self.audit(self.old)
        self.units['api'] = {'roots': ['absent'], 'needs': []}
        with self.assertRaises(Hold):
            self.audit(self.old)

    def test_real_git_inventory_ignores_dirty_worktree(self):
        with tempfile.TemporaryDirectory() as tmp:
            def git(*args):
                return subprocess.check_output(['git', '-C', tmp, *args],
                                               stderr=subprocess.DEVNULL).decode().strip()
            git('init', '-q')
            git('config', 'user.name', 'Test')
            git('config', 'user.email', 'test@example.invalid')
            p = Path(tmp, 'file')
            p.write_text('original')
            git('add', 'file')
            git('commit', '-qm', 'initial')
            old = git('rev-parse', 'HEAD')
            a = git_tree(tmp, old)
            p.write_text('dirty')
            self.assertEqual(git_tree(tmp, old), a)
            git('add', 'file')
            git('commit', '-qm', 'change')
            self.assertNotEqual(git_tree(tmp, git('rev-parse', 'HEAD')), a)
            with self.assertRaises(Hold):
                git_tree(tmp, 'HEAD')


if __name__ == '__main__':
    unittest.main()
