// Import
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";

// Construct
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create({ provider: wsProvider });

const keyring = new Keyring({ type: "sr25519" });

// Add Alice to our keyring (she is our sender)
const alice = keyring.addFromUri("//Alice");

const talles = keyring.addFromMnemonic(
  "funny joke rack badge clutch bargain aerobic kick equip announce sick stand end travel version",
);
console.log("Talles: " + talles.toJson());

// Do something
console.log(api.genesisHash.toHex());

// read something from pallet templateModule
let something = await api.query.templateModule.something();
console.log(something.toJSON());

// set value to doSomething on pallet templateModule
await api.tx.templateModule.doSomething(100).signAndSend(alice);

api.disconnect();
