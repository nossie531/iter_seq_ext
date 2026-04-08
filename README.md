# iter_seq_ext

Iterator extension for sequential items.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

This crate provides an iterator extension that includes
methods inspired by those of [`str`] type.

## Core items

- `IteratorSeqExt::contains` - similar to [`str::contains`].
- `IteratorSeqExt::replace` - similar to [`str::replace`].

## Examples

- `contains` example.

  ```rust
  use iter_seq_ext::prelude::*;
  
  let iter = [1, 2, 3, 4].into_iter();
  let result = IteratorSeqExt::contains(iter, &[2, 3]);
  assert!(result);
  ```

- `replace` example.

  ```rust
  use iter_seq_ext::prelude::*;
  
  let iter = [1, 2, 3, 4].into_iter();
  let result = IteratorSeqExt::replace(iter, &[2, 3], &[-2, -3]);
  assert_eq!(result.collect::<Vec<_>>(), vec![1, -2, -3, 4]);
  ```

## History

See [CHANGELOG](CHANGELOG.md).

<!-- links -->

[`str`]: https://doc.rust-lang.org/std/primitive.str.html
[`str::contains`]: https://doc.rust-lang.org/std/primitive.str.html#method.contains
[`str::replace`]: https://doc.rust-lang.org/std/primitive.str.html#method.replace
