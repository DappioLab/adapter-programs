export type AdapterTulip = {
  "version": "0.1.0",
  "name": "adapter_tulip",
  "instructions": [
    {
      "name": "deposit",
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
      "name": "withdraw",
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
      "name": "supply",
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
      "name": "unsupply",
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
      "name": "DepositInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpOrTokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
            "type": "u64"
          },
          {
            "name": "farmType0",
            "type": "u64"
          },
          {
            "name": "farmType1",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "WithdrawInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "shareAmount",
            "type": "u64"
          },
          {
            "name": "farmType0",
            "type": "u64"
          },
          {
            "name": "farmType1",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "supplyAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "DepositOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "shareAmount",
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
      "name": "WithdrawOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpOrTokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
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
      "name": "SupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
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
      "name": "UnsupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unsupplyAmount",
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedVaultProtocol",
      "msg": "Unsupported Vault Protocol"
    },
    {
      "code": 6001,
      "name": "IndexOutOfBound",
      "msg": "Index might out of bound, currently only support 30 addresses"
    }
  ]
};

export const IDL: AdapterTulip = {
  "version": "0.1.0",
  "name": "adapter_tulip",
  "instructions": [
    {
      "name": "deposit",
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
      "name": "withdraw",
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
      "name": "supply",
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
      "name": "unsupply",
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
      "name": "DepositInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpOrTokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
            "type": "u64"
          },
          {
            "name": "farmType0",
            "type": "u64"
          },
          {
            "name": "farmType1",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "WithdrawInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "shareAmount",
            "type": "u64"
          },
          {
            "name": "farmType0",
            "type": "u64"
          },
          {
            "name": "farmType1",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "supplyAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "DepositOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "shareAmount",
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
      "name": "WithdrawOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpOrTokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
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
      "name": "SupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reservedAmount",
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
      "name": "UnsupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "unsupplyAmount",
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedVaultProtocol",
      "msg": "Unsupported Vault Protocol"
    },
    {
      "code": 6001,
      "name": "IndexOutOfBound",
      "msg": "Index might out of bound, currently only support 30 addresses"
    }
  ]
};
