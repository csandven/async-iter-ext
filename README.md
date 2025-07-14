# async-iter-ext

[![codecov](https://codecov.io/gh/csandven/async-iter-ext/graph/badge.svg?token=XUM5L9BOYR)](https://codecov.io/gh/csandven/async-iter-ext)

Async iterator methods and async methods for option and result.

--- 

### How to use with Cargo:

```toml
[dependencies]
async-iter-ext = "0.2.0"
```

### How to use in your crate:

```rust
use async_iter_ext::AsyncIterTools;
```

### Simple map_async example

```rust
use async_iter_ext::{AsyncIterTools, AsyncIterator};
use async_std::task::sleep;
use std::time::Duration;

#[async_std::main]
async fn main() {
  let items = [1, 2, 3, 4];

  let mapped_items_vec = items
    .iter()
    .map_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      item * 2
    })
    .async_collect::<Vec<_>>()
    .await;

  assert_eq!(mapped_items_vec.len(), items.len());
  assert_eq!(mapped_items_vec, vec![2, 4, 6, 8]);  
}
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
<https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<https://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.