pub mod minroot;

#[cfg(test)]
mod tests {
    use crate::minroot;

    #[test]
    fn it_works() {
	let num_steps = 1000;
	let num_iterations_per_step = 1024;
	let (pp, z0_primary, z0_secondary, rec) =
	    minroot::construct_recursive_snark(num_steps, num_iterations_per_step);
	assert!(minroot::verify(&pp, rec, z0_primary, z0_secondary, num_steps));
    }
}
