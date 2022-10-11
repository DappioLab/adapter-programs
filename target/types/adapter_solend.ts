export type AdapterSolend = {
  "version": "0.1.0",
  "name": "adapter_solend",
  "instructions": [
    {
      "name": "supply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unsupply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "borrow",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "repay",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "types": [
    {
      "name": "GatewayStateWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "discriminator",
            "type": "u64"
          },
          {
            "name": "userKey",
            "type": "publicKey"
          },
          {
            "name": "randomSeed",
            "type": "u64"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "currentIndex",
            "type": "u8"
          },
          {
            "name": "queueSize",
            "type": "u8"
          },
          {
            "name": "protocolQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "actionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "versionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "payloadQueue",
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "swapMinOutAmount",
            "type": "u64"
          },
          {
            "name": "poolDirection",
            "type": "u8"
          }
        ]
      }
    }
  ]
};

export const IDL: AdapterSolend = {
  "version": "0.1.0",
  "name": "adapter_solend",
  "instructions": [
    {
      "name": "supply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unsupply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "borrow",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "repay",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "types": [
    {
      "name": "GatewayStateWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "discriminator",
            "type": "u64"
          },
          {
            "name": "userKey",
            "type": "publicKey"
          },
          {
            "name": "randomSeed",
            "type": "u64"
          },
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "currentIndex",
            "type": "u8"
          },
          {
            "name": "queueSize",
            "type": "u8"
          },
          {
            "name": "protocolQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "actionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "versionQueue",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "payloadQueue",
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "swapMinOutAmount",
            "type": "u64"
          },
          {
            "name": "poolDirection",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
