# watch

watch is a program that notices a change in a file of a directory and sub-directories to then execute a list of given commands.
It also executes commands when one or multiple keys are pressed and you can make multiple of these events.

<br>

To create a watch.json file, type this command:

```
watch init
```

<br>

To use it, create a watch.json file, and then you'll need to at least put a watch section that should look like that modified to your need:

``` json
{
    "watch": {
        "root": "src/",
        "format": ".rs",
        "commands":[
            "cargo build",
            "echo Compilation finished for !{$}!"
        ]
    },
    "keys": {
        "events":[
            {
                "keys": ["LAlt", "LShift"],
                "commands": [
                    "start cargo run"
                ]
            }
        ]
    }
}
```

<br>

# Key Events

As seen on the code on top, you can add key events to the config file, the keys in the keys list are named how they are in the device_query library as they are translated from a string to the key.

<br>

# File-specific events

As seen also on the code on top, the "!{$}" will put the name of the file that has been recently changed in the command.