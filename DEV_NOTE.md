# This is a dev note to myself, do not read it

## For each new update

In the github repo folder
```bash
git add .
git commit -m "commit message"
git push
git tag -a v0.2.1 -m "Release version 0.2.1"    # Example version
git push origin v0.2.1                          # Example version
```

In the AUR repo folder
```bash
nvim PKGBUILD # Once in here change the version to match the one in the github repo
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to 0.2.1"                 # Match version
git push aur master
```