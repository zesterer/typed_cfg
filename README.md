# `typed_cfg`

A statically-typed and exhaustively-checked alternative to `cfg(feature)`.

## The problem

Let's say you're writing a library. It has two features, `foo` and `bar`.

Some functions are only enabled by `foo`. Some functions are only enabled with `bar`.

```rust
#[cfg(feature = "foo")]
fn frobnicate() {
    ...
}

#[cfg(feature = "bar")]
fn barnicate() {
    ...
}
```

Later in development, you make a mistake. You accidentally call a `bar`-only function from a `foo`-only function. That's
a problem! It means that if a user only wants to enable `foo`, they end up with a *compilation error* unless they enable
`bar` too - that's a semver-breaking change!

```rust
#[cfg(feature = "foo")]
fn frobnicate() {
    ...
    barnicate(); // Uh oh!
    ...
}
```

How do you test for this in CI? Most projects do a compilation check with no features enabled and all features enabled,
but the only way to catch bugs like this is to do an exhaustive check through *every combination* of features. With just
a handful of features, you already start needing hundreds of compilation checks - clearly, this isn't viable!

## The solution

`typed_cfg` provides a solution: what if Rust's type system could be used to perform these checks?

Here's how it works:

```rust
use typed_cfg::*;

// First, we list the features that our crate supports
cfgs! { feature = { "foo", "bar" } }

// Next, we express our feature gates as *trait bounds*

fn frobnicate() where feature: Is<"foo"> {
    ...
}

fn barnicate() where feature: Is<"bar"> {
    ...
}
```

That's it! Users of your crate can call the functions as normal. Only one change is required in CI: we perform a normal
`cargo check`, but with the `CHECK_CFG` environment variable set.

```
CHECK_CFG=1 cargo check
```

Now, let's see what happens if we accidentally make the mistake we made before:

```
error[E0277]: Configuration requirements are not always met
  --> src/main.rs:8:5
   |
8  |     barnicate();
   |     ^^^^^^^^^^^ The compile-time condition cfg(feature = "bar") is not always true in this scope
   |
   = help: the trait `typed_cfg::Is<"bar">` is not implemented for `feature`
   = note: Consider adding a `where feature: Is<"bar">` bound to ensure the caller respects the required configuration
note: required by a bound in `barnicate`
  --> src/main.rs:11:31
   |
11 | fn barnicate() where feature: Is<"bar"> {
   |                               ^^^^^^^^^ required by this bound in `barnicate`
```

Bingo! The compiler has successfully alerted us to our mistake, and we've avoided breaking our crate's API for
downstream users.

## Targets and more

`typed_cfg` doesn't just work with feature flags! Arbitrary cfg keys are also supported, such as `target_family`:

```rust
use typed_cfg::*;

// The `read_at` operation is only supported on POSIX-like operating systems!
fn file_read_at(file: &File, buf: &mut [u8], offset: u64) -> std::io::Result<usize>
    where target_family: Is<"unix">
{
    #[cfg(target_family = "unix")]
    std::os::unix::fs::FileExt::read_at(file, buf, offset)

    // On non-POSIX platforms, the function isn't even callable!
    #[cfg(not(target_family = "unix"))]
    unreachable!()
}
```

## Using `typed_cfg`

Unfortunately, `typed_cfg` does rely on a (currently) nightly-only Rust feature, `trivial_bounds`.
