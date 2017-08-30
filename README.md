# taps3-rs

[![Build Status](https://travis-ci.org/lloydmeta/taps3-rs.svg?branch=master)](https://travis-ci.org/lloydmeta/taps3-rs) [![](https://images.microbadger.com/badges/image/lloydmeta/taps3.svg)](https://microbadger.com/images/lloydmeta/taps3 "TapS3 docker image details")


Useful if you want to know if you've set firewall rules/roles/permissions/etc. properly for your Docker cluster
to access S3.

## Usage

`$ docker run lloydmeta/taps3:latest --help`

```
taps3 0.1.0
Lloyd (github.com/lloydmeta)
taps3 (tapsy) is a command that just writes the current time to a given S3 bucket and filename

USAGE:
    taps3 [OPTIONS] --bucket <bucket> --region <region>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -B, --bucket <bucket>    The name of the bucket that you want to write your tap file to.
    -F, --file <file>        The name of the file that you want to write to. [default: tapped]
    -R, --region <region>    The region of the bucket that you want to write your tap file to.
```

## Details

`taps3` is compiled as a statically-linked library via [`rust-musl-builder`](https://github.com/emk/rust-musl-builder)
and the CA certificates are downloaded from [curl](https://curl.haxx.se/docs/caextract.html) and added
separately to the Alpine-based Docker Image.

## AWS credentials

This lib uses Rusoto's `DefaultCredentialsProvider`, which resolves AWS credentials via [`ChainedProvider`](https://rusoto.github.io/rusoto/rusoto_core/struct.ChainProvider.html).

> The following sources are checked in order for credentials when calling credentials:
>
>   1. Environment variables: `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`
>   2. AWS credentials file. Usually located at `~/.aws/credentials`.
>   3. IAM instance profile. Will only work if running on an EC2 instance with an instance profile/role.
>
> If the sources are exhausted without finding credentials, an error is returned.

