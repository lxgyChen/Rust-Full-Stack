# How to make it work

1. Echo app(Done)
2. Proxy GET request bot(Done)
3. Your own project.(Done)

## Errors

I had this error to compile the project.

```console
error[E0658]: `&std::sync::Arc<bot::Bot>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
```

It was a problem with Rust stable. You can solve this with this command.

```console
$rustup override set stable/rustup override set nightly
```
