// SPDX-License-Identifier: MIT
pragma solidity >=0.7.0 <0.9.0;

contract Chat {

    address payable private immutable contractOwner;

    constructor(address _contractOwner) {
        contractOwner = payable(_contractOwner);
    }

    mapping(address => uint) public rewards;

    event NewQuestion(address creator, string question, uint256 token);

    // 새로운 작업 생성을 위한 함수
    // User가 사용하는 함수
    function newQuestion(address creator, string memory _question, uint256 token) public payable {
        // 콜을 보내기 위해서는 0.01 ETH 필요 
        require(msg.value >= 0.01 ether, "Minimum 0.01 ETH not met");

        // 전송 받은 ETH를 코어 프로세서 주소로 보냄 
        // 작업을 EVM 컨트랙트로 다시 보내기 위한 값임
        (bool success, ) = contractOwner.call{value: msg.value}("");
        require(success, "Transfer failed.");

        // 새로운 작업 이벤트를 배포함
        emit NewQuestion(creator, _question, token);
    }

    // coprocessor에 의해 보상이 지급될 때 호출되는 콜백 함수
		// coprocessor는 컨트랙트 주인을 뜻하는 것 같음
    // ICP chain fusion canister에서 실행하는 함수
    function callback(address creator, uint256 reward) public {
        require(
            msg.sender == contractOwner,
            "Only the contract Owner can call this function"
        );

        // 보상 증가
        rewards[creator] += reward;
    }

}