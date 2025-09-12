<table><tr>
<td colspan="1"> <h3 align="center"> <picture>
<source media="(prefers-color-scheme: dark)" srcset="https://PlayForm.Cloud/Dark/Image/GitHub/Land.svg">
<source media="(prefers-color-scheme: light)" srcset="https://PlayForm.Cloud/Image/GitHub/Land.svg">
<img width="28" alt="Land Logo" src="https://PlayForm.Cloud/Image/GitHub/Land.svg">
</picture> </h3> </td> <td colspan="3" valign="top"> <h3 align="center"> Common 👨🏻‍🏭
</h3> </td>
</tr></table>

---

# **Common** 👨🏻‍🏭 Deep Dive & Architecture

This document provides a detailed technical overview of the **Common** crate for
developers contributing to the Land project. It explores the internal
architecture, the core design patterns, and the philosophy behind the
effects-based system that powers the entire native backend.

## Core Philosophy

The architecture of the `Common` crate is built on two fundamental principles:

1.  **Separation of Concerns (The "What" vs. The "How"):** `Common` is strictly
    concerned with defining **what** the application can do. It does this by
    defining abstract `trait`s (e.g., `trait FileSystemReader`). It has
    absolutely no knowledge of **how** these actions are performed. The concrete
    implementation (the "how") is the responsibility of another crate,
    `Mountain`, which might use `tokio::fs` or a network client to fulfill the
    contract. This is a form of the "Ports and Adapters" architecture.

2.  **Declarative Logic (Effects as Values):** Instead of executing side effects
    directly, our system describes them as data. An `ActionEffect` is a value
    that represents an asynchronous operation. This declarative approach makes
    our logic highly composable, easier to reason about, and significantly more
    testable, as we can test the logic that _creates_ the effects without ever
    touching a real filesystem or network.

---

## Detailed Component Breakdown

### 1. The `ActionEffect` System (`Effect/`)

This is the most important concept in the `Common` crate.

- **`ActionEffect<TCapability, TError, TOutput>` Struct:**

    - This struct does not contain logic. It simply wraps an
      `Arc<dyn Fn(...) -> Future>`.
    - The function it wraps is the "potential" operation. The `Arc` makes the
      effect cheap to clone and pass around.
    - The type parameters define the contract:
        - `TCapability`: The type of capability the effect's closure needs to
          run (e.g., `Arc<dyn FileSystemReader>`).
        - `TError`: The type of error it can fail with (always `CommonError`).
        - `TOutput`: The type of value it will produce on success.

- **`ApplicationRunTime` Trait:**
    - This trait defines the contract for an "executor". Its primary method is
      `Run`.
    - The `Run` method is what takes an `ActionEffect` value, provides it with
      the required capability from its environment, and finally `await`s the
      future, executing the side effect.

**Conceptual Flow:**

```rust
// 1. A constructor function in `Common` creates an effect.
//    No I/O happens here. It just creates a struct.
let read_effect = Common::FileSystem::ReadFile(path);

// 2. The effect is passed to a runtime.
let result = my_runtime.Run(read_effect).await;

// 3. Inside the runtime's Run method:
//    a. It gets its environment using self.GetEnvironment().
//    b. It asks the environment for the required capability: `env.Require::<Arc<dyn FileSystemReader>>()`.
//    c. It calls the effect's internal function, passing in the required capability.
//    d. The function is awaited, and the actual tokio::fs::read() call is finally made.
```

### 2. The Dependency Injection System (`Environment/`)

- **`Environment` Trait:** A simple marker trait. Any struct that can act as a
  dependency container for our application must implement this trait.
- **`Requires<Capability>` Trait:** This is the core of the DI system. A struct
  implementing `Requires<T>` guarantees that it can provide an instance of `T`.
- **Usage:** Our `MountainEnvironment` will implement
  `Requires<Arc<dyn FileSystemReader>>`,
  `Requires<Arc<dyn UserInterfaceProvider>>`, etc., for every service trait. The
  `ApplicationRunTime` uses these implementations to provide the necessary
  dependencies to the effects it runs. This decouples the effects from the
  concrete environment.

### 3. The Service Provider Pattern

Every functional domain in `Common` (e.g., `FileSystem`, `UserInterface`,
`Command`) follows a strict pattern:

1.  **A Trait Definition (`MyService/MyServiceProvider.rs`):** An `async trait`
    that defines the high-level capabilities of the service (e.g.,
    `trait FileSystemReader { async fn ReadFile(...); }`).
2.  **DTOs (`MyService/DTO/`):** A submodule containing all `serde`-compatible
    `struct`s and `enum`s that are used as parameters or return types for the
    service's methods. These DTOs form the stable data contract.
3.  **Effect Constructors (`MyService/MyEffect.rs`):** A set of public
    functions, one for each method in the service trait. Each function takes the
    same arguments as the trait method but returns an `ActionEffect` instead of
    executing the logic directly.

### 4. The Universal Error (`Error/CommonError.rs`)

- To maintain predictability, every `ActionEffect` returns a
  `Result<T, CommonError>`.
- `CommonError` is a single, comprehensive `enum` that covers all possible
  failure domains (FileSystem, IPC, UI, etc.).
- This pattern allows consumers of effects to handle all possible errors with a
  single `match` statement, while still providing specific, tagged error
  variants for precise handling when needed. It uses `thiserror::Error` for
  clean, derivable error implementations.

---

## How to Add a New Service to `Common`

Adding a new capability to the application follows a clear, repeatable recipe:

1.  **Create the Module:** Create a new directory, e.g., `Source/NewService/`,
    and a `mod.rs` inside it.

2.  **Define the Trait:** In `Source/NewService/NewServiceProvider.rs`, define
    the new `async trait`:

    ```rust
    #[async_trait]
    pub trait NewServiceProvider: Environment {
        async fn DoSomething(&self, Options: MyOptionsDTO) -> Result<MyResultDTO, CommonError>;
    }
    ```

3.  **Define the DTOs:** In `Source/NewService/DTO/`, create files for
    `MyOptionsDto.rs` and `MyResultDto.rs` with
    `#[derive(Serialize, Deserialize)]`.

4.  **Update `CommonError`:** In `Source/Error/CommonError.rs`, add a new
    variant for failures related to your service:

    ```rust
    #[error("NewService failed: {Reason}")]
    NewServiceError { Reason: String },
    ```

5.  **Create the Effect Constructor:** In `Source/NewService/DoSomething.rs`,
    create the function that returns the `ActionEffect`. Note its signature:
    it's no longer generic, and its dependency is explicit.

    ```rust
    use std::sync::Arc;
    use crate::Effect::ActionEffect;
    use crate::Error::CommonError;
    use super::NewServiceProvider;

    pub fn DoSomething(
        Options: MyOptionsDTO,
    ) -> ActionEffect<Arc<dyn NewServiceProvider>, CommonError, MyResultDTO> {
        ActionEffect::New(Arc::new(move |Provider: Arc<dyn NewServiceProvider>| {
            let OptionsClone = Options.clone();
            Box::pin(async move {
                Provider.DoSomething(OptionsClone).await
            })
        }))
    }
    ```

6.  **Export from Modules:** Update `Source/NewService/mod.rs` and
    `Source/Library.rs` to publicly export the new trait, DTOs, and effect
    constructor.

---

## Relationship to Other Components

- **`Mountain`:** `Mountain` is the primary **implementor** of the traits in
  `Common`. The `MountainEnvironment` struct will have
  `impl NewServiceProvider for MountainEnvironment` blocks that contain the
  concrete logic. `Mountain` is also the home of the `ApplicationRunTime` that
  **executes** the effects.
- **`Cocoon`:** `Cocoon` is a remote **consumer**. When `Cocoon` sends a
  request, `Mountain`'s `Track` dispatcher creates the corresponding
  `ActionEffect` from `Common` and runs it. The DTOs defined in `Common` serve
  as the data contract for this communication.

This clear separation ensures that `Common` remains the universal, abstract
blueprint for all native backend functionality.
