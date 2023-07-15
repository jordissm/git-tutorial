# git-tutorial

- [ ] Clone project to local development system using `git clone`
- [ ] Set up *origin* git remote using `git remote -v`
- [ ] Set up *upstream* git remote using `git remote add <remote name> <URL of remote repo>`. This is done only the first time
- [ ] Create a local branch for any new development `git branch <feature branch name>` and checkout the branch `git checkout <feature branch name>`
- [ ] Develop and commit changes to the local branch `git status`, `git add`, `git commit -m "<Change log>"`, `git log`
- [ ] Get the most up to date version of the upstream code `git fetch upstream`
- [ ] Rebase your changes onto upstream `git rebase upstream/master`. Use `git log` to check that your commit is the most recent.
- [ ] Push the changes to your repo `git push origin <feature branch name>`
- [ ] Go to your account in GitHub and submit the pull request.
