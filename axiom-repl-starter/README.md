# AxiomREPL Starter

This AxiomREPL Next.js 14 starter app provides a barebones framework through which you can utilize the circuit that you built in AxiomREPL to build a full dApp. This starter app is meant to be used in conjunction with [AxiomREPL](https://repl.axiom.xyz/).

Axiom-related code is mostly in `src/components/axiom/`

## Setup
1. Copy `.env.local.example` as a new file named `.env.local`
2. Fill in the values in `.env.local` with your own values
    a. `NEXT_PUBLIC_ALCHEMY_KEY` is your Alchemy API key; you'll need to sign up for an (Alchemy)[https://www.alchemy.com/] account
    b. `NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID` is your WalletConnect project ID; you'll need to sign up for a (WalletConnect)[https://walletconnect.com/] account
3. Run `npm install` (or `yarn`/`pnpm`) to install dependencies
4. Run `npm run dev` to start the local development server
