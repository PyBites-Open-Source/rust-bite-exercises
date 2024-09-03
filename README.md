# Pybites Rust Bite Exercises

Thanks for stopping by. This repo allows you to suggest ideas for exercises and write your own.

## How to suggest an idea

1. [Create an issue](https://github.com/pybites/rust-bite-exercises/issues) with a short description of your idea.

2. If you want to write the exercise, mention this in the issue.

## How to write your own exercise

1. Fork this repo. If this is your first exercise, make sure you install the pre-commit hook: `pre-commit install` (we use `cargo fmt|check|clippy` to keep the code clean).

2. Create a new directory with the exercise name (use underscores): `cp -r template my_exercise_name`

3. Add your exercise code and tests to `src/lib.rs`.

4. Copy the `src/lib.rs` file to `src/lib-template.rs` (`cp src/main.rs src/main-template.rs`) and remove the tests (just so we have them once only) and update remove the part of the solution code you don't want users to see (so this version is what users will see when they first open the exercise).

5. Update `exercise.md` with the meta data (header), title and description using markdown (keep it concise please).

6. Update the `Cargo.toml` file if you need to include any dependencies in order to run the exercise, in that case list them under `[dependencies]` wit their exact version (and features if needed).

7. Validate the code will compile locally by running the tests in your exercise directory: `cargo test` (you can ignore dead code warnings for now, that's because the function(s) are not called outside of the tests).

8. Open a PR with your new exercise, we will review and when all good, merge it in. After merge your exercise will be added to [the platform](https://rustplatform.com) crediting your GitHub user account you listed in `exercise.md`.

Happy coding and let's learn more Rust together! 🦀
