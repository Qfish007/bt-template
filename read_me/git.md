# git 提交规范  [参考](https://www.conventionalcommits.org/zh-hans/v1.0.0/)
# 步骤
```bash
# 1. git init // 初始化仓库
# 2. git add . // 添加所有文件到暂存区
# 3. git commit -m "提交备注" // 提交到本地仓库
# 4. git remote add origin https://github.com/yourname/yourrepo.git // 添加远程仓库
# 5. git push -u origin master // 推送本地仓库到远程仓库
# 6. git pull origin master // 拉取远程仓库到本地仓库
# 7. git branch -a // 查看所有分支
# 8. git branch -r // 查看远程分支
# 9. git branch -vv // 查看本地分支和远程分支的关联关系
# 10. git branch --set-upstream-to=origin/master master // 设置本地分支与远程分支的关联关系
# 11. git checkout -b branch_name // 创建并切换到新分支
# 12. git checkout branch_name // 切换到已有分支
# 13. git merge branch_name // 合并分支
# 14. git branch -d branch_name // 删除分支
# 15. git branch -D branch_name // 强制删除分支
# 16. git push origin --delete branch_name // 删除远程分支
# 17. git stash // 暂存当前工作区
# 18. git stash pop // 恢复暂存的工作区
# 19. git stash list // 查看暂存的工作区列表
# 20. git stash apply stash@{n} // 恢复指定的暂存的工作区
# 21. git stash drop stash@{n} // 删除指定的暂存的工作区
# 22. git stash clear // 清空暂存的工作区
# 23. git log // 查看提交历史
# 24. git reset --hard HEAD^ // 回退到上一个版本
# 25. git reset --hard HEAD~n // 回退到前n个版本
# 26. git reset --hard commit_id // 回退到指定版本
# 27. git reflog // 查看命令历史
# 28. git checkout -- file_name // 撤销工作区的修改
# 29. git rm file_name // 删除文件
# 30. git mv old_file_name new_file_name // 重命名文件
# 31. git diff // 查看工作区和暂存区的差异
# 32. git diff --cached // 查看暂存区和最近一次commit的差异
# 33. git diff HEAD // 查看工作区和最近一次commit的差异
# 34. git diff commit_id1 commit_id2 // 查看两次commit之间的差异
# 35. git diff branch_name // 查看工作区和指定分支的差异
# 36. git diff commit_id branch_name // 查看两次commit和指定分支之间的差异
# 37. git remote -v // 查看远程仓库信息
# 38. git remote add origin url // 添加远程仓库
# 39. git remote remove origin // 删除远程仓库
# 40. git remote set-url origin url // 修改远程仓库地址
# 41. git pull origin branch_name // 拉取远程分支
# 42. git push origin branch_name // 推送分支到远程仓库
# 43. git merge branch_name // 合并分支
# 44. git rebase branch_name // 变基分支
# 45. git cherry-pick commit_id // 拉取指定commit
# 46. git tag tag_name // 创建标签
# 47. git tag -a tag_name -m "tag描述" // 创建带描述的标签
# 48. git tag -d tag_name // 删除标签
# 49. git push origin tag_name // 推送标签到远程仓库
# 50. git push origin --tags // 推送所有标签到远程仓库
# 51. git checkout tags/tag_name // 切换到指定标签
# 52. git show tag_name // 查看标签信息
# 53. git log --graph --pretty=oneline --abbrev-commit // 查看分支合并图
# 54. git stash // 暂存工作区修改
# 55. git stash list // 查看暂存区列表
# 56. git stash apply stash@{n} // 恢复暂存区修改
# 57. git stash drop stash@{n} // 删除暂存区修改
# 58. git stash pop stash@{n} // 恢复暂存区修改并删除
# 59. git reset --hard commit_id // 回退到指定commit
# 60. git reflog // 查看命令历史
# 61. git blame filename // 查看文件修改历史
# 62. git gc // 清理仓库
# 63. git remote prune origin // 清理远程仓库已删除的分支
# 64. git remote update origin --prune // 清理远程仓库已删除的分支
# 65. git gc --prune=now // 清理仓库并删除长期未使用的对象
# 66. git gc --aggressive --prune=now // 清理仓库并删除长期未使用的对象，更激进
# 67. git gc --auto // 自动清理仓库
# 68. git gc --aggressive --auto // 自动清理仓库，更激进
# 69. git gc --prune=now --aggressive // 清理仓库并删除长期未使用的对象，更激进
# 70. git gc --prune=now --aggressive --auto // 自动清理仓库并删除长期未使用的对象，更激进
# 71. git gc --prune=now --aggressive --auto --threads=4 // 自动清理仓库并删除长期未使用的对象，更激进，使用4个线程
# 72. git gc --prune=now --aggressive --auto --threads=4 --quiet // 自动清理仓库并删除长期未使用的对象，更激进，使用4个线程，静默模式
# 73. git gc --prune=now --aggressive --auto --threads=4 --quiet --verbose // 自动清理仓库并删除长期未使用的对象，更激进，使用4个线程，静默模式，详细输出
# 74. git gc --prune=now --aggressive --auto --threads=4 --quiet --verbose --progress // 自动清理仓库并删除长期未使用的对象，更激进，使用4个线程，静默模式，详细输出，显示进度
# 75. git gc --prune=now --aggressive --auto --threads=4 --quiet --verbose --progress --no-progress // 自动清理仓库并删除长期未使用的对象，更激进，使用4个线程，静默模式，详细输出，显示进度，不显示进度
# 76. git gc --prune=now --aggressive --auto --threads=4 --quiet --verbose --progress --no-progress --no-quiet // 自动清理仓库并删除长期未使用的对象，更激进，使用4个线程，详细输出，显示进度，不显示进度，不静默模式
```


# git 提交规范
```bash
# 提交所有已跟踪文件的变更
git commit -a -m "你的提交备注内容"
git commit -a -m "主要修改内容" -m "详细描述" -m "额外信息"

git commit -a -m "feat: 新增功能描述"
git commit -a -m "fix: 修复bug描述"
git commit -a -m "docs: 更新文档"
```