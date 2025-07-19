import { Actor, HttpAgent } from '@dfinity/agent';

import { idlFactory as profileIdlFactory, canisterId as profileCanisterId } from '../../declarations/profile_backend';
import { idlFactory as reviewIdlFactory, canisterId as reviewCanisterId } from '../../declarations/registry_backend';

const agent = new HttpAgent();

export const profileActor = Actor.createActor(profileIdlFactory, {
  agent,
  canisterId: profileCanisterId,
});

export const reviewActor = Actor.createActor(reviewIdlFactory, {
  agent,
  canisterId: reviewCanisterId,
});
