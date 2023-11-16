// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

contract ValueIncrementer {
    string public name;
    uint private value;

    constructor(string memory _name) {
        name = _name;
        value = 0;
    }

    function incrementValue() public {
        value += 1;
    }

    function getValue() public view returns (uint) {
        return value;
    }
}
