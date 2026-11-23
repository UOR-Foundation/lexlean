import importlib.util
import tempfile
import unittest
from pathlib import Path

MODULE = Path(__file__).with_name('ci_cargo_scope.py')
spec = importlib.util.spec_from_file_location('cargo_scope', MODULE)
cargo_scope = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cargo_scope)


def fixture(root):
    (root / 'Cargo.toml').write_text('[workspace]\nmembers = ["crates/*", "xtask"]\n')
    specs = [
        ('crates/model', 'repo-model', ''),
        ('crates/lexlean', 'lexlean', ''),
        ('crates/conformance', 'repo-conformance', '[dependencies]\nlexlean = "0.4"\nrepo-model = "0.4"\n'),
        ('xtask', 'xtask', '[dependencies]\nrepo-conformance = "0.4"\nrepo-model = "0.4"\nlexlean = "0.4"\n'),
    ]
    for folder, name, dependencies in specs:
        directory = root / folder
        directory.mkdir(parents=True)
        (directory / 'Cargo.toml').write_text('[package]\nname = "' + name + '"\n' + dependencies)


class CargoScope(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        fixture(self.root)

    def check(self, paths, expected):
        result = cargo_scope.affected(self.root, paths)
        self.assertEqual(result['cargo_check_packages'], sorted(expected))
        self.assertEqual(len(result['cargo_manifest_sha256']), 64)
        self.assertTrue(result['cargo_feedback_only'])
        return result

    def test_model_transitive_consumers(self):
        self.check(['crates/model/src/lib.rs'], ['repo-model', 'repo-conformance', 'xtask'])

    def test_compiler_transitive_consumers(self):
        self.check(['crates/lexlean/src/compiler.rs'], ['lexlean', 'repo-conformance', 'xtask'])

    def test_conformance_consumed_by_xtask(self):
        self.check(['crates/conformance/tests/cases.rs'], ['repo-conformance', 'xtask'])

    def test_xtask_only_and_no_change(self):
        self.check(['xtask/src/main.rs'], ['xtask'])
        self.check([], [])

    def test_renamed_code_unions_both_root_owners(self):
        self.check(['crates/lexlean/src/old.rs', 'crates/model/src/new.rs'],
                   ['repo-model', 'lexlean', 'repo-conformance', 'xtask'])

    def test_unmapped_rust_path_cannot_be_proven_safe(self):
        result = self.check(['examples/special.rs'], [])
        self.assertEqual(result['cargo_unresolved_rust_paths'], ['examples/special.rs'])

    def test_missing_or_renamed_manifest_fails_closed(self):
        (self.root / 'crates/lexlean/Cargo.toml').unlink()
        with self.assertRaisesRegex(ValueError, 'manifest'):
            cargo_scope.affected(self.root, ['xtask/src/main.rs'])

    def test_unmodeled_workspace_crate_fails_closed(self):
        folder = self.root / 'crates/new'
        folder.mkdir()
        (folder / 'Cargo.toml').write_text('[package]\nname = "new"\n')
        with self.assertRaisesRegex(ValueError, 'unmodeled'):
            cargo_scope.affected(self.root, ['xtask/src/main.rs'])

    def test_mutated_workspace_member_fails_closed(self):
        (self.root / 'Cargo.toml').write_text('[workspace]\nmembers = ["crates/*"]\n')
        with self.assertRaisesRegex(ValueError, 'membership'):
            cargo_scope.affected(self.root, ['xtask/src/main.rs'])

    def test_target_specific_reverse_dependency(self):
        file = self.root / 'crates/conformance/Cargo.toml'
        file.write_text('[package]\nname = "repo-conformance"\n'
                        '[target.\'cfg(unix)\'.dev-dependencies]\nlexlean = "0.4"\n')
        self.check(['crates/lexlean/src/lib.rs'], ['lexlean', 'repo-conformance', 'xtask'])

    def test_manifest_content_hash_changes(self):
        before = cargo_scope.affected(self.root, [])['cargo_manifest_sha256']
        with (self.root / 'crates/model/Cargo.toml').open('a') as fd:
            fd.write('# source changed\n')
        after = cargo_scope.affected(self.root, [])['cargo_manifest_sha256']
        self.assertNotEqual(before, after)


if __name__ == '__main__':
    unittest.main()
