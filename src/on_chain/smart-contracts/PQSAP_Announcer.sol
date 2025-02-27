// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

library Constants {
    uint256 constant PQ_SCHEME_ID = 3328;
}

interface IPQSAP_Announcer {
    function sendEthViaProxy(address payable _stealthAddress, bytes memory _R, bytes memory _viewTag)
    external
    payable;

    event Announcement(
    uint256 indexed schemeId,
    address indexed stealthAddress,
    address indexed caller,
    bytes ephemeralPubKey,
    bytes metadata
  );
}

contract PQSAP_Announcer is IPQSAP_Announcer {
      function sendEthViaProxy(address payable _stealthAddress, bytes memory _R, bytes memory _viewTag)
            external
            payable
        {
            _announce(_stealthAddress, _R, _viewTag);

            _stealthAddress.transfer(msg.value);
        }

  
    function _announce(address _stealthAddress, bytes memory _R, bytes memory _viewTag) internal {
        emit Announcement(Constants.PQ_SCHEME_ID, _stealthAddress, msg.sender, _R, _viewTag);
    }
}