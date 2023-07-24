use crate::test_framework::custom_assert;
use crate::{println, print};
use crate::sv39_mmu::validate_virtual_address;
use crate::sv39_mmu::validate_physical_address;
use crate::sv39_mmu::errors;
use crate::sv39_mmu::*;

#[test_case]
pub fn sv39_mmu_tests(){
    println!("\n---------  Running sv39_mmu tests  ---------\n");
    test_validate_virtual_address_above_range();
    test_validate_virtual_address_within_range();
    test_validate_physical_address_within_range();
    test_validate_physical_address_above_range();
    test_validate_access_map_messed();
    test_validate_access_map_RWX();
    test_map_function_catches_bad_phy_addr();
    test_map_function_catches_bad_virt_addr();
    test_map_function_catches_bad_access_map();
}

fn test_validate_virtual_address_above_range(){
    let test_address: u64 = 2u64.pow(48); 
    let res = validate_virtual_address(test_address);
    let suc_msg = "test_validate_virtual_address_above_range    ....   [OK]";
    let fail_msg = "test_validate_virtual_address_above_range   ....    [FAIL]";
    custom_assert(false, res, suc_msg, fail_msg);
}

fn test_validate_virtual_address_within_range(){
    let test_address: u64 = 2u64.pow(39); 
    let res = validate_virtual_address(test_address);
    let suc_msg = "test_validate_virtual_address_within_range    ....   [OK]";
    let fail_msg = "test_validate_virtual_address_within_range   ....    [FAIL]";
    custom_assert(true, res, suc_msg, fail_msg);
}

fn test_validate_physical_address_within_range(){
    let test_address: u64 = 2u64.pow(56); 
    let res = validate_physical_address(test_address);
    let suc_msg = "test_validate_physical_address_within_range    ....   [OK]";
    let fail_msg = "test_validate_physical_address_within_range   ....    [FAIL]";
    custom_assert(true, res, suc_msg, fail_msg);
}

fn test_validate_physical_address_above_range(){
    let test_address: u64 = 2u64.pow(60); 
    let res = validate_physical_address(test_address);
    let suc_msg = "test_validate_physical_address_above_range    ....   [OK]";
    let fail_msg = "test_validate_physical_address_above_range   ....    [FAIL]";
    custom_assert(false, res, suc_msg, fail_msg);
}

fn test_validate_access_map_messed(){
    let test_map: u64 = 0b11110;
    let res = validate_access_map(test_map);
    let suc_msg = "test_validate_access_map_messed    ....   [OK]";
    let fail_msg = "test_validate_access_map_messed   ....    [FAIL]";
    custom_assert(false, res, suc_msg, fail_msg);
}

// ------------------  Validate_map_accesss functions  ------------------ //
fn test_validate_access_map_branch(){
    let test_map: u64 = 0b10000;
    let res = validate_access_map(test_map);
    let suc_msg = "test_validate_access_map_messed    ....   [OK]";
    let fail_msg = "test_validate_access_map_messed   ....    [FAIL]";
    custom_assert(false, res, suc_msg, fail_msg);
}

fn test_validate_access_map_RWX(){
    let test_map: u64 = 0b01110;
    let res = validate_access_map(test_map);
    let suc_msg = "test_validate_access_map_RWX    ....   [OK]";
    let fail_msg = "test_validate_access_map_RWX   ....    [FAIL]";
    custom_assert(true, res, suc_msg, fail_msg);
}

fn test_validate_access_map_RO(){
    let test_map: u64 = 0b00010;
    let res = validate_access_map(test_map);
    let suc_msg = "test_validate_access_map_RO    ....   [OK]";
    let fail_msg = "test_validate_access_map_RO   ....    [FAIL]";
    custom_assert(true, res, suc_msg, fail_msg);
}


// --------------------  test the Map Function -------------------------- //

fn test_map_function_catches_bad_phy_addr(){
    let bad_physical_address: u64 = 2u64.pow(61);
    let good_virtual_address: u64 = 2u64.pow(24);
    let good_access_map : u64 = 2u64; // read Only access map
    let good_root_table_adress: u64 = 2u64.pow(20);

    let res = map(good_virtual_address, bad_physical_address, good_access_map, good_root_table_adress); 
    match res {
        Ok(x) => println!("test_map_function_catches_bad_phy_addr ...  [FAIL]"),
        Err(x) => {
            if x == MappingError::InvalidPhysicalAddress("Invalid Physical address passed to mapping function"){
                println!("test_map_function_catches_bad_phy_addr ...  [OK]")
            }
        }
    }
}

fn test_map_function_catches_bad_virt_addr(){
    let good_physical_address: u64 = 2u64.pow(48);
    let bad_virtual_address: u64 = 2u64.pow(48);
    let good_access_map : u64 = 2u64; // read Only access map
    let good_root_table_adress: u64 = 2u64.pow(20);

    let res = map(bad_virtual_address, good_physical_address, good_access_map, good_root_table_adress); 
    match res {
        Ok(x) => println!("test_map_function_catches_bad_virt_addr ...  [FAIL]"),
        Err(x) => {
            if x == MappingError::InvalidVirtualAddress("Invalid Virtual address passed to mapping function"){
                println!("test_map_function_catches_bad_virt_addr ...  [OK]")
            }
        }
    }
}

fn test_map_function_catches_bad_access_map(){
    let good_physical_address: u64 = 2u64.pow(48);
    let good_virtual_address: u64 = 2u64.pow(24);
    let bad_access_map : u64 = 0b100110; 
    let good_root_table_adress: u64 = 2u64.pow(20);

    let res = map(good_virtual_address, good_physical_address, bad_access_map, good_root_table_adress); 
    match res {
        Ok(x) => println!("test_map_function_catches_bad_access_map ...  [FAIL]"),
        Err(x) => {
            if x == MappingError::InvalidAccessMap("Invalid access_map passed to mapping function"){
                println!("test_map_function_catches_bad_access_map ...  [OK]")
            }
        }
    }
}