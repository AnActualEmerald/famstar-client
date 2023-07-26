import { Peer, Replica, Syncer } from "@forge/earthstar";
import { ReplicaDriverWeb } from "@forge/earthstar/browser";

let localReplica: Replica;
let localPeer: Peer;
let syncer: Syncer<undefined, unknown>;

export const init = (address: string, server: string) => {
    localReplica = new Replica({
        driver: new ReplicaDriverWeb(address)
    });

    localPeer = new Peer();
    localPeer.addReplica(localReplica);

    syncer = localPeer.sync(server, true);

};

export { syncer, localReplica };