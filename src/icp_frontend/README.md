# 🧠 Persona ICP — Decentralized Identity & Review System

## 📌 Overview

**Persona ICP** is a decentralized platform built on the **Internet Computer Protocol (ICP)** that allows users to enroll, manage, and query verified professional identities. Each identity includes profile data and peer/client reviews, all securely stored on-chain using **Rust-based canisters**. The frontend is built with **React + Vite**, providing a clean and modern user experience.

This project aims to eliminate centralized gatekeepers in identity verification by leveraging the power of Web3.

---

## 🚀 Features

### 🔐 Identity Management
- Users can **enroll** by submitting their profile details (first name, last name, skills).
- Data is uniquely associated with a **user passkey** (a string or principal).
- Peer/client reviews can also be recorded.

### 🔍 Profile Querying
- Retrieve full user profiles by passkey.
- Filter specific fields like `skills`, `firstName`, etc.
- Backend enforces integrity with strong typing via Candid.

### 🌐 Technologies Used
- 🧠 **Rust** for secure, high-performance backend (canisters)
- 💾 **Candid** interface for type-safe serialization
- 🌍 **React + TypeScript + Vite** frontend
- 📡 **DFINITY Agent** to interface with canisters
- 🧪 **DFX** for local development and deployment

---

## 🛠️ Project Structure

persona/icp/
├── src/
│ ├── icp_frontend/ # React + Vite frontend
│ │ ├── src/
│ │ │ ├── components/ # UI Components
│ │ │ ├── contexts/ # React Context (e.g., Auth)
│ │ │ ├── hooks/ # Custom hooks (e.g., use-toast)
│ │ │ ├── pages/ # Page routes (e.g., Enroll, Dashboard)
│ │ │ └── icpConnect.ts # Connects to backend actors
│ │ └── vite.config.ts # Vite config
│
│ ├── rust_profile_canister_backend/ # Profile write/read logic
│ └── rust_profile_canister_backend_registry/ # Optional registry lookup
│
├── dfx.json # DFX project config
├── tsconfig.json # TypeScript config
├── package.json # Frontend dependencies
└── README.md


---

## ⚙️ Installation & Running

### 🧩 Prerequisites
- Node.js (18+)
- DFX SDK (`dfx --version` should work)
- Rust (with `wasm32-unknown-unknown` target installed)

### 📦 Install dependencies
```bash
cd src/icp_frontend
npm install

🔨 Start local canisters

dfx start --background
dfx deploy

🌐 Run the frontend

cd src/icp_frontend
npm run dev

Open http://localhost:5173 in your browser.
🧪 Testing the Backend (Optional)

Use dfx canister call to manually test write/read:
📤 Write:

dfx canister call profile_backend write '(
  "your-user-passkey",
  record {
    firstName = "Alice";
    lastName = "Wonderland";
    skills = "Rust, React";
    clientReviews = "Excellent developer"
  }
)'

📥 Read:

dfx canister call profile_backend get_user_details_by_passkey '("your-user-passkey")'

✅ Example Flow

    User opens the Enroll page and fills in personal and platform details.

    Submits form → profileActor.write() called.

    Data is stored in the Rust canister.

    Any authenticated client can fetch data using read() or get_user_details_by_passkey().

✨ Acknowledgments

    DFINITY for the Internet Computer ecosystem

    candid, ic-cdk, and agent-js for low-level canister communication
