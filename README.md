# s3find

Find your files over your S3 bucket just like you would do with find. Use --bucket to specify bucket, --name to define a regex pattern and --type to filter files or directories. The --prefix and --maxdepth arguments are not supported right now, because I did not need them. If you do feel free to contribute or fork it.

## Installation

Clone this repository and run

```bash
cargo build --release
```

...and symlink to one of your directories in your PATH.

## License

This project is licensed under the [GLWTSPL](/LICENSE).

![Good Luck](https://github.com/me-shaon/GLWTPL/raw/master/good-luck.gif)

...and godspeed.
