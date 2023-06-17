# Blood Donation dApp on NEAR Protocol



The Blood Donation dApp is designed to streamline the blood donation process by connecting blood donors with blood recipients in a decentralized manner. It utilizes the NEAR Protocol, a blockchain platform, to ensure transparency, security, and immutability.

Key features of the Blood Donation dApp include:

- User registration for both blood donors and recipients.
- Ability for blood donors to record their donation history and available blood types.
- Blood recipients can search for donors based on specific blood types and location.
- A trust system to build credibility and encourage active participation.

## Prerequisites

Before running the Blood Donation dApp locally, make sure you have the following prerequisites:

- Node.js: [https://nodejs.org](https://nodejs.org) (version 14 or above)
- NEAR CLI: Installation instructions can be found at [https://docs.near.org/docs/tools/near-cli](https://docs.near.org/docs/tools/near-cli)

## Getting Started

Follow these steps to set up and run the Blood Donation dApp locally:

1. Clone the repository or download the source code as a ZIP file.
2. Open a terminal and navigate to the project directory.
3. Install the project dependencies by running the following command:
   ```
   npm install
   ```
4. Once the installation is complete, start the local development server using the command:
   ```
   npm run dev
   ```
5. The dApp will be accessible in your web browser at `http://localhost:1234`.

## Project Structure

The project structure is organized as follows:

- `src/`: This directory contains the application source code.
  - `components/`: This directory contains the React components used in the dApp.
  - `pages/`: This directory contains the different pages of the dApp.
  - `contracts/`: This directory contains the smart contracts written in Rust.
- `public/`: This directory contains the public assets for the dApp.
- `neardev/`: This directory contains the compiled smart contracts and configuration files.

## Usage

To use the Blood Donation dApp, follow these steps:

1. Access the dApp through your web browser at `http://localhost:1234` or the deployed URL.
2. Register as a blood donor or recipient by providing the required information.
3. Blood donors can record their donation history and specify their available blood types.
4. Blood recipients can search for donors based on specific blood types and location.
5. The trust system helps establish credibility and encourages active participation.

## Deployment

To deploy the Blood Donation dApp to a live network, follow the NEAR Protocol deployment process. The detailed deployment instructions can be found at [https://docs.near.org/docs/develop/deploy/js/deploy-contract](https://docs.near.org/docs/develop/deploy/js/deploy-contract).

## Contributing

Contributions to the Blood Donation dApp are welcome! If you would like to contribute, please follow these guidelines:

1. Fork the repository and create a new branch for your contribution.
2. Make your changes and ensure that the code adheres