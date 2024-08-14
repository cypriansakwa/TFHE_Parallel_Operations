This project implements several homomorphic procedures in parallel utilizing the TFHE (Tor's Fully Homomorphic Encryption) library, which includes optional execution modes. It supports both parallel and single-threaded execution, allowing users the flexibility to optimize performance based on their individual requirements.
## Overview
This project shows how to use the TFHE (Tor's Fully Homomorphic Encryption) library to execute homomorphic encryption operations with better performance due to parallelism. It supports both parallel and single-threaded execution of typical homomorphic operations, giving users the ability to optimize performance dependent on workload and dataset size.

The project, which supports addition, subtraction, multiplication, and division, uses Rayon for parallel execution and allows users to select the execution mode that best meets their needs. This is especially valuable for instances involving large-scale computations, where parallel processing can provide significant performance gains.
## Features
- **Parallel Execution:** Uses Rayon to execute homomorphic operations concurrently, which improves speed for huge datasets or complex computations.
- **Single-Threaded Execution:** Optionally perform operations sequentially for smaller workloads or when the overhead of parallelism outweighs the advantages.
- **Supported Operations:**
   - Addition
   - Subtraction
   - Multiplication
   - Division

## Performance Enhancement Through Parallelism
- **Parallel Execution with Rayon**
   - The Rayon library is used in the project to allow homomorphic operations to be executed in parallel. Rayon is a data parallelism library for Rust that facilitates concurrent processing and fully utilizes multi-core computers.
## How It Works
- **Parallelism Enabled:** Rayon supports parallelism, which allows operations like addition, subtraction, multiplication, and division to be performed concurrently over several threads. This enables the program to do numerous operations concurrently, which can considerably reduce total calculation time.
- **Execution Mode:** Users can select between parallel and single-threaded execution modes based on their requirements. Parallel mode is activated by calling ExecutionMode::Parallel, which causes Rayon to handle the operations concurrently. In contrast, single-threaded mode (ExecutionMode::SingleThreaded) performs actions sequentially rather than in parallel.
## Benefits
- **Improved Performance:** Parallel execution of huge datasets or complex computations can result in significant performance increases. This is accomplished by dividing the task across numerous CPU cores, which reduces processing time.
- **Efficient Resource Utilization:** Parallel processing makes the best use of available CPU cores, resulting in better resource utilization and faster execution times.
- **Scalability:** The technique scales well with the size of the dataset and the number of operations, making it appropriate for scenarios involving enormous amounts of data.
## Configuration
-Execution Modes
  - **Parallel:** Uses Rayon to parallelize operations, which can improve performance for large datasets or complex computations.
  - **SingleThreaded:** Executes operations sequentially in a single thread, which might be preferable for smaller datasets or cases where parallelism overhead outweighs benefits.
## Example Configuration in Code
 >```
> fn parallel_homomorphic_operations(mode: ExecutionMode) -> Result<(), Box<dyn std::error::Error>> {
> // Set the execution mode here
> let mode = ExecutionMode::Parallel; // or ExecutionMode::SingleThreaded
> // Perform operations
> // ...
> }

## Requirements
- **Rust:** Ensure you have the latest stable version of Rust installed.
- **Rust version:** a minimum Rust version of $1.73$ is required to compile TFHE-rs.
- **TFHE-rs:** This project uses the TFHE-rs library for fully homomorphic encryption. It is included as a dependency in the Cargo.toml file.
- **Rayon:** The Rayon library is used for parallel processing, also included as a dependency.
``` 
#To include library run: paste the line below in 'Cargo.toml' 
#For x86_64 machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "x86_64-unix" ] }
#For ARM machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "aarch64-unix" ] }
#For x86_64 machines with the rdseed instruction running Windows:

tfhe = { version = "*", features = ["boolean", "shortint", "integer", "x86_64"] }

#ensure to build cargo after adding the tfhe library
cargo run build
```
## Usage 
- To run the program and perform homomorphic operations in parallel mode, use
 >```
>  cargo run --release --parallel
- To run the program and perform homomorphic operations in single-threaded mode, use:
 >```
>  cargo run --release --single-threaded
- By default, the code runs in parallel mode. You can switch to single-threaded mode by modifying the parallel_homomorphic_operations function call in the main function:
>```
>  fn main() -> Result<(), Box<dyn std::error::Error>> {
>  parallel_homomorphic_operations(ExecutionMode::Parallel)?; // Change to ExecutionMode::SingleThreaded for single-threaded execution
> Ok(())
> }

 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Acknowledgments
- TFHE Library
- Rayon
- Rust
  
```bash
git clone https://github.com/cypriansakwa/TFHE_Parallel_Operations.git
cd TFHE_Parallel_Operations
