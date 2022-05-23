# Extra Features

YamSH supports most features that Bash or Zsh supports, but here are some features that are not present in either of these popular shells:

## 1. Safe Deletion of Directory

Have you ever run `rm -rf` and deleted a directory that ruined your life (ज़िन्दगी बर्बाद होगयी)? Not anymore, with YamSH, you can add an option to safely delete a directory, just create an empty file inside the directory called `.saferm` using:

```bash
user@programmer~:$ cd my-favorite-directory
user@programmer~:$ touch .saferm
```

Once you are done with this step, your directory is safe from incorrect `rm -rf` commands, every time you run the command on this directory, it will ask you if you are sure you want to delete the directory and only after your confirmation will delete the directory. Didn't this make you life easier (ज़िन्दगी सफल)?