# tcp-chat
## Installation
``` bash
~ ❯ cargo install tcp-chat
```
### Usage
``` bash
~ ❯ server 0.0.0.0:8080
[CONNECTED] 127.0.0.1:49574
[ECHO] abcd
[ECHO] hello wo,ro!d.
[ECHO] 한123글
[DISCONNECTED]
```
``` bash
~ ❯ client 0.0.0.0:8080
[INFO] Connected
abcd
[RECV] abcd
hello wo,ro!d.
[RECV] hello wo,ro!d.
한123글
[RECV] 한123글
^C
```
