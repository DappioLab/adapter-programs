export type AdapterLifinity = {
  "version": "0.1.0",
  "name": "adapter_lifinity",
  "instructions": [
    {
      "name": "swap",
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
      "name": "addLiquidity",
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
      "name": "removeLiquidity",
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
      "name": "GetLifinityLpPriceWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "coinBalance",
            "type": "u64"
          },
          {
            "name": "pcBalance",
            "type": "u64"
          },
          {
            "name": "coinToPcPrice",
            "type": "f64"
          },
          {
            "name": "pcToCoinPrice",
            "type": "f64"
          },
          {
            "name": "lpAmount",
            "type": "f64"
          }
        ]
      }
    },
    {
      "name": "SwapResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "outAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "AddLiquidityResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RemoveLiquidityResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "swapInAmount",
            "type": "u64"
          }
        ]
      }
    },
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
    },
    {
      "name": "PoolDirectionP",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Obverse"
          },
          {
            "name": "Reverse"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedPoolDirection",
      "msg": "Unsupported PoolDirection"
    }
  ]
};

export const IDL: AdapterLifinity = {
  "version": "0.1.0",
  "name": "adapter_lifinity",
  "instructions": [
    {
      "name": "swap",
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
      "name": "addLiquidity",
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
      "name": "removeLiquidity",
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
      "name": "GetLifinityLpPriceWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "coinBalance",
            "type": "u64"
          },
          {
            "name": "pcBalance",
            "type": "u64"
          },
          {
            "name": "coinToPcPrice",
            "type": "f64"
          },
          {
            "name": "pcToCoinPrice",
            "type": "f64"
          },
          {
            "name": "lpAmount",
            "type": "f64"
          }
        ]
      }
    },
    {
      "name": "SwapResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "outAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "AddLiquidityResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RemoveLiquidityResultWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "swapInAmount",
            "type": "u64"
          }
        ]
      }
    },
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
    },
    {
      "name": "PoolDirectionP",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Obverse"
          },
          {
            "name": "Reverse"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedPoolDirection",
      "msg": "Unsupported PoolDirection"
    }
  ]
};
