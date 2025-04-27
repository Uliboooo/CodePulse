# Potato Issues

## command map

```bash
Usage: poit <COMMAND>

Commands:
	init	initialized potato issues tracker in /folder_path
	list	show list issues or commit messsages
	issue	control issues
	commit	commit message or other ctrl

------------------------------------------------------------------

Usage: poit init [DIR]

Arguments:
	[DIR]	target dir path. when it is empty, set current dir

------------------------------------------------------------------

Usage: poit list [OPTIONS]

Options:
	-c, --cmtmsg		show commit messages list
	-i, --id <ISSUE_ID>	when show commit messages, need to choose which isuue by id. require `--cmtmsg`
		--all			show all(include closed contents)
		--oneline		show list as oneline

------------------------------------------------------------------

Usage: poit issue [OPTIONS] [TARGET_ID]

Arguments:
	[TARGET_ID]	target issue name

Options:
	-n, --new <NEW_ISSUE_NAME>	create new issue
	-t, --tags <TAGS>			add tags to issue_name. requires "TAREGT_ID". split tags by `,`
	-d, --delete				delete issue by id. requires "TAREGT_ID"

------------------------------------------------------------------

Usage: poit commit [OPTIONS] [ISSUE_ID]

Arguments:
	[ISSUE_ID]	target id for tags, etc... requires from --message, --delete

Options:
	-m, --message <MESSAGE>	commit message by issue_id
	-d, --delete			delete by id
```

### example

**this is ""Pre-development design stage materials""**
so, there may be changes.

```bash
>poit init
initialized potato issues tracker in /folder_path

>poit issue "hot bug!!!" # create new issue
created new issue: "issuename"
id: 1

>poit issue -l
#1 issue_title
created at: Sun, 27 Apr 2025 00:53:01 +0900
updated at: Sun, 27 Apr 2025 00:57:04 +0900
tags:       ["bug fix"]

>poit issue -l --all
#1 issue_title
created at: Sun, 27 Apr 2025 00:53:01 +0900
updated at: Sun, 27 Apr 2025 00:57:04 +0900
status:     Open
tags:       ["bug fix"]

#2 issue2_title
created at: Sun, 27 Apr 2025 00:53:01 +0900
updated at: Sun, 27 Apr 2025 00:57:04 +0900
status:     Cloesd(Resolved)
tags:       ["bug fix"]

>poit issue -l --oneline
#1 issue1_title

>poit issue -l --oneline --all
#1 issue1_title
#2 issue2_title *Cloesd



>poit commit -m "there is cause in line-42" 1
added new commit "new_commit message" in "#1 issue_title" #println!("#{} {}", id, title);
```