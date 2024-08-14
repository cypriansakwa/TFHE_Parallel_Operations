use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint64};
use rayon::prelude::*;

// Configuration enum to choose between parallel and single-threaded execution
#[derive(Clone, Copy)]
pub enum ExecutionMode {
    Parallel,
    SingleThreaded,
}

// Define a trait to encapsulate various homomorphic operations with configurable execution mode
pub trait HomomorphicOps {
    fn homomorphic_add(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self>
    where
        Self: Sized + Clone + Send + Sync;

    fn homomorphic_sub(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self>
    where
        Self: Sized + Clone + Send + Sync;

    fn homomorphic_mul(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self>
    where
        Self: Sized + Clone + Send + Sync;

    fn homomorphic_div(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self>
    where
        Self: Sized + Clone + Send + Sync;
}

// Implement the HomomorphicOps trait for FheUint64
impl HomomorphicOps for FheUint64 {
    fn homomorphic_add(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self> {
        let self_vec = vec![self; other.len()];
        match mode {
            ExecutionMode::Parallel => self_vec
                .into_par_iter()
                .zip(other.into_par_iter())
                .map(|(x, y)| x + y)
                .collect(),
            ExecutionMode::SingleThreaded => self_vec
                .into_iter()
                .zip(other.into_iter())
                .map(|(x, y)| x + y)
                .collect(),
        }
    }

    fn homomorphic_sub(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self> {
        let self_vec = vec![self; other.len()];
        match mode {
            ExecutionMode::Parallel => self_vec
                .into_par_iter()
                .zip(other.into_par_iter())
                .map(|(x, y)| x - y)
                .collect(),
            ExecutionMode::SingleThreaded => self_vec
                .into_iter()
                .zip(other.into_iter())
                .map(|(x, y)| x - y)
                .collect(),
        }
    }

    fn homomorphic_mul(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self> {
        let self_vec = vec![self; other.len()];
        match mode {
            ExecutionMode::Parallel => self_vec
                .into_par_iter()
                .zip(other.into_par_iter())
                .map(|(x, y)| x * y)
                .collect(),
            ExecutionMode::SingleThreaded => self_vec
                .into_iter()
                .zip(other.into_iter())
                .map(|(x, y)| x * y)
                .collect(),
        }
    }

    fn homomorphic_div(self, other: Vec<Self>, mode: ExecutionMode) -> Vec<Self> {
        let self_vec = vec![self; other.len()];
        match mode {
            ExecutionMode::Parallel => self_vec
                .into_par_iter()
                .zip(other.into_par_iter())
                .map(|(x, y)| x / y)
                .collect(),
            ExecutionMode::SingleThreaded => self_vec
                .into_iter()
                .zip(other.into_iter())
                .map(|(x, y)| x / y)
                .collect(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    parallel_homomorphic_operations(ExecutionMode::Parallel)?;
    Ok(())
}

fn parallel_homomorphic_operations(mode: ExecutionMode) -> Result<(), Box<dyn std::error::Error>> {
    // Configuration using small encryption parameters
    let config = ConfigBuilder::default_with_small_encryption();

    // Generate client and server keys
    let (client_key, server_key) = generate_keys(config);

    // Set server key for encrypted operations
    set_server_key(server_key);

    // Clear text values to be encrypted
    let clear_a = 1234567890123456789_u64;
    let clear_b = 9876543210987654321_u64;

    // Encrypt the clear text values
    let a = FheUint64::try_encrypt(clear_a, &client_key)?;
    let b = FheUint64::try_encrypt(clear_b, &client_key)?;

    // Perform homomorphic operations, cloning `a` each time it's used
    let result_add = a.clone().homomorphic_add(vec![b.clone()], mode);
    let result_sub = a.clone().homomorphic_sub(vec![b.clone()], mode);
    let result_mul = a.clone().homomorphic_mul(vec![b.clone()], mode);
    let result_div = a.homomorphic_div(vec![b], mode);

    // Decrypt the results
    let decrypted_add: Vec<u64> = result_add.into_iter().map(|c| c.decrypt(&client_key)).collect();
    let decrypted_sub: Vec<u64> = result_sub.into_iter().map(|c| c.decrypt(&client_key)).collect();
    let decrypted_mul: Vec<u64> = result_mul.into_iter().map(|c| c.decrypt(&client_key)).collect();
    let decrypted_div: Vec<u64> = result_div.into_iter().map(|c| c.decrypt(&client_key)).collect();

    // Clear text calculations for verification
    let clear_add = clear_a + clear_b;
    let clear_sub = clear_a - clear_b;
    let clear_mul = clear_a * clear_b;
    let clear_div = clear_a / clear_b;

    assert_eq!(decrypted_add[0], clear_add);
    assert_eq!(decrypted_sub[0], clear_sub);
    assert_eq!(decrypted_mul[0], clear_mul);
    assert_eq!(decrypted_div[0], clear_div);

    println!("Decrypted addition result: {:?}", decrypted_add[0]);
    println!("Decrypted subtraction result: {:?}", decrypted_sub[0]);
    println!("Decrypted multiplication result: {:?}", decrypted_mul[0]);
    println!("Decrypted division result: {:?}", decrypted_div[0]);

    Ok(())
}


