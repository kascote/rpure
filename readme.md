# rpure

> a simple prompt inspired by [pure](https://github.com/sindresorhus/pure)

![rpure.png](/kascote/rpure/raw/master/screenshot.png)


`rpure` is based on code from ![pista](https://github.com/nerdypepper/pista). 
Why? Started as a Rust learning project.

### features

 - shortened current working directory
 - git info (branch/commit, ahead/behind status, repo status)
 - superuser indicator
 - fully configurable
 
### Readline

The symbol (●) at the beginning of the 2nd line is the Readline vim state.
To setup it need to put this lines on the file `.inputrc` on the home directory:

```
set show-mode-in-prompt on
set vi-cmd-mode-string "⊙"
set vi-ins-mode-string "●"
```

for more information check the [Readline init file syntax](https://www.gnu.org/software/bash/manual/html_node/Readline-Init-File-Syntax.html#Readline-Init-File-Syntax)
