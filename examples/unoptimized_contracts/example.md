# Example of struct packing

input

```solidity
    struct RequestMeta {
        uint64 completedRequests;
        Custom.Datatype data;
        address requestingContract;
        uint72 adminFee; // in wei
        address subscriptionOwner;
        bytes32 flags; // 32 bytes of flags
        uint96 availableBalance; // in wei. 0 if not specified.
        uint64 subscriptionId;
        uint64 initiatedRequests;// number of requests initiated by this contract
        uint32 callbackGasLimit;
        uint16 dataVersion;
    }
```

Expected output

```solidity
    struct RequestMeta {
        Custom.Datatype data; //
        bytes32 flags; //                  32 bytes of flags
        address requestingContract; // ──╮
        uint96 availableBalance; // ─────╯ in wei. 0 if not specified.
        address subscriptionOwner; // ───╮
        uint64 completedRequests; //     │
        uint32 callbackGasLimit; // ─────╯
        uint72 adminFee; // ─────────────╮ in wei
        uint64 subscriptionId; //        │
        uint64 initiatedRequests; //     │ number of requests initiated by this contract
        uint16 dataVersion; // ──────────╯
    }
```
