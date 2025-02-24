@0xba022a6d25269e68;

interface Echo {
    struct EchoRequest {
        name @0 :Text;
    }

    struct EchoReply {
        message @0 :Text;
    }

    sayEcho @0 (request: EchoRequest) -> (reply: EchoReply);
}