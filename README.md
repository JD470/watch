# watch

watch is a program that looks for a change in a directory and sub-directories to then execute a list of given commands.
It also executes commands when one or multiple keys are pressed.

<br>

# Installation guide

Execute this command to install this program:
```
cargo install --path .
``` 

To create a watch.json file in a directory, type this command:

```
watch init
```

# watch.json example

``` json
{
    "to_watch": {
        "root": [
            "src/"
        ],
        "format": [
            ".rs"
        ],
        "commands":[
            "cargo build",
            "echo Compilation finished for !{$}!"
        ]
    },
    "key_events": [
        {
            "keys": ["LAlt", "LShift"],
            "commands": [
                "start cargo run"
            ]
        }
    ]
}
```

<br>

# Key Events

As seen on the code on top, you can add key events to the config file, the keys in the keys list are named how they are in the device_query library as they are translated from a string to the key.

<br>

# File-specific events

As seen also on the code on top, the "!{$}" will put the name of the file that has been recently changed in the command.