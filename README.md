# Pybites Rust Bite Exercises

Thanks for stopping by. This repo allows you to suggest ideas for exercises and write your own.

## How to suggest an idea

1. [Create an issue](https://github.com/pybites/rust-bite-exercises/issues) with a short description of your idea.
2. If you want to write the exercise, mention this in the issue.

## How to write your own exercise

1. Fork this repo.
2. Create a new directory with the exercise name (use underscores): `cp -r template my_exercise_name`
3. Add your exercise code and tests to the `main.rs` template file.
4. Copy the `main.rs` file to `main-template.rs` (`cp main.rs main-template.rs`) and remove the solution code here (not the tests). This is the file that users will see when they first open the exercise.
5. Update `exercise.md` with the meta data, title and description (keep it concise please).
6. Include your `Cargo.toml` file with the dependencies needed to run your exercise.
7. Open a PR with your new exercise, we will review and when all good, merge it in.
8. After merge an admin will add the exercise to the platform crediting you as the author.

Happy coding and let's learn more Rust together! 🦀
