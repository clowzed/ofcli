# Onefile cli

Cli tool for uploading files to onefile.clowzed.ru

Related project: [link](https://github.com/clowzed/onefile)

# Installation
```bash
cargo install ofcli
```
or
```bash
git clone https://github.com/clowzed/ofcli
cd ofcli
cargo install --path .
cd ..
rm -rf ./ofcli
```

# Usage
Uploading
```bash
ofcli upload ./file.html
```

Opening in browser
```bash
ofcli open key-key-key
```