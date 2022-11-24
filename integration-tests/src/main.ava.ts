import { Worker, NEAR, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";
import * as ethers from "ethers";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();
  const root = worker.rootAccount;

  // some test accounts
  const alice = await root.createSubAccount("alice", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const contract = await root.createSubAccount("contract", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  // Get wasm file path from package.json test script in folder above
  await contract.deploy(process.argv[2]);

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { contract, alice };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("calculates ecrecover for personal_sign", async (t) => {
  // ARRANGE =======================

  const message = "Hello, World!";

  const wallet = ethers.Wallet.createRandom();
  const signature = await wallet.signMessage(message);

  const prefixedMessage = "\x19Ethereum Signed Message:\n" + message.length + message;
  const hash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(prefixedMessage)).slice(2, 66); // remove 0x

  const sig = signature.slice(2, 130); // first 64 bytes without 0x
  const v = signature.slice(130, 132); // last 1 byte

  // Transform yellow paper V from 27/28 to 0/1
  // More info:
  // https://stackoverflow.com/questions/49085737/geth-ecrecover-invalid-signature-recovery-id
  // https://github.com/ethereum/go-ethereum/blob/55599ee95d4151a2502465e0afc7c47bd1acba77/internal/ethapi/api.go#L459
  const compatibleV = parseInt('0x' + v) - 27;

  const expectedAddress = wallet.address.toLowerCase();

  // ACT ============================

  const { contract } = t.context.accounts;
  const result: any = await contract.view("eth_ecrecover", {
    data: {
      m: hash,
      sig: sig,
      v: compatibleV,
      mc: false // ToDo: check correctness
    }
  });

  const receivedAddress = '0x' + result.address.toLowerCase();

  // ASSERT =========================

  t.is(receivedAddress, expectedAddress);
})

test("calculates ecrecover for given signature", async (t) => {
  // ARRANGE =======================

  const message = "Example `personal_sign` message";
  const signature = "0xb6483ed445dbd979650106011bc1c6e6ef255c84404df7729bdaf8bda1ae428b19c13e1447bc3fdf618b351c951d992ee236b462dc3e8f9a6fd8dc952aa18ebe1b";
  const address = "0xaAF9E9Ce86D3f85ee15797B996c33eB720b185c0";
  
  const prefixedMessage = "\x19Ethereum Signed Message:\n" + message.length + message;
  const hash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(prefixedMessage)).slice(2, 66); // remove 0x

  const sig = signature.slice(2, 130); // first 64 bytes without 0x
  const v = signature.slice(130, 132); // last 1 byte

  // Transform yellow paper V from 27/28 to 0/1
  // More info:
  // https://stackoverflow.com/questions/49085737/geth-ecrecover-invalid-signature-recovery-id
  // https://github.com/ethereum/go-ethereum/blob/55599ee95d4151a2502465e0afc7c47bd1acba77/internal/ethapi/api.go#L459
  const compatibleV = parseInt('0x' + v) - 27;
  
  const expectedAddress = address.toLowerCase();

  // ACT ============================

  const { contract } = t.context.accounts;
  const result: any = await contract.view("eth_ecrecover", {
    data: {
      m: hash,
      sig: sig,
      v: compatibleV,
      mc: false // ToDo: check correctness
    }
  });

  const receivedAddress = '0x' + result.address.toLowerCase();

  // ASSERT =========================

  t.is(receivedAddress, expectedAddress);
});
