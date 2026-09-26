# coolit
Incredible tool to easily unpack files into smaller chunks and vice-versa.

# Usage

Pack `./big_file.txt` into multiple small chunks of a maximum of 10 Megabytes, and output it to `./small_files`.
```bash
coolit -i big_file.txt -o small_files -l 10Mb
```

Unpack `./small_files` into a single file, and output it to `./big_file_again.txt`.
```bash
coolit -u -i small_files.cpkg -o big_file_again.txt
```

The `-l` (or `--limit`) can be any number with a unit next to it, the supported units are:

| Unit | Name | Bytes |
|------|---------|-------|
| `[number]B` | Bytes | 1 byte |
| `[number]kB` | Kilobytes (1000 bytes) | 1000 bytes |
| `[number]KiB` | Kibibytes (1024 bytes) | 1024 bytes |
| `[number]MB` | Megabytes (1000² bytes) | 1,000,000 bytes |
| `[number]MiB` | Mebibytes (1024² bytes) | 1,048,576 bytes |
| `[number]GB` | Gigabytes (1000³ bytes) | 1,000,000,000 bytes |
| `[number]GiB` | Gibibytes (1024³ bytes) | 1,073,741,824 bytes |

# Usefulness

If you're still sitting here, reading this, and wonder what you could do with this... let's just say you *could* make
a messaging platform your next hosting service :)

The whole purpose of this tool is so you can upload files as big as you want, for example, if a platform has an
upload limit, then you can just divide it into chunks. My favorite example is discord, where (currently) `20 MB` is
the maximum you can upload, you can just divide the file you want to upload into smaller chunks of `20 MB`, and you
now have infinite space.

The best messaging platforms i found most efficient to use are the following, ranked:

| Platform | Upload Limit |
| -------- | ------------ |
| Telegram | 2 GB         |
| Whatsapp | 180 MB       |
| Discord  | 20 MB        |

(assuming no payment or perks are added, note that this could change in the future)

Also keep in mind it's useful to note the file name, extension, and if it was packaged using `coolit` before
uploading the chunks, so stay organized!
