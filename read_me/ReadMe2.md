### 脚本

```bash
brew install pipx
# 安装 pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。
pipx install pre-commit
pre-commit install
# 是 Git 中一个用于快速提交已跟踪文件所有变更的快捷命令
git commit -a
```

# 生成项目
```bash
 cargo generate Qfish007/bt-template

 # 安装 pre-commit 钩子
 pre-commit install
```

# 预提交检查
```bash 
pre-commit run end-of-file-fixer --all-files
pre-commit run --all-files
```