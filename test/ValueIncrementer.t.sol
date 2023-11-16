// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {Test, console2} from "forge-std/Test.sol";
import {ValueIncrementer} from "../src/ValueIncrementer.sol";

contract ValueIncrementerTest is Test {
    ValueIncrementer public valueIncrementer;

    function setUp() public {
        valueIncrementer = new ValueIncrementer("ExampleName");
    }

    function test_Initial() public {
        assertEq(valueIncrementer.getValue(), 0);
    }

    function test_Increment() public {
        valueIncrementer.incrementValue();
        assertEq(valueIncrementer.getValue(), 1);
    }
}
