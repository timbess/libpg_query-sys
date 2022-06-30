# libpg_query-sys

This crate provides FFI bindings to
[libpg_query](https://github.com/pganalyze/libpg_query).

## Building Against the System Library

By default, this library builds against a version of the C library that it has
vendored. However, if you set the `LIBPG_QUERY_PATH` env var when running
`cargo`, it will use this path to find the needed header and library
files. Specifically, it will look for `$LIBPG_QUERY_PATH/include/pg_query.h`
and look in `$LIBPG_QUERY_PATH/lib` for the compiled library.

## Version Mapping

This table shows how versions of this crate correspond to libpg_query
versions:

| Crate Version | C Library Version |
|---------------|-------------------|
| 0.2.0         | 13-2.1.0          |
| 0.1.3         | 10-1.0.2          |
