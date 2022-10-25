export type AdapterSaber = {
  "version": "0.1.0",
  "name": "adapter_saber",
  "instructions": [
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
    },
    {
      "name": "stake",
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
      "name": "unstake",
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
      "name": "harvest",
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
      "name": "RemoveLiquidityInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          },
          {
            "name": "action",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "StakeInputWrapper",
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
      "name": "UnstakeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "shareAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "HarvestInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
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
      "name": "RemoveLiquidityOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dummy1",
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
      "name": "StakeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dummy1",
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
      "name": "UnstakeOutputWrapper",
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
      "name": "HarvestOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dummy1",
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
      "name": "PoolDirection",
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
    },
    {
      "code": 6001,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};

export const IDL: AdapterSaber = {
  "version": "0.1.0",
  "name": "adapter_saber",
  "instructions": [
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
    },
    {
      "name": "stake",
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
      "name": "unstake",
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
      "name": "harvest",
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
      "name": "RemoveLiquidityInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpAmount",
            "type": "u64"
          },
          {
            "name": "action",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "StakeInputWrapper",
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
      "name": "UnstakeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "shareAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "HarvestInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
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
      "name": "RemoveLiquidityOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dummy1",
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
      "name": "StakeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dummy1",
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
      "name": "UnstakeOutputWrapper",
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
      "name": "HarvestOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "dummy1",
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
      "name": "PoolDirection",
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
    },
    {
      "code": 6001,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};
