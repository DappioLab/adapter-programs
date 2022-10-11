export type AdapterFriktion = {
  "version": "0.1.0",
  "name": "adapter_friktion",
  "instructions": [
    {
      "name": "initiateDeposit",
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
      "name": "initiateWithdrawal",
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
      "name": "cancelWithdrawal",
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
      "name": "cancelDeposit",
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
      "name": "finalizeWithdrawal",
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
      "name": "finalizeDeposit",
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
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};

export const IDL: AdapterFriktion = {
  "version": "0.1.0",
  "name": "adapter_friktion",
  "instructions": [
    {
      "name": "initiateDeposit",
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
      "name": "initiateWithdrawal",
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
      "name": "cancelWithdrawal",
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
      "name": "cancelDeposit",
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
      "name": "finalizeWithdrawal",
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
      "name": "finalizeDeposit",
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
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};
