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
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "name": "SwapInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "swapInAmount",
            "type": "u64"
          },
          {
            "name": "swapMinOutAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SwapOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "swapOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "AddLiquidityInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenInAmount",
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
      "name": "AddLiquidityOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RemoveLiquidityInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
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
      "name": "RemoveLiquidityOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
            "type": "u64"
          },
          {
            "name": "lpAmount",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
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
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
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
      "name": "SwapInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "swapInAmount",
            "type": "u64"
          },
          {
            "name": "swapMinOutAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SwapOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "swapOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "AddLiquidityInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenInAmount",
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
      "name": "AddLiquidityOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RemoveLiquidityInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
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
      "name": "RemoveLiquidityOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
            "type": "u64"
          },
          {
            "name": "lpAmount",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
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
