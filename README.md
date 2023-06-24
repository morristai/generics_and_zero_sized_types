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

Result: Copilot create a trait named `CredManagerState` and three structs named `LockedState`, `UnlockedState`, and `CredManager`.
However, the `LockedState` and `UnlockedState` structs are not zero sized structs. And API usage is totally not ideal.
Like how can I obtain an existing `LockedState` instance when I call the lock method in `UnlockedState`?
Furthermore, in the example, the lock method in UnlockedState returns `Box::new(LockedState::new(self.master_pass))`, 
which uses `self.master_pass` seemingly out of nowhere. This implies that whenever I unlock `LockedState`, I lose the `LockedState` instance I just created.

(In addition, the code provided by copilot often can't be compiled in first time. Need to modify it to make it work.
```rust
// 1. Compile error
   Compiling generics_and_zero_sized_types v0.1.0 (/home/morris_tai/Documents/whynot/generics_and_zero_sized_types)
error[E0277]: the trait bound `Box<LockedState>: CredManagerState` is not satisfied
--> examples/4_copilot_first_response.rs:62:9
|
62 |         Box::new(self)
|         ^^^^^^^^^^^^^^ the trait `CredManagerState` is not implemented for `Box<LockedState>`
|
= help: the following other types implement trait `CredManagerState`:
LockedState
UnlockedState
= note: required for the cast from `Box<LockedState>` to the object type `dyn CredManagerState`

// I found the answer is not I wanted. But I don't want to give Copilot too many hints.
```

## 2. (Use `1_bad_api.rs` for example) to ask Copilot for suggestion:

As you can see, when a user uses `CredManager` to create their manager instance,
they can freely call any method under the `CredManager` struct.
This causes potential runtime errors or unexpected results.
What I want is a better design for `CredManager`.
It should have a `lock` and `unlock` state, and these states should not occupy memory size.
Additionally, each state should have its own methods.
For example, users should only call `list_tokens`, `add_token`, and `lock` when `CredManager`'s state is unlocked.
This approach should capture misuse of the API at compile time.
Notice! Please don't use dynamic dispatch to give another non-zero sized struct to solve this problem.

Result: Copilot give up dynamically dispatching, but it still gives me three sized structs to solve this problem. 
Which is not ideal since I want to keep all field in one single struct.


## 3. (Use `1_bad_api.rs` for example) to ask Copilot for suggestion:

As you can see, when a user uses `CredManager` to create their manager instance,
they can freely call any method under the `CredManager` struct.
This causes potential runtime errors or unexpected results.
What I want is a better design for `CredManager`.
It should have a `lock` and `unlock` state, and these states should not occupy memory size.
Additionally, each state should have its own methods.
For example, users should only call `list_tokens`, `add_token`, and `lock` when `CredManager`'s state is unlocked.
This approach should capture misuse of the API at compile time.
**Please use type state pattern to solve this problem.**

Result: Copilot give a good answer, but it requires me to use key word **"type state pattern"** to get the answer.

## 4. (Use `1_bad_api.rs` for example) to ask Copilot for suggestion:

As you can see, when a user uses CredManager to create their manager instance,
they can freely call any method under the CredManager struct. This causes potential runtime errors or unexpected results.
What I want is a better design for CredManager. It should have a lock and unlock state,
and these states should not occupy memory size. Additionally, each state should have its own methods.
For example, users should only call list_tokens, add_token, and lock when CredManager's state is unlocked.
This approach should capture misuse of the API at compile time.
Please find a suitable pattern for modifying the code that allows users to utilize it intuitively.

Result: Copilot give a mediocre answer using enum to store state, but it's not perfect, since it can only capture misuse of the API at runtime.

## 5. (Use `1_bad_api.rs` for example) to ask Copilot for suggestion:
As you can see, when a user uses CredManager to create their manager instance,
they can freely call any method under the CredManager struct. This causes potential runtime errors or unexpected results.
What I want is a better design for CredManager. It should have a lock and unlock state,
and these states should not occupy memory size. Additionally, each state should have its own methods.
For example, users should only call list_tokens, add_token, and lock when CredManager's state is unlocked.
This approach should capture misuse of the API at compile time.
Please find a suitable pattern for modifying the code that allows users to utilize it intuitively, it would be better if you can use generic.

Result: The answer provide by Copilot is just tragedy.
