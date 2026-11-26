# The reviewed text of every patch-set lifecycle boundary of regenerate.yml
# (#634). `test_check_regen_writers.py` requires each job's header, the order
# of its steps, and the whole text of each step named here, exactly: a
# masked command, an early exit, a skipped or reordered step, a changed shell
# or a dropped `set -euo pipefail` is a change of that text. A deliberate
# change of a lifecycle step changes this file in the same commit.

HEADERS = {
    'writer': (
        '  writer:\n'
        '    name: write (${{ matrix.writer }})\n'
        "    if: github.repository == 'UOR-Foundation/LexLean'\n"
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
    'writer': [
        ('uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n', None),
        ('name: Install the pinned Rust toolchain\n', None),
        ('uses: taiki-e/install-action@b6b84cf49ebfe0176417bdce007c624f0db37f20 # v2.86.2\n', None),
        ('name: Install elan and the pinned Lean toolchain\n', None),
        ('name: Reserve bounded overflow memory for the proofs\n', None),
        ('name: Refresh the project locks\n', None),
        ('name: Run the write recipes\n', None),
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
            '        run: python3 .github/scripts/check_regen_writers.py --parts-dir "$RUNNER_TEMP/parts" --base "$COMMIT"\n'
        ),
        (
            'name: Apply every part\n'
            '        shell: bash\n'
            '        env:\n'
            '          COMMIT: ${{ inputs.ref || github.sha }}\n'
            '        run: |\n'
            '          set -euo pipefail\n'
            '          # Each set is checked whole again, applied in turn, and the index is\n'
            "          # then each writer's tree on its paths and the commit's elsewhere.\n"
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
        ('name: Install the pinned Rust toolchain\n', None),
        ('name: Install elan and the pinned Lean toolchain\n', None),
        ('name: Reserve bounded overflow memory for the proofs\n', None),
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
        ('name: Write the moved counts\n', None),
        ('name: Write the model documents\n', None),
        ('name: Measure the stack bound of each recursive package the record lacks\n', None),
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
