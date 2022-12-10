# Rustler Error

My attempt at getting Rust's `?` working for Rustler, both `{:ok, _}`/`{:error,
_}` variants and raising exceptions.

This uses phantom types to add some context to the exceptions. This was my
introduction to Phantom types: https://blog.hayleigh.dev/phantom-types-in-gleam
It's in Gleam not Rust, but the syntax is very similar and it's a well-written
article!

To try it out (make sure you have elixir and rust):

```console
$ mix deps.get
$ iex -S mix
iex> NIF.getenv("FOO")
{:error,
 "Error getting the environment variable: variable not set: environment variable not found"}
iex> NIF.getenv! "FOO"
** (ErlangError) Erlang error: "Error getting the environment variable: variable not set: environment variable not
found"
    (rustler_raise_error 0.1.0) NIF.getenv!("FOO")
```

But of course, the code is more interesting than the use of it (because this is
a small example)
