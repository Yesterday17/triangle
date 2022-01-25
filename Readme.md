# Triangle

A simple new year red packet quiz platform.

## Installation

```bash
cargo install github.com/Yesterday17/triangle
```

## Usage

First, write quiz in a TOML file:

```toml
# quiz.toml
[[quiz]]
name = "Title"
description = '''
Description <br>
HTML tags are available here, be careful with xss!
'''
links = [
    { name = "Link", url = "https://blog.mmf.moe" },
]

# Multiple quizs are supported.
[[quiz]]
name = "Second"
description = "Second description"
# links is optional
```

Then use triangle:

```bash
# generate lock file & watch quiz.toml
triangle --config quiz.toml --watch
```

Finally open `http://localhost:8080`, your quiz will be ready!

## UUIDs

UUID is used to identify unique quiz in Triangle. Usually it's generated automatically and saved in the
corresponding `.lock` file, but you can also specify it manually.

Quiz sequence is defined in `.toml` file as array order. Once a quiz was solved, user should be guided to the next one.
So your actual quiz(for example, another web sever) may require UUID generated in `.lock` file as the answer(flag)
to identify the next quiz.

## License

[Apache 2.0](LICENSE)