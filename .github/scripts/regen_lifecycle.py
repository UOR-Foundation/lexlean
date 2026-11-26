# The reviewed text of the regeneration lifecycle of regenerate.yml (#634).
# `test_check_regen_writers.py` requires each job's header, its steps in
# order, the whole text of every step, and the whole of
# deliver-regenerated.sh, exactly: a masked command, an early exit, a
# skipped, reordered or added step, a changed shell or condition, or a
# changed branch of the delivery is a change of that text. A deliberate
# change of the workflow or the delivery changes this file in the same
# commit, and the two are reviewed together.

HEADERS = {
    'identity': (
        '  identity:\n'
        '    name: relock identity\n'
        "    if: github.repository == 'UOR-Foundation/LexLean'\n"
        '    runs-on: ubuntu-24.04\n'
        '    timeout-minutes: 120\n'
        '    steps:\n'
    ),
    'writer': (
        '  writer:\n'
        '    name: write (${{ matrix.writer }})\n'
        '    needs: [identity]\n'
        '    strategy:\n'
        '      fail-fast: false\n'
        '      matrix:\n'
        '        writer: [production-export, records, oracles]\n'
        '    runs-on: ubuntu-24.04\n'
        '    timeout-minutes: 360\n'
        '    env:\n'
        '      # The profile of the `examples` leg (§22.11): a schedule, never a byte\n'
        '      # of what is written, which is that of width 1.\n'
        '      LEXLEAN_VERIFY_WIDTH: "2"\n'
        '      LEXLEAN_VERIFY_MEMORY_KIB: "12582912"\n'
        '      LEXLEAN_VERIFY_HINTS: ${{ github.workspace }}/.github/verify-hints.json\n'
        '    steps:\n'
    ),
    'combine': (
        '  combine:\n'
        '    name: combine\n'
        '    needs: [writer]\n'
        '    runs-on: ubuntu-24.04\n'
        '    timeout-minutes: 30\n'
        '    steps:\n'
    ),
    'finish': (
        '  finish:\n'
        '    name: counts and stack bounds\n'
        '    needs: [combine]\n'
        '    runs-on: ubuntu-24.04\n'
        '    timeout-minutes: 360\n'
        '    steps:\n'
    ),
    'deliver': (
        '  deliver:\n'
        '    name: deliver\n'
        '    needs: [finish]\n'
        '    runs-on: ubuntu-24.04\n'
        '    timeout-minutes: 15\n'
        '    permissions:\n'
        '      contents: write\n'
        '      actions: write\n'
        '      pull-requests: read\n'
        '    steps:\n'
    ),
}

STEPS = {
    'identity': [
        (
            'uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n'
            '        with:\n'
            '          ref: ${{ inputs.ref || github.sha }}\n'
            '          persist-credentials: false\n'
        ),
        (
            'name: Install the pinned Rust toolchain\n'
            '        run: rustup toolchain install\n'
        ),
        (
            'uses: taiki-e/install-action@b6b84cf49ebfe0176417bdce007c624f0db37f20 # v2.86.2\n'
            '        with:\n'
            '          tool: just\n'
            '      # A lock records the Lean toolchain the project resolves.\n'
        ),
        (
            'name: Install elan and the pinned Lean toolchain\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          curl https://elan.lean-lang.org/elan-init.sh -sSf | sh -s -- -y --default-toolchain none\n'
            '          echo "$HOME/.elan/bin" >> "$GITHUB_PATH"\n'
            '          "$HOME/.elan/bin/elan" toolchain install "$(cat lean-toolchain)"\n'
        ),
        (
            'name: Relock and repin the moved identity\n'
            '        run: just relock-identity\n'
        ),
        (
            'name: Capture what it wrote\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          git add --all\n'
            "          # The relocked identity as a bounded patch set on the commit's tree:\n"
            "          # the tree it gives is the start tree of every writer's set.\n"
            '          python3 .github/scripts/patch_set.py write --name identity --base "$COMMIT" --out "$RUNNER_TEMP/identity" | tee -a "$GITHUB_STEP_SUMMARY"\n'
        ),
        (
            'uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1\n'
            '        with:\n'
            '          name: regenerate-part-identity\n'
            '          path: ${{ runner.temp }}/identity\n'
            '          if-no-files-found: error\n'
            '          retention-days: 7\n'
        ),
    ],
    'writer': [
        (
            'uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n'
            '        with:\n'
            '          ref: ${{ inputs.ref || github.sha }}\n'
            '          persist-credentials: false\n'
        ),
        (
            'uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2\n'
            '        with:\n'
            '          name: regenerate-part-identity\n'
            '          path: ${{ runner.temp }}/identity\n'
            "      # Committed locally, so that this writer's part is only what its own\n"
            '      # recipes wrote; `combine` applies the identity first.\n'
        ),
        (
            'name: Start from the relocked identity\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            "          # The set is checked whole and applied to the commit's own tree.\n"
            '          python3 .github/scripts/patch_set.py apply --dir "$RUNNER_TEMP/identity" --name identity --base "$COMMIT"\n'
            '          if ! git diff --cached --quiet; then\n'
            '            git -c user.name=regenerate -c user.email=regenerate@localhost commit --quiet --no-verify -m identity\n'
            '          fi\n'
        ),
        (
            'name: Install the pinned Rust toolchain\n'
            '        run: rustup toolchain install\n'
        ),
        (
            'uses: taiki-e/install-action@b6b84cf49ebfe0176417bdce007c624f0db37f20 # v2.86.2\n'
            '        with:\n'
            '          tool: just\n'
        ),
        (
            'name: Install elan and the pinned Lean toolchain\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          curl https://elan.lean-lang.org/elan-init.sh -sSf | sh -s -- -y --default-toolchain none\n'
            '          echo "$HOME/.elan/bin" >> "$GITHUB_PATH"\n'
            '          "$HOME/.elan/bin/elan" toolchain install "$(cat lean-toolchain)"\n'
        ),
        (
            'name: Reserve bounded overflow memory for the proofs\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          sudo fallocate -l 4G "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo chmod 600 "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo mkswap "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo swapon "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo sysctl vm.swappiness=10\n'
        ),
        (
            'name: Refresh the project locks\n'
            '        shell: bash\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          cargo build -q -p lexlean\n'
            '          L="$PWD/target/debug/lexlean"\n'
            '          for p in bootstrap stdlib compiler; do (cd "$p" && "$L" lock); done\n'
        ),
        (
            'name: Run the write recipes\n'
            '        shell: bash\n'
            '        env:\n'
            "          ATLAS: ${{ inputs.atlas || startsWith(github.ref_name, 'regenerate/atlas/') }}\n"
            '        run: |\n'
            '          set -euo pipefail\n'
            '          case "${{ matrix.writer }}" in\n'
            '            production-export) recipes="production-export-write" ;;\n'
            '            records)\n'
            '              recipes="verify-write"\n'
            '              if [ "$ATLAS" != "true" ]; then\n'
            '                export LEXLEAN_EXAMPLES_PART=without:uor-atlas\n'
            '              fi\n'
            '              ;;\n'
            '            oracles) recipes="golden-write fixtures-write calculus-write model-write" ;;\n'
            '          esac\n'
            "          just --timestamp --timestamp-format '%s%.3f' $recipes\n"
        ),
        (
            'name: Capture what the recipes wrote\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          git add --all\n'
            '          # Every writer refreshes the locks, because its recipes read them; one\n'
            '          # writer reports them, or `combine` sees three parts write one path.\n'
            '          if [ "${{ matrix.writer }}" != production-export ]; then\n'
            '            git reset -q -- bootstrap/lexlean.lock stdlib/lexlean.lock compiler/lexlean.lock\n'
            '          fi\n'
            '          # The staged change as a bounded patch set: no renames, so every\n'
            '          # path a part touches is named; NUL-delimited paths, including\n'
            '          # legal newline filenames.\n'
            '          python3 .github/scripts/patch_set.py write --name "${{ matrix.writer }}" --base "$COMMIT" --out "$RUNNER_TEMP/part" | tee -a "$GITHUB_STEP_SUMMARY"\n'
        ),
        (
            'uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1\n'
            '        with:\n'
            '          name: regenerate-part-${{ matrix.writer }}\n'
            '          path: ${{ runner.temp }}/part\n'
            '          if-no-files-found: error\n'
            '          retention-days: 7\n'
        ),
    ],
    'combine': [
        (
            'uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n'
            '        with:\n'
            '          ref: ${{ inputs.ref || github.sha }}\n'
            '          persist-credentials: false\n'
        ),
        (
            'uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2\n'
            '        with:\n'
            '          pattern: regenerate-part-*\n'
            '          path: ${{ runner.temp }}/parts\n'
            '          merge-multiple: true\n'
        ),
        (
            'name: Exercise writer ownership regression controls\n'
            "        run: python3 -m unittest discover -s .github/scripts -p 'test_check_regen_writers.py' -v\n"
        ),
        (
            'name: Exercise the bounded patch-set controls\n'
            "        run: python3 -m unittest discover -s .github/scripts -p 'test_patch_set.py' -v\n"
        ),
        (
            'name: Exercise transferred-export environment guard checks\n'
            "        run: python3 -m unittest discover -s .github/scripts -p 'test_export_custody_guard.py' -v\n"
        ),
        (
            'name: Reject overlapping or unaccounted writer paths\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: python3 .github/scripts/check_regen_writers.py --parts-dir "$RUNNER_TEMP/parts" --base "$COMMIT" --identity identity\n'
        ),
        (
            'name: Apply every part\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          # Each set is checked whole again: the identity applied to the\n'
            "          # commit's tree, then the writers' sets in turn, after which the\n"
            "          # index is each writer's tree on its paths and the identity's\n"
            '          # elsewhere.\n'
            '          python3 .github/scripts/patch_set.py apply --dir "$RUNNER_TEMP/parts" --name identity --base "$COMMIT" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '          python3 .github/scripts/patch_set.py combine --dir "$RUNNER_TEMP/parts" --names production-export,records,oracles --base "$COMMIT" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '          python3 .github/scripts/patch_set.py write --name combined --base "$COMMIT" --out "$RUNNER_TEMP/combined" | tee -a "$GITHUB_STEP_SUMMARY"\n'
        ),
        (
            'uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1\n'
            '        with:\n'
            '          name: regenerate-combined\n'
            '          path: ${{ runner.temp }}/combined\n'
            '          if-no-files-found: error\n'
            '          retention-days: 7\n'
        ),
    ],
    'finish': [
        (
            'uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n'
            '        with:\n'
            '          ref: ${{ inputs.ref || github.sha }}\n'
            '          persist-credentials: false\n'
        ),
        (
            'name: Install the pinned Rust toolchain\n'
            '        run: rustup toolchain install\n'
        ),
        (
            'name: Install elan and the pinned Lean toolchain\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          curl https://elan.lean-lang.org/elan-init.sh -sSf | sh -s -- -y --default-toolchain none\n'
            '          echo "$HOME/.elan/bin" >> "$GITHUB_PATH"\n'
            '          "$HOME/.elan/bin/elan" toolchain install "$(cat lean-toolchain)"\n'
        ),
        (
            'name: Reserve bounded overflow memory for the proofs\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          sudo fallocate -l 4G "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo chmod 600 "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo mkswap "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo swapon "$RUNNER_TEMP/lexlean-regenerate.swap"\n'
            '          sudo sysctl vm.swappiness=10\n'
        ),
        (
            'uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2\n'
            '        with:\n'
            '          name: regenerate-combined\n'
            '          path: ${{ runner.temp }}/combined\n'
        ),
        (
            'name: Apply the regenerated tree\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: python3 .github/scripts/patch_set.py apply --dir "$RUNNER_TEMP/combined" --name combined --base "$COMMIT"\n'
        ),
        (
            'name: Write the moved counts\n'
            '        shell: bash\n'
            '        run: |\n'
            '          set -uo pipefail\n'
            '          mkdir -p "$RUNNER_TEMP/regenerated"\n'
            '          # The parts of `.github/bdd-parts.json` that hold a case with a\n'
            '          # `crate::counts!` check; each runs only up to its counts.\n'
            "          parts=$(python3 - <<'PY'\n"
            '          import json, pathlib, re\n'
            '          ids = set()\n'
            '          for path in pathlib.Path("crates/conformance/src").rglob("*.rs"):\n'
            '              ids |= set(re.findall(r\'counts!\\(\\s*"([A-Z]+-[0-9]+)"\', path.read_text()))\n'
            '          plan = json.loads(pathlib.Path(".github/bdd-parts.json").read_text())\n'
            '          parts = plan["parts"] if "parts" in plan else plan\n'
            '          print(" ".join(sorted(name for name, members in parts.items()\n'
            '                                if isinstance(members, list) and ids & set(members))))\n'
            '          PY\n'
            '          )\n'
            '          if [ -z "$parts" ]; then\n'
            '            echo "no case of this tree has a \\`crate::counts!\\` check: no counts written" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '            exit 0\n'
            '          fi\n'
            '          echo "parts with counts: $parts"\n'
            '          for part in $parts; do\n'
            '            LEXLEAN_COUNTS=fast LEXLEAN_BDD_PART="$part" cargo xtask bdd > "$RUNNER_TEMP/fast-$part.log" 2>&1\n'
            '            echo "$part: exit $?"\n'
            '          done\n'
            '          cargo xtask counts --write 2>&1 | grep -v -E \'^\\s*(Finished|Running|Compiling)\' | tee "$RUNNER_TEMP/regenerated/counts.txt"\n'
            '          test "${PIPESTATUS[0]}" -eq 0\n'
        ),
        (
            'name: Write the model documents\n'
            '        run: cargo xtask validate-model --write\n'
        ),
        (
            'name: Measure the stack bound of each recursive package the record lacks\n'
            '        shell: bash\n'
            '        env:\n'
            '          LEXLEAN_VERIFY_WIDTH: "2"\n'
            '          LEXLEAN_VERIFY_MEMORY_KIB: "12582912"\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          # Each probe that runs out of stack aborts. The runner pipes a core\n'
            '          # to a handler (apport, systemd-coredump) that copies the whole\n'
            "          # probe process, and dozens of them exhaust the runner's memory, so\n"
            '          # cores go to a plain file under a zero limit: none is written.\n'
            '          sudo sysctl -w kernel.core_pattern=core\n'
            '          ulimit -c 0\n'
            '          # A runner that runs out of memory loses its agent and leaves no log\n'
            '          # of why, so the free memory goes to the log once a minute.\n'
            '          ( while true; do echo "memory $(date -u +%H:%M:%S) $(free -m | awk \'/^Mem:/ {print "used=" $3 "M available=" $7 "M"} /^Swap:/ {print "swap=" $3 "M"}\' | tr \'\\n\' \' \')"; sleep 60; done ) &\n'
            '          trace=$!\n'
            '          trap \'kill "$trace" 2>/dev/null || true\' EXIT\n'
            '          # Rows for packages no longer in the exports are dropped; for each\n'
            '          # recursive package without a row, the case whose source names its\n'
            '          # root runs whole and measures it (`LEXLEAN_MEASURE_STACK=all`).\n'
            "          cases=$(python3 - <<'PY'\n"
            '          import json, pathlib, re\n'
            '          record_path = pathlib.Path("tests/stack-bounds.json")\n'
            '          record = json.loads(record_path.read_text())\n'
            '          recursive = set()\n'
            '          for corpus in ("bootstrap", "stdlib"):\n'
            '              for provenance in pathlib.Path("tests/production-export", corpus, "packages").rglob("provenance.json"):\n'
            '                  if "recursion" in json.loads(provenance.read_text()).get("effects", []):\n'
            '                      recursive.add(f"{corpus}/{provenance.parent.name}")\n'
            '          rows = {key for key in record["rows"] if key != "bootstrap/evaluator"}\n'
            '          for stale in sorted(rows - recursive):\n'
            '              print(f"dropping the row of {stale}", file=__import__("sys").stderr)\n'
            '              del record["rows"][stale]\n'
            '          if rows - recursive:\n'
            '              record_path.write_text(json.dumps(record, indent=2) + "\\n")\n'
            '          sources = list(pathlib.Path("crates/conformance/src/cases").rglob("*.rs"))\n'
            '          cases = set()\n'
            "          # Each corpus's roots.json names the root a package exports; the\n"
            "          # case that runs the package names that root's member.\n"
            '          roots = {}\n'
            '          for corpus in ("bootstrap", "stdlib"):\n'
            '              listed = pathlib.Path("tests/production-export", corpus, "roots.json")\n'
            '              if listed.exists():\n'
            '                  for entry in json.loads(listed.read_text()).get("roots", []):\n'
            '                      roots[f"{corpus}/{entry[\'name\']}"] = entry["root"].rsplit(".", 1)[-1]\n'
            '          for package in sorted(recursive - rows):\n'
            '              name = package.split("/", 1)[1]\n'
            '              root = roots.get(package) or name.split("_")[0] + "".join(part.title() for part in name.split("_")[1:])\n'
            '              owners = set()\n'
            '              for source in sources:\n'
            '                  text = source.read_text()\n'
            "                  # The file's first line names its own case.\n"
            "                  own = re.match(r'//! `([A-Z]+-[0-9]+)`', text)\n"
            '                  if f\'"{root}"\' in text and own:\n'
            '                      owners.add(own.group(1))\n'
            '              if not owners:\n'
            '                  raise SystemExit(f"{package}: no case names the root `{root}`; measure it by hand")\n'
            '              print(f"{package}: measured by {\', \'.join(sorted(owners))}", file=__import__("sys").stderr)\n'
            '              cases |= owners\n'
            '          print(" ".join("conformance_" + case.lower().replace("-", "_") for case in sorted(cases)))\n'
            '          PY\n'
            '          )\n'
            '          if [ -z "$cases" ]; then\n'
            '            echo "every recursive package has its row" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '            exit 0\n'
            '          fi\n'
            '          echo "measuring: $cases" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '          # Three runs on 2026-11-14 lost their runner while several of these\n'
            "          # cases measured at once. Alone, BS-36 left 7 GiB of this runner's\n"
            '          # 16 GiB available and BS-72 measured 10 to 12 GiB, so these run one\n'
            '          # at a time and the rest two at a time.\n'
            '          heavy="conformance_bs_36 conformance_bs_69 conformance_bs_70 conformance_bs_71 conformance_bs_72"\n'
            '          light=""\n'
            '          heavy_cases=""\n'
            '          for case in $cases; do\n'
            '            is_heavy=0\n'
            '            for h in $heavy; do\n'
            '              if [ "$case" = "$h" ]; then\n'
            '                is_heavy=1\n'
            '                break\n'
            '              fi\n'
            '            done\n'
            '            if [ "$is_heavy" = 1 ]; then\n'
            '              heavy_cases="$heavy_cases $case"\n'
            '            else\n'
            '              light="$light $case"\n'
            '            fi\n'
            '          done\n'
            "          # A job's own 360-minute limit cancels this step outright if it is\n"
            '          # reached, and a cancelled job skips every later step, so the tree\n'
            '          # never returns and a row measured seconds before the limit is\n'
            '          # lost with everything else. Each row is instead written to\n'
            '          # tests/stack-bounds.json as it is measured (stack_bounds.rs\n'
            '          # record_row/rewrite), so a step that fails, rather than being\n'
            '          # cancelled, still returns whatever is already on disk. Stop\n'
            "          # ourselves inside a budget comfortably under the job's own limit,\n"
            '          # so a run that cannot finish everything fails cleanly instead of\n'
            '          # being cancelled, and keeps every row it reached.\n'
            '          step_start=$(date +%s)\n'
            '          deadline_s=$((320 * 60))\n'
            '          overall_status=0\n'
            '          remaining() {\n'
            '            local now elapsed left\n'
            '            now=$(date +%s)\n'
            '            elapsed=$((now - step_start))\n'
            '            left=$((deadline_s - elapsed))\n'
            '            if [ "$left" -lt 0 ]; then\n'
            '              left=0\n'
            '            fi\n'
            '            echo "$left"\n'
            '          }\n'
            '          if [ -n "$light" ]; then\n'
            '            left=$(remaining)\n'
            '            if [ "$left" -gt 0 ]; then\n'
            '              echo "measuring (threads=2, budget ${left}s): $light" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '              if ! timeout "${left}s" env LEXLEAN_MEASURE_STACK=all cargo test -p repo-conformance --all-features --test conformance -- $light --test-threads=2 --nocapture 2>&1 \\\n'
            "                | grep -E --line-buffered 'debug [0-9]+ bytes|test result|panicked|^test '; then\n"
            '                overall_status=1\n'
            '              fi\n'
            '            else\n'
            '              echo "no time left in the ${deadline_s}s budget; skipping: $light" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '              overall_status=1\n'
            '            fi\n'
            '          fi\n'
            '          # Heavy cases already run one at a time; looping them individually\n'
            '          # (instead of one --test-threads=1 invocation over all of them)\n'
            '          # costs nothing and prints exactly when each one starts and ends.\n'
            '          for case in $heavy_cases; do\n'
            '            left=$(remaining)\n'
            '            if [ "$left" -le 0 ]; then\n'
            '              echo "no time left in the ${deadline_s}s budget; skipping: $case" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '              overall_status=1\n'
            '              continue\n'
            '            fi\n'
            '            started=$(date +%s)\n'
            '            echo "$case: started $(date -u +%H:%M:%S) (threads=1, budget ${left}s)" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '            if ! timeout "${left}s" env LEXLEAN_MEASURE_STACK=all cargo test -p repo-conformance --all-features --test conformance -- "$case" --test-threads=1 --nocapture 2>&1 \\\n'
            "              | grep -E --line-buffered 'debug [0-9]+ bytes|test result|panicked|^test '; then\n"
            '              overall_status=1\n'
            '            fi\n'
            '            ended=$(date +%s)\n'
            '            mins=$(( (ended - started) / 60 ))\n'
            '            echo "$case: ended $(date -u +%H:%M:%S), ${mins}m" | tee -a "$GITHUB_STEP_SUMMARY"\n'
            '          done\n'
            '          exit "$overall_status"\n'
        ),
        (
            'name: Return the regenerated tree\n'
            '        if: ${{ !cancelled() }}\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          out="$RUNNER_TEMP/regenerated"\n'
            '          mkdir -p "$out"\n'
            '          git add --all\n'
            '          python3 .github/scripts/patch_set.py write --name regenerated --base "$COMMIT" --out "$out"\n'
            '          git diff --cached --name-only > "$out/regenerated-files.txt"\n'
            '          echo "$COMMIT" > "$out/base-commit.txt"\n'
            '          # Every file of the artifact, so `deliver` refuses one missing,\n'
            '          # altered, or added. It binds the files to each other, not to their\n'
            "          # producer: the run's own artifact store is what this trusts.\n"
            '          (cd "$out" && find . -mindepth 1 -maxdepth 1 -type f ! -name SHA256SUMS -printf \'%P\\0\' \\\n'
            '            | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS)\n'
            '          {\n'
            '            echo "### Regenerated $(wc -l < "$out/regenerated-files.txt") paths on $COMMIT"\n'
            '            echo\n'
            '            if [ -s "$out/counts.txt" ]; then\n'
            "              echo '#### Counts (the writer lists here anything it could not place)'\n"
            "              echo '```text'\n"
            '              cat "$out/counts.txt"\n'
            "              echo '```'\n"
            '            fi\n'
            "            echo '```bash'\n"
            '            echo "gh run download $GITHUB_RUN_ID --repo $GITHUB_REPOSITORY --name regenerated-$COMMIT --dir regenerated"\n'
            '            echo "python3 .github/scripts/patch_set.py apply --dir regenerated --name regenerated --base $COMMIT"\n'
            "            echo '```'\n"
            '          } >> "$GITHUB_STEP_SUMMARY"\n'
        ),
        (
            'uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1\n'
            '        if: ${{ !cancelled() }}\n'
            '        with:\n'
            '          name: regenerated-${{ inputs.ref || github.sha }}\n'
            '          path: ${{ runner.temp }}/regenerated\n'
            '          if-no-files-found: error\n'
            '          retention-days: 7\n'
        ),
    ],
    'deliver': [
        (
            'uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n'
            '        with:\n'
            '          ref: ${{ inputs.ref || github.sha }}\n'
        ),
        (
            'uses: actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333 # v8.0.2\n'
            '        with:\n'
            '          name: regenerated-${{ inputs.ref || github.sha }}\n'
            '          path: ${{ runner.temp }}/regenerated\n'
        ),
        (
            'name: Commit the regenerated tree to the pull request and run the gate there\n'
            '        shell: bash\n'
            '        env:\n'
            '          GH_TOKEN: ${{ github.token }}\n'
            '          REPO: ${{ github.repository }}\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '          BRANCH: ${{ inputs.branch }}\n'
            '          RUN_URL: ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}\n'
            '          REGENERATED: ${{ runner.temp }}/regenerated\n'
            '        run: .github/scripts/deliver-regenerated.sh\n'
        ),
    ],
}

DELIVERY = (
    '#!/usr/bin/env bash\n'
    '# The delivery of `regenerate.yml` (SPEC.md §9.4): the patch set its `finish` job\n'
    '# returned, committed on the commit it was made from and pushed to the branch\n'
    '# that commit heads, then the gate dispatched on that branch.\n'
    '#\n'
    '# The branch is the one a caller names (BRANCH, `cut.yml`), or else the head\n'
    '# branch of the one open pull request of this repository whose head is\n'
    '# COMMIT. With none, or several, nothing is pushed and the patch set stays the\n'
    "# run's artifact. The push is a fast-forward of COMMIT: a branch pushed again\n"
    '# while the run worked rejects it, and the newer head is left alone. A push\n'
    '# with the workflow token starts no `pull_request` run, so `ci.yml` is\n'
    '# dispatched on the branch, where its plan treats the run as the pull\n'
    "# request's (`.github/ci-plan.sh`). The `regenerate/` branch that started the\n"
    '# run is deleted once its tree is delivered, if it still names COMMIT.\n'
    '#\n'
    '# Inputs, from the environment: REPO, COMMIT, BRANCH (may be empty), RUN_URL,\n'
    '# REGENERATED (the downloaded `regenerated-<commit>` artifact), GITHUB_REF,\n'
    '# GITHUB_STEP_SUMMARY. The checkout is COMMIT with the workflow token. Output,\n'
    '# to GITHUB_OUTPUT: `head`, the commit the gate was dispatched on, when it was\n'
    '# (`cut.yml` waits for that run and lands the cut on it); nothing otherwise.\n'
    'set -euo pipefail\n'
    '\n'
    'repo=${REPO:?REPO names owner/name}\n'
    'commit=${COMMIT:?COMMIT names the regenerated commit}\n'
    'branch=${BRANCH:-}\n'
    'dir=${REGENERATED:?REGENERATED names the artifact directory}\n'
    'summary=${GITHUB_STEP_SUMMARY:-/dev/null}\n'
    'output=${GITHUB_OUTPUT:-/dev/null}\n'
    'say() { printf \'%s\\n\' "$1" | tee -a "$summary"; }\n'
    '\n'
    '(cd "$dir" && sha256sum --check --strict --quiet SHA256SUMS)\n'
    '# `sha256sum --check` reads only what SHA256SUMS lists: every other entry of\n'
    '# the artifact is refused, so no file reaches the commit unchecked.\n'
    'listed=$(cd "$dir" && sed -E \'s/^[0-9a-f]{64}  //\' SHA256SUMS | LC_ALL=C sort)\n'
    'present=$(cd "$dir" && find . -mindepth 1 -maxdepth 1 ! -name SHA256SUMS -printf \'%P\\n\' | LC_ALL=C sort)\n'
    'if [ "$listed" != "$present" ]; then\n'
    '  echo "::error::SHA256SUMS does not list exactly the files of the artifact"\n'
    '  exit 1\n'
    'fi\n'
    'if [ "$(cat "$dir/base-commit.txt")" != "$commit" ]; then\n'
    '  echo "::error::the artifact was made from $(cat "$dir/base-commit.txt"), not $commit"\n'
    '  exit 1\n'
    'fi\n'
    '\n'
    'if [ -z "$branch" ]; then\n'
    '  heads=$(gh api "repos/$repo/pulls?state=open&per_page=100" --paginate \\\n'
    '    --jq ".[] | select(.head.repo.full_name == \\"$repo\\" and .head.sha == \\"$commit\\") | .head.ref" | sort -u)\n'
    '  count=$(printf \'%s\' "$heads" | grep -c . || true)\n'
    '  if [ "$count" -ne 1 ]; then\n'
    '    say "No single open pull request of $repo has head \\`$commit\\` ($count found): nothing is pushed; the patch is this run\'s artifact \\`regenerated-$commit\\`."\n'
    '    exit 0\n'
    '  fi\n'
    '  branch=$heads\n'
    'fi\n'
    '\n'
    'remote=$(git ls-remote origin "refs/heads/$branch" | cut -f1)\n'
    'if [ "$remote" != "$commit" ]; then\n'
    '  echo "::warning::$branch moved from $commit to ${remote:-nothing}; nothing is pushed"\n'
    '  say "\\`$branch\\` is at \\`${remote:-nothing}\\`, no longer \\`$commit\\`: nothing is pushed; the patch is this run\'s artifact \\`regenerated-$commit\\`."\n'
    '  exit 0\n'
    'fi\n'
    '\n'
    "# The set is checked whole and applied to the commit's own tree; the index\n"
    '# must then hold exactly the tree `finish` recorded.\n'
    'python3 .github/scripts/patch_set.py apply --dir "$dir" --name regenerated --base "$commit"\n'
    'if git diff --cached --quiet; then\n'
    '  say "The regenerated tree of \\`$commit\\` is the committed tree: nothing to deliver."\n'
    "  # A caller's branch was pushed with the workflow token, which started no\n"
    '  # run: the gate is dispatched for it all the same.\n'
    '  if [ -n "${BRANCH:-}" ]; then\n'
    '    gh workflow run ci.yml --repo "$repo" --ref "$branch"\n'
    '    echo "head=$commit" >> "$output"\n'
    '  fi\n'
    'else\n'
    '  paths=$(git diff --cached --name-only | wc -l)\n'
    '  {\n'
    '    printf \'Regenerate the written outputs of %s\\n\\n\' "${commit:0:10}"\n'
    '    printf \'The write recipes of SPEC.md §9.2, run by regenerate.yml (%s),\\n\' "$RUN_URL"\n'
    '    printf \'rewrote %s paths.\\n\' "$paths"\n'
    '    if [ -s "$dir/counts.txt" ]; then\n'
    "      printf '\\nThe counts writer reported:\\n\\n'\n"
    '      sed \'s/^/    /\' "$dir/counts.txt"\n'
    '    fi\n'
    '  } > "$RUNNER_TEMP/message.txt"\n'
    "  git -c user.name='github-actions[bot]' \\\n"
    "    -c user.email='41898282+github-actions[bot]@users.noreply.github.com' \\\n"
    '    commit --quiet --file "$RUNNER_TEMP/message.txt"\n'
    '  # A plain push is a fast-forward or nothing.\n'
    '  git push origin "HEAD:refs/heads/$branch"\n'
    '  gh workflow run ci.yml --repo "$repo" --ref "$branch"\n'
    '  echo "head=$(git rev-parse HEAD)" >> "$output"\n'
    '  say "Delivered $paths regenerated paths to \\`$branch\\` as \\`$(git rev-parse HEAD)\\` and dispatched \\`ci.yml\\` there."\n'
    'fi\n'
    '\n'
    'case "${GITHUB_REF:-}" in\n'
    '  refs/heads/regenerate/*)\n'
    '    started=${GITHUB_REF#refs/heads/}\n'
    '    if git push --force-with-lease="refs/heads/$started:$commit" origin ":refs/heads/$started"; then\n'
    '      say "Deleted \\`$started\\`."\n'
    '    fi\n'
    '    ;;\n'
    'esac\n'
)
