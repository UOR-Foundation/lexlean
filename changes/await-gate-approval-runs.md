- `cut.yml`'s wait for the gate passes over runs waiting for approval
  (`action_required`): the `pull_request` runs that the workflow-token push of
  a cut's regenerated patch starts never run the gate, and the wait took one
  as the gate's failure (SPEC.md §9.4).
