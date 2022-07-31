pragma solidity >=0.7.0 <0.9.0;

contract IbcPizzaOrderProtocol {

    uint256 orderAmount;
    uint256 deliverAmount;

    address payable customer;
    address payable shop;
    address payable deliverer;
    address payable developer;

    function beginOrder() public payable {
    }

    function beginDeliver() public payable {
    }


    // get back deposit
    function completeOrder() public payable {
    }

    // get back deposit
    function completeDeliver() public payable {
    }


}