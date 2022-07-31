// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

contract IbcPizzaProtocol {

    uint256 goodCount;
    uint256 badCount;
    uint256 goodAmount;
    uint256 badAmount;

    address payable customer;
    address payable shop;
    address payable deliverer;

    function recommendShop() public payable {
        if (msg.value>= goodAmount) {
            goodCount++;
            (bool sent,) = shop.call{value: msg.value}("");
            require(sent, "Failed to send Ether");
        }
    }

    function notRecommendShop() public payable {
        if (msg.value>= badAmount) {
            badCount++;
            (bool sent,) = shop.call{value: msg.value}("");
            require(sent, "Failed to send Ether");
        }
    }

    function getGoodAmount() public view returns (uint256){
        return goodAmount;
    }

    function getBadAmount() public view returns (uint256){
        return badAmount;
    }
}