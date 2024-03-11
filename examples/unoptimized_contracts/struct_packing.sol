// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract NotOptimizedStruct {
    struct Employee {
        uint256 id;        // 32 bytes
        uint32 salary;     // 4 bytes
        uint32 age;        // 4 bytes
        bool isActive;     // 1 byte
        address addr;      // 20 bytes
        uint16 department; // 2 bytes
    }
}

/**
 In this contract, the Employee struct is not optimized because the variables are not ordered by their size to minimize gaps caused by Solidity's storage layout. Solidity stores variables in 32-byte slots, and when a variable does not fill the entire slot, it can be combined with other variables to minimize wasted space.

The optimization can be done by rearranging the struct fields to ensure that smaller fields are packed together within the 32-byte slots. Here's the optimized struct:

pragma solidity ^0.8.0;

contract OptimizedStruct {
    struct Employee {
        uint256 id;        // 32 bytes
        address addr;      // 20 bytes, next 12 bytes can be utilized by smaller types
        uint32 salary;     // 4 bytes, can be packed with age and department
        uint32 age;        // 4 bytes, can be packed with salary and department
        uint16 department; // 2 bytes, can be packed with salary and age
        bool isActive;     // 1 byte, can be combined with another 1 byte variable or occupy the remaining byte after other variables
        // Total: 63 bytes, but will occupy 64 bytes (2 slots) due to alignment
    }

}
In the optimized version, the address type is placed after the uint256 since they both don't completely fill up their slots and cannot be packed with any smaller types. The uint32 and uint16 types are placed next, allowing them to share a slot, and finally, the bool is placed at the end. This struct organization takes advantage of the space within the slots more efficiently, potentially reducing gas costs when storing and retrieving Employee struct instances.
 */