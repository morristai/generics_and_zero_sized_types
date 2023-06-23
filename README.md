# Pattern Design test for Github Copilot

## 1. (Use `1_bad_api.rs` for example) to ask Copilot for suggestion:
As you can see, when a user uses `CredManager` to create their manager instance,
they can freely call any method under the `CredManager` struct. 
This causes potential runtime errors or unexpected results. 
What I want is a better design for `CredManager`.
It should have a `lock` and `unlock` state, and these states should not occupy memory size. 
Additionally, each state should have its own methods.
For example, users should only call `list_tokens`, `add_token`, and `lock` when `CredManager` is unlocked.
This approach should capture misuse of the API at compile time.

