export type AdapterNftFinance = {
  "version": "0.1.0",
  "name": "adapter_nft_finance",
  "instructions": [
    {
      "name": "lockNft",
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
      "name": "unlockNft",
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
      "name": "stakeProof",
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
      "name": "unstakeProof",
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
      "name": "claim",
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
      "name": "LockNftInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "UnlockNftInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "StakeProofInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proveTokenAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnstakeProofInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmTokenAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ClaimInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "LockNftOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proveTokenAmount",
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
      "name": "UnlockNftOutputWrapper",
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
      "name": "StakeProofOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmTokenAmount",
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
      "name": "UnstakeProofOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proveTokenAmount",
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
      "name": "ClaimOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardTokenAmount",
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
  ]
};

export const IDL: AdapterNftFinance = {
  "version": "0.1.0",
  "name": "adapter_nft_finance",
  "instructions": [
    {
      "name": "lockNft",
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
      "name": "unlockNft",
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
      "name": "stakeProof",
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
      "name": "unstakeProof",
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
      "name": "claim",
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
      "name": "LockNftInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "UnlockNftInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "StakeProofInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proveTokenAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnstakeProofInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmTokenAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ClaimInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "LockNftOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proveTokenAmount",
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
      "name": "UnlockNftOutputWrapper",
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
      "name": "StakeProofOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "farmTokenAmount",
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
      "name": "UnstakeProofOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proveTokenAmount",
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
      "name": "ClaimOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardTokenAmount",
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
  ]
};
