# px

manage all your project's shell commands in one file.

# example

the `px.toml` content is

```toml
[values]
x = 12
y = "xx"
z = false

[cmds.echo]
matrix = [["a", "b", "c"], [1, 3]]
args = ["${x}", "${#.0}-${#.1}.${z}"]

[cmds.ls]
dir = "./src"
```

exec `px ls` will get output:

```text
config.rs  exec.rs  main.rs  value.rs
```

exec `px echo a1 b2` will get output:

```text
12 a-1.false a1 b2
12 a-3.false a1 b2
12 b-1.false a1 b2
12 b-3.false a1 b2
12 c-1.false a1 b2
12 c-3.false a1 b2
```
