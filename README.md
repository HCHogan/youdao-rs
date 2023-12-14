# Yet another youdao dictionary cli written in rust

## Features

- Chinese -> English
- English -> Chinese

## Usage

    youdao-rs [FLAGS] <SUBCOMMAND> your_words

FLAGS:
-d, --debug

-h, --help Prints help information

-q, --quiet

-V, --version Prints version information


SUBCOMMANDS:

help Prints this message or the help of the given subcommand(s)

sentence

word

## Example

```bash
youdao-rs word hello
int. 喂，你好（用于问候或打招呼）；喂，你好（打电话时的招呼语）；喂，你好（引起别人注意的招呼语）；<非正式>喂，嘿 (认为别人说了蠢话或分心)；<英，旧>嘿（表示惊讶）
n. 招呼，问候；（Hello）（法、印、美、俄）埃洛（人名）
v. 说（或大声说）“喂”；打招呼
```
