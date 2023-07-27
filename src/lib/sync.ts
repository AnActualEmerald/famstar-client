import { Peer, Replica, Syncer } from "@forge/earthstar";
import { ReplicaDriverWeb } from "@forge/earthstar/browser";

let localReplica: Replica;
let localPeer: Peer;
let syncer: Syncer<undefined, unknown>;

const ensureConnection = (address, server) => {
    if(!syncer) {
        syncer = localPeer.sync(server, true);
        return;
    }

    const status = syncer.getStatus()[address];
    if(status.docs.status === "aborted") {
        syncer.cancel();
        syncer = localPeer.sync(server, true);
        return;
    }
    
}

export const init = (address: string, server: string) => {
    localReplica = new Replica({
        driver: new ReplicaDriverWeb(address)
    });

    localPeer = new Peer();
    localPeer.addReplica(localReplica);

    ensureConnection(address, server);

    setInterval(() => {
        ensureConnection(address, server);
    }, 1000);


};

export { syncer, localReplica };
