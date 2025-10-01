import {
  address,
  appendTransactionMessageInstructions,
  assertIsTransactionWithinSizeLimit,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  devnet,
  getSignatureFromTransaction,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  addSignersToTransactionMessage,
  getProgramDerivedAddress,
  generateKeyPairSigner,
  getAddressEncoder,
} from "@solana/kit";
import {
  getInitializeInstruction,
  getSubmitTsInstruction,
  getCloseInstruction,
} from "./clients/js/src/generated/index";

const MPL_CORE_PROGRAM = address(
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
);
const PROGRAM_ADDRESS = address("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");
const SYSTEM_PROGRAM = address("11111111111111111111111111111111");

import wallet from "./main-wallet.json";
const LAMPORTS_PER_SOL = BigInt(1_000_000_000);
const keypair = await createKeyPairSignerFromBytes(new Uint8Array(wallet));

const rpc = createSolanaRpc(devnet("https://api.devnet.solana.com"));
const rpcSubscriptions = createSolanaRpcSubscriptions(
  devnet("ws://api.devnet.solana.com"),
);

const addressEncoder = getAddressEncoder();

const accountSeeds = [
  Buffer.from("prereqs"),
  addressEncoder.encode(keypair.address),
];

const [account, _bump] = await getProgramDerivedAddress({
  programAddress: PROGRAM_ADDRESS,
  seeds: accountSeeds,
});

const COLLECTION = address("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const mintKeyPair = await generateKeyPairSigner();

// const initializeIx = getInitializeInstruction({
//   github: "xydv",
//   user: keypair,
//   account,
//   systemProgram: SYSTEM_PROGRAM,
// });

const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

// const transactionMessageInit = pipe(
//   createTransactionMessage({ version: 0 }),
//   (tx) => setTransactionMessageFeePayerSigner(keypair, tx),
//   (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
//   (tx) => appendTransactionMessageInstructions([initializeIx], tx),
// );

// const signedTxInit = await signTransactionMessageWithSigners(
//   transactionMessageInit,
// );

// assertIsTransactionWithinSizeLimit(signedTxInit);

const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
  rpc,
  rpcSubscriptions,
});

// try {
//   const result = await sendAndConfirmTransaction(signedTxInit, {
//     commitment: "confirmed",
//     skipPreflight: false,
//   });

//   console.log(result);

//   const signatureInit = getSignatureFromTransaction(signedTxInit);

//   console.log(
//     `Success! Check out your TX here: https://explorer.solana.com/tx/${signatureInit}?cluster=devnet`,
//   );
// } catch (e) {
//   console.error(e);
// }

const authoritySeeds = [
  Buffer.from("collection"),
  addressEncoder.encode(COLLECTION),
];

const [authority, _authorityBump] = await getProgramDerivedAddress({
  programAddress: PROGRAM_ADDRESS,
  seeds: authoritySeeds,
});

const submitIx = getSubmitTsInstruction({
  user: keypair,
  account,
  mint: mintKeyPair,
  collection: COLLECTION,
  authority,
  mplCoreProgram: MPL_CORE_PROGRAM,
  systemProgram: SYSTEM_PROGRAM,
});

// const closeIx = getCloseInstruction({
//   account,
//   user: keypair.address,
//   systemProgram: SYSTEM_PROGRAM,
// });

const transactionMessageSubmit = pipe(
  createTransactionMessage({ version: 0 }),
  (tx) => setTransactionMessageFeePayerSigner(keypair, tx),
  (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
  (tx) => appendTransactionMessageInstructions([submitIx], tx),
  (tx) => addSignersToTransactionMessage([mintKeyPair], tx),
);

const signedTxSubmit = await signTransactionMessageWithSigners(
  transactionMessageSubmit,
);

assertIsTransactionWithinSizeLimit(signedTxSubmit);

try {
  await sendAndConfirmTransaction(signedTxSubmit, {
    commitment: "confirmed",
    skipPreflight: false,
  });

  const signatureSubmit = getSignatureFromTransaction(signedTxSubmit);

  console.log(
    `Success! Check out your TX here: https://explorer.solana.com/tx/${signatureSubmit}?cluster=devnet`,
  );
} catch (e) {
  console.error(e);
}
