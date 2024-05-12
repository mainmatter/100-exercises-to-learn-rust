# Welcome

Welcome to **"100 Exercises To Learn Rust"**!  

This course will teach you Rust's core concepts, one exercise at a time. 
In roughly 100 exercises, you'll go from knowing nothing about Rust to feeling productive on your own.  

## Structure

Each section in this course introduces a new concept or feature of the Rust language.  
To verify your understanding, each section is paired with an exercise that you need to solve. 

Each exercise is structured as a Rust package, located in the `exercises` folder.
The package contains the exercise itself, instructions on what to do (in `src/lib.rs`), and a test suite to 
automatically verify your solution.

### `wr`, the workshop runner

To navigate through the course, you will be using the `wr` CLI (short for "workshop runner").  
Install it with:

```bash
cargo install --locked workshop-runner
```

In a new terminal, navigate back to the top-level folder of the repository.
Run the `wr` command to start the course:

```bash
wr
```

`wr` will verify the solution to the current exercise.  
Don't move on to the next section until you've solved the exercise for the current one.  

Enjoy the course!

## References

- The exercise for this section is located in `exercises/01_intro/00_welcome`