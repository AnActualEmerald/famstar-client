import { Crypto, Peer, Replica, ReplicaDriverWeb } from "./earthstar.web";

let localReplica;
let localPeer;
let syncer;

export const init = (address: string, server: string) => {
    localReplica = new Replica({
        driver: new ReplicaDriverWeb(address)
    });

    localPeer = new Peer();
    localPeer.addReplica(localReplica);

    syncer = localPeer.sync(server, true);

};

export { syncer, localReplica };