/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  sayHello,
  reportGreetings,
  getBalance,
} from './hello_world';

async function main() {
  console.log("Let's say hello to a Solana account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  const startingBalance = await getBalance();

  // Check if the program has been deployed
  await checkProgram();

  // Say hello to an account
  await sayHello();

  // Find out how many times that account has been greeted
  await reportGreetings();

  const balance = await getBalance();

  console.log('Success. Fees were', '$' + (startingBalance - balance) * 175, startingBalance - balance);
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
