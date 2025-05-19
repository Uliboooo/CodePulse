# CodePulse

**STILL in BETA**

## command map

Planned for v1.

[] is option value, <> is require argument.

- `init`: initialize project
    - `[title]`: if it is blank, use now time.
    - `-p [path]`: if it is blank, use current dir.
- `issue`: manage issues
  - `create <issues_title>`: create a new issue from title to current Project(dir)
  - `list [title]`: show list of issues and each IDs(num). you can simple search when using `[title]`
    - `--All`: (require `list`) show list of issues: contained closed. in default, show only opened
  - `remove <issue_id>`: remove issue by id
- `comment [id]`: manage comments
  - `-m --message <comment message> [id(num)]`: add comment by id
  - `--hide [id]`: hide comment message by id
- `open <issue_id>`: open issue by id. if opened issue, you don't need to set issue_id to `comment [id]`.

```bash
cpls init
cpls issue create
           list
           remove
cpls comment -m "
```

Planned for v2.

- -i: interactive mode