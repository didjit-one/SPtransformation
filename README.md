# SPtransformation

The program implements a practical task  "S-block and P-block"

Here is an example of the program output:

```
Forward S-box Transformation:
Input data: "10010111"
Output data: "11011000"
Backward S-box Transformation:
Output data: "11011000"
Input data: "10010111"
Forward P-box Transformation:
Input data: [1, 0, 0, 1, 0, 1, 1, 1]
Output data: [0, 1, 0, 1, 0, 1, 1, 1]
Backward P-box Transformation:
Output data: [0, 1, 0, 1, 0, 1, 1, 1]
Input data: [1, 0, 0, 1, 0, 1, 1, 1]
```

## Instructions

1. Set the input data for encryption by modifying the `INPUT_DATA` constant variable.

2. Run the program.


## Code Explanation

- The `s_box()` function implements the S-box algorithm.

- The `s_box_backward()` function performs the backward S-box transformation.

- The `s_box_forward()` function performs the forward S-box transformation.

- The `p_box()` function implements the P-box algorithm.

- The `p_box_backward()` function performs the backward P-box transformation.

- The `p_box_forward()` function performs the forward P-box transformation.

- The constant variable `INPUT_DATA` represents the input data for encryption.

## Running the Program on the site play.rust-lang.org.

To run your code on play.rust-lang.org, you can follow these steps:

Go to the website [play.rust-lang.org](https://play.rust-lang.org/) .

Clear the existing code on the page.

Paste your code into the empty field.

Click the "Run" button.

Your code will be compiled and executed in the output window on the right. The execution result will be displayed in that area.


## Running the Program on the computer

To run the program, execute the following steps:

1. Ensure you have the Rust programming language installed.

2. Open a terminal or command prompt.

3. Navigate to the directory containing the source code file.

4. Compile the program by running the command: `cargo build`.

5. Run the program by executing the command: `cargo run`.



## License

This program is released under the [MIT License](LICENSE).

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

