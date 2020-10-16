extern crate powercap_sys as powercap;

use powercap::*;
use std::mem;

// CPU socket (count starts at 0)
const SOCKET: u32 = 0;
// Open as read-only (don't need elevated write permission)
const READ_ONLY: i32 = 1;
// Use the RAPL "Package" zone
const ZONE: powercap_rapl_zone = powercap_rapl_zone_POWERCAP_RAPL_ZONE_PACKAGE;
// Use the "long_term" constraint
const CONSTRAINT: powercap_rapl_constraint = powercap_rapl_constraint_POWERCAP_RAPL_CONSTRAINT_LONG;

fn main() {
    // First initialize the context struct
    let mut pkg = mem::MaybeUninit::<powercap_rapl_pkg>::uninit();
    if unsafe { powercap_rapl_init(SOCKET, pkg.as_mut_ptr(), READ_ONLY) } != 0 {
        panic!("Failed to init powercap")
    }
    let mut pkg: powercap_rapl_pkg = unsafe { pkg.assume_init() };
    // Read the power cap and print it
    let mut val: u64 = 0;
    match unsafe { powercap_rapl_get_power_limit_uw(&pkg, ZONE, CONSTRAINT, &mut val) } {
        0 => println!("Power cap = {} uW", val),
        _ => println!("Failed to get power")
    };
    // Clean up
    if unsafe { powercap_rapl_destroy(&mut pkg) } != 0 {
        panic!("Failed to destroy powercap")
    }  
}
